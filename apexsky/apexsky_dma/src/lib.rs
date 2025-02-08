#![feature(duration_millis_float)]

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU64};
use std::time::Duration;

use crate::global_state::G_STATE;
use apex1_common::aimbot::{AimAngles, AimEntity, Aimbot, HitScanReport};
use apex1_common::config::Settings;
use apex1_common::offsets::CustomOffsets;
use apex1_common::pb::apexlegends::{
    AimKeyState, AimTargetInfo, PlayerState, SpectatorInfo, TreasureClue,
};
use obfstr::obfstr as s;
use ohosky_api::common::dmalib::{DmalibAccessTarget, IMemAccess};
use ohosky_api::common::host::IHostApi;
use ohosky_api::skydream_main;
use once_cell::sync::Lazy;
use parking_lot::{Mutex, RwLock};
use secrecy::ExposeSecret;
use tokio::sync::watch;
use tokio::task::{self, JoinHandle};
use tokio::time::sleep;
use tracing::{Level, instrument};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;

use crate::game::player::GamePlayer;

pub use apex1_common::common::{get_config_file_path, load_settings, save_settings};
pub use apex1_common::{config, lock_config, love_players, noobfstr};
pub use obfstr::obfstr;
pub use ohosky_api::skydream as skyapi;

mod actuator;
mod apexdream;
mod context_impl;
mod game;
mod global_state;
mod i18n;
mod menu;
mod system;
mod workers;

const PRINT_LATENCY: bool = false;

pub(crate) static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(get_config_file_path);
pub(crate) static DATA_DIR: Lazy<PathBuf> = Lazy::new(skyapi::HostApi::get_base_dir);
pub(crate) static LOG_DIR: Lazy<PathBuf> = Lazy::new(|| DATA_DIR.join(s!("log")));

pub(crate) static G_TARGET_GAME_VER_DX11: Lazy<bool> =
    Lazy::new(|| global_settings().game_ver_dx11);
pub(crate) static G_OFFSETS: Lazy<CustomOffsets> =
    Lazy::new(|| apex1_common::offsets::load_offsets(*G_TARGET_GAME_VER_DX11));

#[derive(Debug, Default)]
struct SharedState {
    game_baseaddr: AtomicU64,
    tick_num: AtomicU64,
    tick_duration: AtomicU64,
    actions_duration: AtomicU64,
    aim_target: Mutex<(AimAngles, Option<HitScanReport>, Option<[f32; 3]>)>,
    view_matrix: Mutex<[f32; 16]>,
    highlight_injected: AtomicBool,
    teammates: Mutex<Vec<PlayerState>>,
    spectator_list: Mutex<(Vec<SpectatorInfo>, Vec<SpectatorInfo>)>,
    map_testing_local_team: AtomicI32,
    world_ready: AtomicBool,
    frame_count: AtomicI32,
    game_fps: Mutex<f32>,
    players: RwLock<HashMap<u64, Arc<GamePlayer>>>,
    npcs: RwLock<HashMap<u64, Arc<dyn AimEntity>>>,
    treasure_clues: RwLock<HashMap<u64, TreasureClue>>,
    aim_entities: RwLock<HashMap<u64, Arc<dyn AimEntity>>>,
    local_player_ptr: AtomicU64,
    view_player_ptr: AtomicU64,
    aimbot_state: Mutex<Option<(Aimbot, Duration)>>,
}

pub(crate) type SharedStateType = Arc<SharedState>;

#[derive(Debug)]
struct State {
    active: bool,
    active_tx: watch::Sender<bool>,
    shared_state: SharedStateType,
    actions_t: Option<JoinHandle<anyhow::Result<()>>>,
    aim_t: Option<JoinHandle<anyhow::Result<()>>>,
    control_t: Option<JoinHandle<anyhow::Result<()>>>,
    esp_t: Option<JoinHandle<anyhow::Result<()>>>,
    items_t: Option<JoinHandle<anyhow::Result<()>>>,
    remote_t: Option<JoinHandle<anyhow::Result<()>>>,
    terminal_task: Option<JoinHandle<()>>,
    web_t: Option<JoinHandle<anyhow::Result<()>>>,
}

