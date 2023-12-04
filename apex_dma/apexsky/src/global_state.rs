use std::sync::Mutex;

lazy_static! {
    pub static ref G_STATE: Mutex<GlobalState> = Mutex::new(GlobalState::default());
}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct GlobalState {
    pub(crate) settings: crate::config::Config,
    pub terminal_t: bool,
}