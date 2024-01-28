use anyhow::Context;
use obfstr::obfstr as s;
use skyapex_sdk::Skyapex;
use std::sync::Mutex;

use crate::system::SysContext;

lazy_static! {
    pub static ref G_STATE: Mutex<GlobalState> = Mutex::new(GlobalState::default());
    pub static ref G_CONTEXT: Mutex<SysContext> = Mutex::new(SysContext::new().unwrap());
    pub static ref G_MOD: Mutex<Skyapex> = Mutex::new(
        Skyapex::load()
            .context(String::from(s!("Failed to load skyapex mod!")))
            .unwrap(),
    );
}

#[derive(Debug, Clone, Default)]
pub struct GlobalState {
    pub config: crate::config::Config,
    pub terminal_t: bool,
    pub tui_forceupdate: bool,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CGlobalState {
    pub(crate) settings: crate::config::Settings,
    pub terminal_t: bool,
    pub tui_forceupdate: bool,
}

impl From<GlobalState> for CGlobalState {
    fn from(value: GlobalState) -> Self {
        CGlobalState {
            settings: value.config.settings,
            terminal_t: value.terminal_t,
            tui_forceupdate: value.tui_forceupdate,
        }
    }
}

impl GlobalState {
    pub fn update(&mut self, c_state: CGlobalState) {
        self.config.settings = c_state.settings;
        self.terminal_t = c_state.terminal_t;
        self.tui_forceupdate = c_state.tui_forceupdate;
    }
}

#[macro_export]
macro_rules! lock_config {
    () => {
        crate::global_state::G_STATE.lock().unwrap().config
    };
}

#[macro_export]
macro_rules! lock_mod {
    () => {
        crate::global_state::G_MOD.lock().unwrap()
    };
}
