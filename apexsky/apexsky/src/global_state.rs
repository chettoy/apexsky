use anyhow::Context;
use obfstr::obfstr as s;
use once_cell::sync::Lazy;
use skyapex_sdk::Skyapex;
use std::sync::Mutex;

use crate::system::SysContext;

pub static G_STATE: Lazy<Mutex<GlobalState>> = Lazy::new(|| Mutex::new(GlobalState::default()));
pub static G_CONTEXT: Lazy<Mutex<SysContext>> =
    Lazy::new(|| Mutex::new(SysContext::new().unwrap()));
pub static G_MOD: Lazy<Mutex<Skyapex>> = Lazy::new(|| {
    Mutex::new(
        Skyapex::load()
            .context(String::from(s!("Failed to load skyapex mod!")))
            .unwrap(),
    )
});

#[derive(Debug, Clone, Default)]
pub struct GlobalState {
    pub config: crate::config::Config,
    pub terminal_t: bool,
    pub tui_forceupdate: bool,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CGlobalState {
    pub settings: crate::config::Settings,
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
        $crate::global_state::G_STATE.lock().unwrap().config
    };
}

#[macro_export]
macro_rules! lock_mod {
    () => {
        $crate::global_state::G_MOD.lock().unwrap()
    };
}
