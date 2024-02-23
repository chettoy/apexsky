use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use apexsky::__load_settings;
use apexsky::aimbot::AimEntity;
use apexsky::config::Settings;
use apexsky::games::apex::player::GamePlayer;
use apexsky::global_state::G_STATE;
use obfstr::obfstr as s;

use tokio::sync::{mpsc, watch, Mutex};
use tokio::task::{self, JoinHandle};
use tokio::time::sleep;
use tracing::{instrument, Level};
use tracing_appender::non_blocking::NonBlocking;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;

mod context_impl;
mod overlay;
mod workers;

#[derive(Debug, Clone)]
struct TreasureClue {
    item_id: i32,
    custom_item_id: u64,
    position: [f32; 3],
    distance: f32,
    entity_ptr: u64,
}

#[derive(Debug, Default, Clone)]
struct SharedState {
    game_attached: bool,
    aim_target: [f32; 3],
    view_matrix: [f32; 16],
    highlight_injected: bool,
    treasure_clues: Vec<TreasureClue>,
    spectator_count: usize,
    allied_spectator_count: usize,
    map_testing_local_team: i32,
    world_ready: bool,
    frame_count: i32,
    game_fps: f32,
    players: HashMap<u64, GamePlayer>,
    aim_entities: HashMap<u64, Arc<dyn AimEntity>>,
    local_player: Option<GamePlayer>,
}

#[derive(Debug)]
struct State {
    active: bool,
    active_tx: watch::Sender<bool>,
    shared_state: Arc<Mutex<SharedState>>,
    actions_t: Option<JoinHandle<anyhow::Result<()>>>,
    aim_t: Option<JoinHandle<anyhow::Result<()>>>,
    control_t: Option<JoinHandle<anyhow::Result<()>>>,
    esp_t: Option<JoinHandle<anyhow::Result<()>>>,
    items_t: Option<JoinHandle<anyhow::Result<()>>>,
    overlay_task: Option<JoinHandle<()>>,
    terminal_task: Option<JoinHandle<()>>,
}

impl State {
    fn new() -> Self {
        let active = false;
        let (active_tx, _rx) = watch::channel(active);
        Self {
            active,
            active_tx,
            shared_state: Arc::new(Mutex::new(SharedState::default())),
            actions_t: None,
            aim_t: None,
            control_t: None,
            esp_t: None,
            items_t: None,
            overlay_task: None,
            terminal_task: None,
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

        let (aim_key_tx, aim_key_rx) = watch::channel(workers::aim::AimKeyStatus::default());
        let (aim_select_tx, aim_select_rx) = watch::channel(vec![]);
        let (aim_action_tx, aim_action_rx) = mpsc::channel(5);

        self.actions_t = Some(task::spawn(actions_loop(
            self.active_tx.subscribe(),
            self.shared_state.clone(),
            aim_key_tx,
            aim_select_tx,
            aim_action_rx,
        )));
        self.aim_t = Some(task::spawn(aimbot_loop(
            self.active_tx.subscribe(),
            self.shared_state.clone(),
            aim_key_rx,
            aim_select_rx,
            aim_action_tx,
        )));
        self.control_t = Some(task::spawn(control_loop(
            self.active_tx.subscribe(),
            self.shared_state.clone(),
        )));
        self.esp_t = Some(task::spawn(esp_loop(
            self.active_tx.subscribe(),
            self.shared_state.clone(),
        )));
        self.items_t = Some(task::spawn(items_loop(
            self.active_tx.subscribe(),
            self.shared_state.clone(),
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
                    false
                }
            }
        }
        check_task(&mut self.actions_t, s!("actions_t")).await;
        check_task(&mut self.aim_t, s!("aim_t")).await;
        check_task(&mut self.control_t, s!("control_t")).await;
        check_task(&mut self.esp_t, s!("esp_t")).await;
        check_task(&mut self.items_t, s!("items_t")).await;
    }
}

fn main() {
    let (non_blocking, _guard) = tracing_appender::non_blocking(tracing_appender::rolling::daily(
        s!("log"),
        s!("rolling.log"),
    ));
    init_logger(non_blocking, true);

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Unable to create Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    let args: Vec<String> = std::env::args().collect();

    tracing::debug!(?args, "{}", s!("start c9OI8lMNlvrc"));
    __load_settings();

    let mut state = State::new();

    let shared_state = state.shared_state.clone();

    if args.len() == 2 {
        if args[1] == s!("menu") {
            apexsky::menu::main().unwrap();
            return;
        }
        if args[1] == s!("overlay") {
            overlay::main(shared_state).unwrap();
            return;
        }
    }

    // Execute the runtime in its own thread.
    std::thread::spawn(move || {
        rt.block_on(async {
            let g_settings = global_settings();
            let mut debug_mode = g_settings.debug_mode;
            if args.len() == 3 && args[2] == s!("debug") {
                debug_mode = true;
            }

            state.start_tasks().await;

            loop {
                if !state.shared_state.lock().await.game_attached {
                    // stop tasks
                    if let Some(tui_task) = state.terminal_task.take() {
                        quit_tui(tui_task).await;
                    }
                } else {
                    if debug_mode {
                        if let Some(tui_task) = state.terminal_task.take() {
                            quit_tui(tui_task).await;
                        }
                    } else {
                        if state.terminal_task.is_none() {
                            let tui_task = start_tui();
                            state.terminal_task = Some(tui_task);
                        }
                    }
                }
                state.check_tasks().await;
                sleep(Duration::from_millis(10)).await;
            }
        })
    });

    loop {
        if G_STATE.lock().unwrap().config.settings.no_overlay {
            std::thread::sleep(Duration::from_secs(1));
        } else {
            overlay::main(shared_state.clone())
                .unwrap_or_else(|e| tracing::error!(%e, ?e, "{}", s!("overlay::main()")));
        }
    }
}

pub fn global_settings() -> Settings {
    G_STATE.lock().unwrap().config.settings.clone()
}

pub fn press_to_exit() {
    println!("{}", s!("Press enter to exit.."));
    let _ = std::io::stdin().read_line(&mut String::new());
    std::process::exit(0);
}

#[instrument]
pub fn start_tui() -> JoinHandle<()> {
    task::spawn_blocking(|| {
        apexsky::menu::main().unwrap_or_else(|e| tracing::error!(%e, ?e, "{}", s!("menu::main()")))
    })
}

#[instrument]
pub async fn quit_tui(tui_task: JoinHandle<()>) {
    G_STATE.lock().unwrap().terminal_t = false;
    tui_task.await.unwrap_or_else(|e| {
        tracing::error!(%e, ?e);
    });
}

fn init_logger(non_blocking: NonBlocking, print: bool) {
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| {
            EnvFilter::try_new(s!(
                "apexsky_dma=trace,apexsky=trace,apexsky::love_players=info"
            ))
        })
        .unwrap();

    let formatting_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr.with_max_level(Level::INFO))
        .pretty();

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
}