impl State {
    fn new() -> Self {
        let active = false;
        let (active_tx, _rx) = watch::channel(active);
        Self {
            active,
            active_tx,
            shared_state: Arc::new(SharedState::default()),
            actions_t: None,
            aim_t: None,
            control_t: None,
            esp_t: None,
            items_t: None,
            remote_t: None,
            terminal_task: None,
            web_t: None,
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active_tx.send_if_modified(|old_value| {
            if *old_value != active {
                *old_value = active;
                true
            } else {
                false
            }
        });
        self.active = active;
    }

    async fn toggle_tui_active(&mut self, active: bool) {
        #[allow(clippy::collapsible_else_if)]
        if active {
            if self.terminal_task.is_none() {
                let tui_task = task::spawn_blocking(|| {
                    ohosky_menu::main(
                        menu::MenuLevel::Main.into(),
                        &global_state::G_TUI_QUIT,
                        &global_state::G_TUI_FORCE_UPDATE,
                    )
                    .unwrap_or_else(|e| tracing::error!(%e, ?e, "{}", s!("menu::main()")))
                });
                self.terminal_task = Some(tui_task);
            }
        } else {
            if let Some(tui_task) = self.terminal_task.take() {
                global_state::G_TUI_QUIT.store(true, std::sync::atomic::Ordering::Release);
                tui_task.await.unwrap_or_else(|e| {
                    tracing::error!(%e, ?e);
                });
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TaskChannels {
    pub(crate) aim_key_rx: watch::Receiver<AimKeyState>,
    pub(crate) aim_select_rx: watch::Receiver<Vec<AimTargetInfo>>,
    pub(crate) items_glow_rx: watch::Receiver<Vec<(u64, u8)>>,
    pub(crate) update_time_rx: watch::Receiver<f64>,
}

#[derive(Debug, Clone)]
pub struct GameApiHandle {
    pub(crate) state: SharedStateType,
    pub(crate) channels: TaskChannels,
    pub(crate) access_tx: skyapi::dmalib::MemAccess,
}

trait TaskManager {
    async fn start_tasks(&mut self);
    async fn stop_tasks(&mut self);
    async fn check_tasks(&mut self);
}

impl TaskManager for State {
    async fn start_tasks(&mut self) {
        use workers::{
            actions::actions_loop, aim::aimbot_loop, control::control_loop, esp::esp_loop,
            items::items_loop,
        };

        self.stop_tasks().await;

        self.set_active(true);

        let (aim_key_tx, aim_key_rx) = watch::channel(AimKeyState::default());
        let (aim_select_tx, aim_select_rx) = watch::channel(vec![]);
        let (items_glow_tx, items_glow_rx) = watch::channel(vec![]);
        let (update_time_tx, update_time_rx) = watch::channel(0.0);

        let access_tx = skyapi::dmalib::MemAccess::open(&DmalibAccessTarget {
            target_process_name: if *G_TARGET_GAME_VER_DX11 {
                game::data::GAME_VER_DX11_PROCESS_NAME
                    .expose_secret()
                    .to_string()
            } else {
                game::data::GAME_VER_DX12_PROCESS_NAME
                    .expose_secret()
                    .to_string()
            },
            override_module_base: None,
            check_time_date_stamp: match G_OFFSETS.time_date_stamp {
                0 => None,
                x => Some(x.try_into().inspect_err(|e| tracing::error!(?e)).unwrap()),
            },
            speed_test: true,
            cache_phys_addr: true,
        })
        .inspect_err(|e| tracing::error!(?e))
        .unwrap();

        let game_api = GameApiHandle {
            state: self.shared_state.clone(),
            channels: TaskChannels {
                aim_key_rx: aim_key_rx.clone(),
                aim_select_rx: aim_select_rx.clone(),
                items_glow_rx: items_glow_rx.clone(),
                update_time_rx: update_time_rx.clone(),
            },
            access_tx: access_tx.clone(),
        };

        self.actions_t = Some(task::spawn(actions_loop(
            self.active_tx.subscribe(),
            self.shared_state.clone(),
            access_tx.clone(),
            aim_key_tx,
            aim_select_tx,
            update_time_tx,
            aim_select_rx.clone(),
            items_glow_rx.clone(),
        )));
        self.aim_t = Some(task::spawn(aimbot_loop(
            self.active_tx.subscribe(),
            self.shared_state.clone(),
            access_tx.clone(),
            aim_key_rx.clone(),
            aim_select_rx.clone(),
        )));
        self.control_t = Some(task::spawn(control_loop(
            self.active_tx.subscribe(),
            self.shared_state.clone(),
        )));
        self.esp_t = Some(task::spawn(esp_loop(
            self.active_tx.subscribe(),
            game_api.clone(),
        )));
        self.items_t = Some(task::spawn(items_loop(
            self.active_tx.subscribe(),
            self.shared_state.clone(),
            items_glow_tx,
        )));
        self.remote_t = Some(task::spawn(workers::remote::remote_loop(
            self.active_tx.subscribe(),
            game_api,
        )));
        self.web_t = Some(task::spawn(workers::web::web_loop(
            self.active_tx.subscribe(),
        )));
    }

    async fn stop_tasks(&mut self) {
        self.set_active(false);
        if let Some(handle) = self.actions_t.take() {
            handle.await.ok();
        }
        if let Some(handle) = self.aim_t.take() {
            handle.await.ok();
        }
        if let Some(handle) = self.control_t.take() {
            handle.await.ok();
        }
        if let Some(handle) = self.esp_t.take() {
            handle.await.ok();
        }
        if let Some(handle) = self.items_t.take() {
            handle.await.ok();
        }
        if let Some(handle) = self.remote_t.take() {
            handle.await.ok();
        }
        if let Some(handle) = self.web_t.take() {
            handle.await.ok();
        }
    }

    async fn check_tasks(&mut self) {
        #[instrument]
        async fn check_task(
            handle: &mut Option<JoinHandle<anyhow::Result<()>>>,
            tag: &str,
        ) -> bool {
            if let Some(handle) = handle.as_ref() {
                if !handle.is_finished() {
                    return true;
                }
            } else {
                return false;
            }

            let Some(handle) = handle.take() else {
                return false;
            };

            match handle.await {
                Ok(r) => {
                    if let Err(e) = r {
                        tracing::error!(%e, ?e, "{}", tag);
                        false
                    } else {
                        tracing::debug!("{}{}", tag, s!(" finished"));
                        true
                    }
                }
                Err(e) => {
                    tracing::error!(%e, ?e, "{}", tag);
                    if let Ok(reason) = e.try_into_panic() {
                        tracing::error!(?reason, "{}", tag);
                    }
                    false
                }
            }
        }
        check_task(&mut self.actions_t, s!("actions_t")).await;
        check_task(&mut self.aim_t, s!("aim_t")).await;
        check_task(&mut self.control_t, s!("control_t")).await;
        check_task(&mut self.esp_t, s!("esp_t")).await;
        check_task(&mut self.items_t, s!("items_t")).await;
        check_task(&mut self.remote_t, s!("remote_t")).await;
        check_task(&mut self.web_t, s!("web_t")).await;
    }
}

pub(crate) fn main() {
    let _log_appender_guard = init_logger(true);

    let args: Vec<String> = std::env::args().collect();
    tracing::info!(?args, "{}", s!("start T2dN9alaUDm8"));

    // Create tokio runtime
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Unable to create Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    // Check config path
    skyapi::HostApi::check_file_permission(CONFIG_PATH.as_path());

    // Init global settings
    load_settings();

    // Mod Menu
    if args.last() == Some(&s!("mod-menu").to_string()) {
        ohosky_menu::main(
            menu::MenuLevel::Main.into(),
            &global_state::G_TUI_QUIT,
            &global_state::G_TUI_FORCE_UPDATE,
        )
        .unwrap();
        return;
    }

    // Run tasks

    let mut state = State::new();

    let g_settings = global_settings();
    let debug_mode = g_settings.debug_mode;

    rt.block_on(state.start_tasks());

    rt.block_on(async move {
        loop {
            state
                .toggle_tui_active(if state.shared_state.get_game_baseaddr().is_some() {
                    !debug_mode
                } else {
                    false
                })
                .await;

            state.check_tasks().await;

            sleep(Duration::from_millis(10)).await;
        }
    });
}

pub fn global_settings() -> Settings {
    G_STATE.lock().unwrap().config.settings.clone()
}

pub fn press_to_exit() {
    println!("{}", s!("Press enter to exit.."));
    let _ = std::io::stdin().read_line(&mut String::new());
    std::process::exit(0);
}

fn init_logger(print: bool) -> tracing_appender::non_blocking::WorkerGuard {
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| {
            EnvFilter::try_new(s!(
                "apexsky_dma=warn,apex1_common=warn,apexsky_dma::apexdream=warn,apexsky_dma::actuator=info,apexsky_dma::workers::aim=warn,apexsky_dma::workers::actions=warn,apexsky_dma::workers::esp=warn,apexsky_dma::workers::web=warn,apexsky_dma::workers::items=info"
            ))
        })
        .unwrap();

    let formatting_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr.with_max_level(Level::INFO))
        //.with_span_events(FmtSpan::ACTIVE)
        .pretty();

    let (non_blocking, guard) = tracing_appender::non_blocking(tracing_appender::rolling::daily(
        LOG_DIR.as_path(),
        s!("rolling.log"),
    ));

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking.with_max_level(Level::TRACE))
        .with_ansi(false)
        .pretty();

    // let provider = TracerProvider::builder()
    //     .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
    //     .build();
    // let tracer = provider.tracer(s!("apexsky_dma").to_string());

    // let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    //let console_layer = console_subscriber::spawn();

    let subscriber = tracing_subscriber::Registry::default()
        //.with(console_layer)
        .with(filter_layer)
        .with(file_layer);
    //.with(telemetry)

    if print {
        tracing::subscriber::set_global_default(subscriber.with(formatting_layer))
    } else {
        tracing::subscriber::set_global_default(subscriber)
    }
    .expect(s!("setting default subscriber failed"));

    guard
}

skydream_main!(main);
