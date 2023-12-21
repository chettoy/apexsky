use std::sync::Mutex;

lazy_static! {
    pub static ref G_STATE: Mutex<GlobalState> = Mutex::new(GlobalState::default());
}

#[derive(Debug, Clone, Default)]
pub struct GlobalState {
    pub(crate) config: crate::config::Config,
    pub terminal_t: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct CGlobalState {
    pub(crate) settings: crate::config::Settings,
    pub terminal_t: bool,
}

#[macro_export]
macro_rules! lock_config {
    () => {
        crate::global_state::G_STATE.lock().unwrap().config
    };
}

impl From<GlobalState> for CGlobalState {
    fn from(value: GlobalState) -> Self {
        CGlobalState {
            settings: value.config.settings,
            terminal_t: value.terminal_t,
        }
    }
}
