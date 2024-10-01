use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU64};
use std::sync::Arc;
use std::time::Duration;

use apexsky::__load_settings;
use apexsky::aimbot::{AimAngles, AimEntity, Aimbot, HitScanReport};
use apexsky::config::Settings;
use apexsky::global_state::G_STATE;
use apexsky_dmalib::MemConnector;
use apexsky_proto::pb::apexlegends::{
    AimKeyState, AimTargetInfo, PlayerState, SpectatorInfo, TreasureClue,
};
use clap::Parser;
use obfstr::obfstr as s;
use once_cell::sync::Lazy;
use parking_lot::{Mutex, RwLock};
use tokio::sync::{mpsc, watch, OnceCell};
use tokio::task::{self, JoinHandle};
use tokio::time::sleep;
use tracing::{instrument, Level};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;
use usermod_thr::UserModEvent;

use crate::game::player::GamePlayer;

pub use apexsky::noobfstr;

mod actuator;
mod apexdream;
mod api_impl;
mod cli;
mod context_impl;
mod game;
mod menu;
mod usermod_thr;
mod workers;

const PRINT_LATENCY: bool = false;

pub(crate) static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(apexsky::config::get_config_file_path);
pub(crate) static DATA_DIR: Lazy<PathBuf> = Lazy::new(apexsky::get_base_dir);
pub(crate) static LOG_DIR: Lazy<PathBuf> = Lazy::new(|| DATA_DIR.join(s!("log")));
pub(crate) static MODS_DIR: Lazy<PathBuf> = Lazy::new(|| DATA_DIR.join(s!("mods")));

pub(crate) static ACCESS_TX: OnceCell<apexsky_dmalib::access::MemApi> = OnceCell::const_new();
pub(crate) static USERMOD_TX: OnceCell<mpsc::UnboundedSender<UserModEvent>> = OnceCell::const_new();

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

pub(crate) type SharedStateWrapper = Arc<SharedState>;

#[derive(Debug)]
struct State {
    active: bool,
    active_tx: watch::Sender<bool>,
    shared_state: SharedStateWrapper,
    io_thread: Option<std::thread::JoinHandle<anyhow::Result<()>>>,
    actions_t: Option<JoinHandle<anyhow::Result<()>>>,
    aim_t: Option<JoinHandle<anyhow::Result<()>>>,
    control_t: Option<JoinHandle<anyhow::Result<()>>>,
    esp_t: Option<JoinHandle<anyhow::Result<()>>>,
    items_t: Option<JoinHandle<anyhow::Result<()>>>,
    terminal_task: Option<JoinHandle<()>>,
    usermod_t: Option<JoinHandle<anyhow::Result<()>>>,
}

