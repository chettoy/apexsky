use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use apexsky::{
    aimbot::{AimAngles, Aimbot},
    global_state::G_STATE,
};
use apexsky_extension::{
    ExtensionMessage, GameApi, PackageManager, RunningManager, RuntimePermission,
};
use bitset_core::BitSet;
use indexmap::IndexMap;
use obfstr::obfstr as s;
use serde::{Deserialize, Serialize};
use tokio::{
    sync::{mpsc, oneshot},
    time::Instant,
};

pub(crate) enum UserModEvent {
    HotLoadPackage(
        PathBuf,
        Option<String>,
        Option<oneshot::Sender<anyhow::Result<()>>>,
    ),
    KillPackage(String, Option<oneshot::Sender<anyhow::Result<()>>>),
    GameAttached,
    GameUnattached,
    ActionTick(ActionTickData),
    AimbotTick(Aimbot, AimAngles),
}

pub(crate) struct ActionTickData {
    pub(crate) input_state: [u32; 4],
}

#[derive(Debug, Default, Deserialize)]
struct GuestOptions {
    watch_aimbot: Option<bool>,
    watch_input: Option<Vec<i32>>,
}

#[derive(Debug, Default, Serialize)]
struct ActionTickPayload {
    input: Option<IndexMap<i32, bool>>,
}

#[derive(Debug, Default, Serialize)]
struct AimbotTickPayload {
    aimbot: Option<Aimbot>,
    aim_result: Option<AimAngles>,
}

struct RunningCtx {
    guest_options: GuestOptions,
    msg_tx: async_channel::Sender<ExtensionMessage>,
}

pub(crate) async fn usermod_loop(
    game_api: Arc<dyn GameApi>,
    mut event_rx: mpsc::UnboundedReceiver<UserModEvent>,
) -> anyhow::Result<()> {
    let mut install_mgr = PackageManager::default();
    let mut running_mgr = RunningManager::new(game_api);

    let mut running_ctx = HashMap::new();

    // Load and install packages
    if let Err(e) = install_packages(&mut install_mgr).await {
        tracing::warn!(%e);
    }

    // Run installed packages
    {
        let installed_package_names: Vec<String> =
            install_mgr.get_all_installed().keys().cloned().collect();
        for package_name in installed_package_names {
            if let Err(e) = run_a_package(
                &mut install_mgr,
                &mut running_mgr,
                &mut running_ctx,
                package_name.to_owned(),
            )
            .await
            {
                tracing::warn!(%e, "{}{}", package_name, s!(" not running"));
            }
        }
    }

    // Run message loop
    loop {
        let Some(event) = event_rx.recv().await else {
            break;
        };

        match event {
            UserModEvent::HotLoadPackage(path, checksum, result_tx) => {
                tracing::info!("{}{}", s!("hot load "), path.to_string_lossy());
                match install_mgr.install(path, checksum).await {
                    Ok(package_name) => {
                        let r = run_a_package(
                            &mut install_mgr,
                            &mut running_mgr,
                            &mut running_ctx,
                            package_name.clone(),
                        )
                        .await;
                        if r.is_ok() {
                            tracing::info!("{}{}", s!("hot loaded "), package_name);
                        }
                        if let Some(result_tx) = result_tx {
                            let _ = result_tx.send(r);
                        }
                    }
                    Err(e) => {
                        if let Some(result_tx) = result_tx {
                            let _ = result_tx.send(Err(e.into()));
                        }
                    }
                }
                continue;
            }
            UserModEvent::KillPackage(package_name, result_tx) => {
                running_ctx.remove(&package_name);
                let ret = running_mgr.stop(&package_name).await;
                if let Some(result_tx) = result_tx {
                    let _ = result_tx.send(ret);
                }
                continue;
            }
            _ => (),
        }

        for RunningCtx {
            guest_options,
            msg_tx,
        } in running_ctx.values()
        {
            let mut callback_rx = None;
            let msg = match event {
                UserModEvent::GameAttached => {
                    ExtensionMessage::new(String::from("game_attached"), None)
                }
                UserModEvent::GameUnattached => {
                    ExtensionMessage::new(String::from("game_unattached"), None)
                }
                UserModEvent::ActionTick(ref data) => {
                    let (tx, rx) = oneshot::channel();
                    callback_rx = Some(rx);
                    ExtensionMessage::new_with_data(
                        String::from("action_tick"),
                        Some(generate_tick_payload(data, guest_options)),
                        Some(tx),
                    )?
                }
                UserModEvent::AimbotTick(ref aimbot, ref aim_result) => {
                    let (tx, rx) = oneshot::channel();
                    callback_rx = Some(rx);
                    let data = if guest_options.watch_aimbot == Some(true) {
                        AimbotTickPayload {
                            aimbot: Some(aimbot.clone()),
                            aim_result: Some(aim_result.clone()),
                        }
                    } else {
                        AimbotTickPayload::default()
                    };
                    ExtensionMessage::new_with_data(
                        String::from("aimbot_tick"),
                        Some(data),
                        Some(tx),
                    )?
                }
                _ => break,
            };
            msg_tx.send(msg).await?;
            if let Some(rx) = callback_rx {
                let wait_start = Instant::now();
                let result = rx.await?;
                let wait_time = wait_start.elapsed();
                tracing::trace!("wait={}ms, return={:?}", wait_time.as_millis(), result);
            }
        }
    }

    tracing::warn!("{}", s!("usermod_task finished"));

    Ok(())
}

