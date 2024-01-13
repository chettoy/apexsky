use std::sync::Mutex;

use anyhow::Context;

use crate::{skyapex::Skyapex, system::SysContext};

lazy_static! {
    pub static ref G_STATE: Mutex<GlobalState> = Mutex::new(GlobalState::default());
    pub static ref G_CONTEXT: Mutex<SysContext> = Mutex::new(SysContext::new().unwrap());
    pub static ref G_MOD: Mutex<Skyapex> = Mutex::new(
        Skyapex::load()
            .context("Failed to load skyapex mod!")
            .unwrap()
    );
}

#[derive(Debug, Clone, Default)]
pub struct GlobalState {
    pub config: crate::config::Config,
    pub terminal_t: bool,
    pub tui_forceupdate: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct CGlobalState {
    pub(crate) settings: crate::config::Settings,
    pub terminal_t: bool,
    pub tui_forceupdate: bool,
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

impl From<GlobalState> for CGlobalState {
    fn from(value: GlobalState) -> Self {
        CGlobalState {
            settings: value.config.settings,
            terminal_t: value.terminal_t,
            tui_forceupdate: value.tui_forceupdate,
        }
    }
}