impl State {
    fn new() -> Self {
        let active = false;
        let (active_tx, _rx) = watch::channel(active);
        Self {
            active,
            active_tx,
            shared_state: Arc::new(SharedState::default()),
            io_thread: None,
            actions_t: None,
            aim_t: None,
            control_t: None,
            esp_t: None,
            items_t: None,
            terminal_task: None,
            usermod_t: None,
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
        if active {
            if self.terminal_task.is_none() {
                let tui_task = task::spawn_blocking(|| {
                    apexsky::menu::main(menu::CustomMenuLevel::ApexskyMenu.into())
                        .unwrap_or_else(|e| tracing::error!(%e, ?e, "{}", s!("menu::main()")))
                });
                self.terminal_task = Some(tui_task);
            }
        } else {
            if let Some(tui_task) = self.terminal_task.take() {
                G_STATE.lock().unwrap().terminal_t = false;
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

        let (access_tx, access_rx) = apexsky_dmalib::access::create_api();
        let (aim_key_tx, aim_key_rx) = watch::channel(AimKeyState::default());
        let (aim_select_tx, aim_select_rx) = watch::channel(vec![]);
        let (items_glow_tx, items_glow_rx) = watch::channel(vec![]);
        let (update_time_tx, update_time_rx) = watch::channel(0.0);
        let (usermod_tx, usermod_rx) = mpsc::unbounded_channel();

        if let Err(e) = ACCESS_TX.set(access_tx.clone()) {
            tracing::error!(%e, ?e);
        }
        if let Err(e) = USERMOD_TX.set(usermod_tx.clone()) {
            tracing::error!(%e, ?e);
        }

        let game_api = api_impl::GameApiHandle {
            state: self.shared_state.clone(),
            channels: TaskChannels {
                aim_key_rx: aim_key_rx.clone(),
                aim_select_rx: aim_select_rx.clone(),
                items_glow_rx: items_glow_rx.clone(),
                update_time_rx: update_time_rx.clone(),
            },
            access_tx: access_tx.clone(),
        };

        self.io_thread = Some({
            let active_rx = self.active_tx.subscribe();
            std::thread::spawn(move || {
                use apexsky_dmalib::access::{io_thread, ConnectConfig};
                use apexsky_dmalib::AccessError;
                match io_thread(
                    active_rx,
                    access_rx,
                    ConnectConfig {
                        mem_connector: CMD_OPTIONS.connector.clone(),
                        target_proc_name: CMD_OPTIONS.target_process_name.clone(),
                        specify_module_base: CMD_OPTIONS.specify_module_base,
                        check_time_date_stamp: (!CMD_OPTIONS.force_bypass_check).then_some(
                            apexsky::offsets::G_OFFSETS
                                .time_date_stamp
                                .try_into()
                                .unwrap(),
                        ),
                        speed_test: true,
                    },
                ) {
                    Ok(_) => Ok(()),
                    Err(e) => match e {
                        AccessError::Connector(connector, e) => {
                            tracing::error!(?connector, ?e, %e);
                            press_to_exit();
                            Ok(())
                        }
                        AccessError::InvalidTimeDateStamp(_got, _except) => {
                            tracing::error!(%e);
                            press_to_exit();
                            Ok(())
                        }
                        AccessError::AnyError(e) => Err(e),
                    },
                }
            })
        });
        self.usermod_t = Some(task::spawn(usermod_thr::usermod_loop(
            Arc::new(game_api.clone()),
            usermod_rx,
        )));
        self.actions_t = Some(task::spawn(actions_loop(
            self.active_tx.subscribe(),
            self.shared_state.clone(),
            access_tx.clone(),
            aim_key_tx,
            aim_select_tx,
            update_time_tx,
            usermod_tx.clone(),
            aim_select_rx.clone(),
            items_glow_rx.clone(),
        )));
        self.aim_t = Some(task::spawn(aimbot_loop(
            self.active_tx.subscribe(),
            self.shared_state.clone(),
            access_tx.clone(),
            usermod_tx.clone(),
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
        if let Some(handle) = self.usermod_t.take() {
            handle.await.ok();
        }
        if let Some(handle) = self.io_thread.take() {
            handle.join().ok();
        }
    }

    async fn check_tasks(&mut self) {
        fn check_thread(
            handle: &mut Option<std::thread::JoinHandle<anyhow::Result<()>>>,
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

            match handle.join() {
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
                    tracing::error!(?e, "{}", tag);
                    false
                }
            }
        }

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
        check_thread(&mut self.io_thread, s!("io_thread"));
        check_task(&mut self.usermod_t, s!("usermod_thread")).await;
        check_task(&mut self.actions_t, s!("actions_t")).await;
        check_task(&mut self.aim_t, s!("aim_t")).await;
        check_task(&mut self.control_t, s!("control_t")).await;
        check_task(&mut self.esp_t, s!("esp_t")).await;
        check_task(&mut self.items_t, s!("items_t")).await;
    }
}

#[derive(Debug, Clone)]
struct CmdOptions {
    menu_mode: bool,
    debug: bool,
    connector: MemConnector,
    target_process_name: String,
    specify_module_base: Option<u64>,
    force_bypass_check: bool,
}

impl CmdOptions {
    #[instrument]
    fn parse() -> Self {
        use cli::{Cli, Commands};
        let cli = Cli::parse();
        let mut options = Self {
            menu_mode: false,
            debug: cli.debug,
            connector: MemConnector::PCILeech(s!("fpga").to_string()),
            target_process_name: cli
                .proc_name
                .unwrap_or(game::data::TARGET_PROCESS_NAME.to_owned()),
            specify_module_base: cli.module_base,
            force_bypass_check: cli.force_bypass_check,
        };

        if let Some(subcmd) = cli.command {
            match &subcmd {
                Commands::Menu => {
                    options.menu_mode = true;
                }
                Commands::Kvm => options.connector = MemConnector::MemflowKvm,
                Commands::Native => {
                    options.connector = MemConnector::MemflowNative;
                    options
                        .specify_module_base
                        .get_or_insert(game::data::OFFSET_MODULE_BASE);
                }
                Commands::Fpga => {
                    options.connector = MemConnector::PCILeech(s!("fpga").to_string())
                }
                Commands::Leechcore { device } => {
                    options.connector = MemConnector::PCILeech(device.to_owned())
                }
            }
        }

        options
    }
}

static CMD_OPTIONS: Lazy<CmdOptions> = Lazy::new(CmdOptions::parse);

fn main() {
    // This line initializes the CMD_OPTIONS static variable, which is lazily loaded
    // to ensure CLI argument parsing occurs at program start-up,
    // facilitating --help and other arguments to be parsed and leading to an orderly termination for documentation purposes.
    let _ = *CMD_OPTIONS;

    let _log_appender_guard = init_logger(true);

    #[cfg(unix)]
    chown_files_to_runner(
        [
            CONFIG_PATH.as_path(),
            CONFIG_PATH.parent().unwrap(),
            DATA_DIR.as_path(),
            LOG_DIR.as_path(),
            MODS_DIR.as_path(),
        ]
        .into_iter(),
    );

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Unable to create Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    let args: Vec<String> = std::env::args().collect();
    tracing::debug!(?args, "{}", s!("start PJbGRfJ0aZpx"));
    __load_settings();

    let mut state = State::new();

    if CMD_OPTIONS.menu_mode {
        apexsky::menu::main(menu::CustomMenuLevel::ApexskyMenu.into()).unwrap();
        return;
    }

    let g_settings = global_settings();

    let debug_mode = g_settings.debug_mode || CMD_OPTIONS.debug;

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
                "apexsky_dma=warn,apexsky=warn,apexsky_dmalib=info,apexsky_extension=info,apexsky_dma::actuator=info,apexsky_dma::usermod_thr=warn,apexsky_dma::workers::aim=warn,apexsky_dma::workers::actions=warn,apexsky_dma::workers::esp=warn,apexsky_dma::workers::items=info,apexsky_dma::apexdream=warn"
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

#[cfg(unix)]
fn chown_files_to_runner<'a, I>(paths: I)
where
    I: Iterator<Item = &'a std::path::Path>,
{
    use nix::unistd::{Gid, Uid};
    use std::fs;
    use std::os::unix::fs::{chown, MetadataExt};
    use uzers::{get_current_uid, get_user_by_uid};

    let current_uid = get_current_uid();

    if current_uid != 0 {
        return;
    }

    let original_user_uid = match std::env::var(s!("SUDO_UID")) {
        Ok(var) => Uid::from_raw(var.parse::<u32>().expect(s!("Invalid SUDO_UID"))),
        Err(_) => return,
    };
    let original_user_gid = Gid::from_raw(
        std::env::var(s!("SUDO_GID"))
            .expect(s!("Faild to read SUDO_GID"))
            .parse::<u32>()
            .expect(s!("Invalid SUDO_GID")),
    );

    let original_user =
        get_user_by_uid(original_user_uid.into()).expect(s!("Failed to get original user"));

    let original_home = apexsky_utils::get_runner_home_dir().unwrap();
    for path in paths {
        if !path.starts_with(&original_home) {
            continue;
        }
        if !path.exists() {
            continue;
        }
        let metadata =
            fs::metadata(path).expect(&format!("{}{:?}", s!("Failed to get metadata of "), path));
        let current_file_uid = Uid::from_raw(metadata.uid());
        let current_file_gid = Gid::from_raw(metadata.gid());

        if current_file_uid != original_user_uid || current_file_gid != original_user_gid {
            tracing::warn!(
                ?path,
                "{}{:?}",
                s!("Changing ownership to user: "),
                original_user.name()
            );
            chown(
                path,
                Some(original_user_uid.into()),
                Some(original_user_gid.into()),
            )
            .expect(&format!(
                "{}{:?}",
                s!("Failed to change ownership of "),
                path
            ));
        }
    }
}
