use std::sync::{LazyLock, Mutex};

pub static G_STATE: LazyLock<Mutex<GlobalState>> =
    LazyLock::new(|| Mutex::new(GlobalState::default()));

#[derive(Debug, Clone, Default)]
pub struct GlobalState {
    pub config: crate::config::Config,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CGlobalState {
    pub settings: crate::config::Settings,
}

impl From<GlobalState> for CGlobalState {
    fn from(value: GlobalState) -> Self {
        CGlobalState {
            settings: value.config.settings,
        }
    }
}

impl GlobalState {
    pub fn update(&mut self, c_state: CGlobalState) {
        self.config.settings = c_state.settings;
    }
}

#[macro_export]
macro_rules! lock_config {
    () => {
        $crate::global_state::G_STATE.lock().unwrap().config
    };
}
