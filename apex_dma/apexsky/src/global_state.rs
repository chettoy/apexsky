use std::sync::Mutex;

use crate::system::SysContext;

lazy_static! {
    pub static ref G_STATE: Mutex<GlobalState> = Mutex::new(GlobalState::default());
    pub static ref G_CONTEXT: Mutex<SysContext> = Mutex::new(SysContext::new().unwrap());
}

#[derive(Debug, Clone, Default)]
pub struct GlobalState {
    pub config: crate::config::Config,
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