async fn install_packages(install_mgr: &mut PackageManager) -> anyhow::Result<()> {
    let install_list = G_STATE.lock().unwrap().config.dlc.install.clone();
    for entry in std::fs::read_dir(crate::MODS_DIR.as_path())? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }
        if let Some(file_ext) = path.extension() {
            if file_ext != "spk" {
                continue;
            }
        } else {
            continue;
        }

        let Some(package_name) = install_mgr.install(path, None).await.ok() else {
            continue;
        };
        let Some(installed) = install_mgr.get_installed(&package_name) else {
            continue;
        };

        let Some(install_item) = install_list.get(&package_name) else {
            install_mgr.remove(&package_name);
            continue;
        };
        if installed.checksum != install_item.checksum {
            install_mgr.remove(&package_name);
            tracing::warn!(?install_item, "{}{}", package_name, s!(" not installed"));
            continue;
        }
    }
    Ok(())
}

#[tracing::instrument(skip_all, fields(path))]
async fn run_a_package(
    install_mgr: &mut PackageManager,
    running_mgr: &mut RunningManager,
    running_ctx: &mut HashMap<String, RunningCtx>,
    package_name: String,
) -> anyhow::Result<()> {
    if running_mgr.get_running().contains_key(&package_name) {
        anyhow::bail!("{}", s!("Already running"));
    }

    let Some(installed) = install_mgr.get_installed(&package_name).cloned() else {
        anyhow::bail!("{}", s!("Invalid package"));
    };

    let permissions: RuntimePermission = installed.manifest.get_permissions().into();

    running_mgr.start(installed).await?;

    let Some(running) = running_mgr.get_running().get(&package_name) else {
        anyhow::bail!("{}", s!("Failed to get running state"));
    };
    running_ctx.insert(
        package_name,
        RunningCtx {
            guest_options: running
                .return_value
                .clone()
                .map(|v| {
                    let mut options: GuestOptions = serde_json::from_value(v).unwrap_or_default();
                    if options.watch_aimbot == Some(true) && !permissions.game_world_access {
                        options.watch_aimbot = None;
                        tracing::warn!("{}", s!("No permission to watch aimbot"));
                    }
                    if options.watch_input.is_some() && !permissions.game_input_access {
                        options.watch_input = None;
                        tracing::warn!("{}", s!("No permission to watch game input"));
                    }
                    options
                })
                .unwrap_or_default(),
            msg_tx: running.msg_tx.clone(),
        },
    );

    Ok(())
}

fn generate_tick_payload(data: &ActionTickData, guest_options: &GuestOptions) -> ActionTickPayload {
    ActionTickPayload {
        input: guest_options.watch_input.as_ref().and_then(|list| {
            let button_state = &data.input_state;
            let result = list
                .iter()
                .map(|&keycode| {
                    (
                        keycode,
                        if keycode as usize >= button_state.bit_len() {
                            false
                        } else {
                            button_state.bit_test(keycode as usize)
                        },
                    )
                })
                .collect();
            Some(result)
        }),
    }
}
