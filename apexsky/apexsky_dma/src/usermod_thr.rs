use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use apexsky_extension::{
    ExtensionMessage, GameApi, InstallManager, RunningManager, RuntimePermission,
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
    InstallPackage(PathBuf, oneshot::Sender<anyhow::Result<()>>),
    GameAttached,
    GameUnattached,
    ActionTick(ActionTickData),
}

pub(crate) struct ActionTickData {
    pub(crate) input_state: [u32; 4],
}

#[derive(Debug, Default, Deserialize)]
struct GuestOptions {
    watch_input: Option<Vec<i32>>,
}

#[derive(Debug, Serialize)]
struct ActionTickPayload {
    input: Option<IndexMap<i32, bool>>,
}

struct RunningCtx {
    guest_options: GuestOptions,
    msg_tx: async_channel::Sender<ExtensionMessage>,
}

pub(crate) async fn usermod_loop(
    game_api: Arc<dyn GameApi>,
    mut event_rx: mpsc::UnboundedReceiver<UserModEvent>,
) -> anyhow::Result<()> {
    let mut install_mgr = InstallManager::default();
    let mut running_mgr = RunningManager::new(game_api);

    let mut running_ctx = HashMap::new();

    // Install auto_sg
    if let Err(e) = run_a_package(
        &mut install_mgr,
        &mut running_mgr,
        &mut running_ctx,
        s!("./auto_sg.spk").into(),
    )
    .await
    {
        tracing::warn!(%e, "{}", s!("auto_sg.spk not installed"));
    }

    // Install example
    if let Err(e) = run_a_package(
        &mut install_mgr,
        &mut running_mgr,
        &mut running_ctx,
        s!("./example.spk").into(),
    )
    .await
    {
        tracing::warn!(%e, "{}", s!("example.spk not installed"));
    }

    // Run message loop
    loop {
        let Some(event) = event_rx.recv().await else {
            break;
        };
        for RunningCtx {
            guest_options,
            msg_tx,
        } in running_ctx.values()
        {
            let mut callback_rx = None;
            let msg = match event {
                UserModEvent::InstallPackage(path, result_tx) => {
                    let r =
                        run_a_package(&mut install_mgr, &mut running_mgr, &mut running_ctx, path)
                            .await;
                    let _ = result_tx.send(r);
                    break;
                }
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

#[tracing::instrument(skip_all, fields(path))]
async fn run_a_package(
    install_mgr: &mut InstallManager,
    running_mgr: &mut RunningManager,
    running_ctx: &mut HashMap<String, RunningCtx>,
    path: PathBuf,
) -> anyhow::Result<()> {
    let package_name = install_mgr.install(path).await?;

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
