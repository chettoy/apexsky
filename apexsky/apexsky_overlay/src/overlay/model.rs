use bevy::prelude::*;
use bevy_health_bar3d::prelude as hpbar;

use super::system::game_esp::EspServiceAddr;

#[derive(Resource)]
pub(crate) struct TokioRuntime(pub tokio::runtime::Runtime);

impl FromWorld for TokioRuntime {
    #[cfg(feature = "native")]
    fn from_world(_world: &mut World) -> Self {
        Self(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Unable to create Runtime"),
        )
    }
    #[cfg(feature = "web-wasm")]
    fn from_world(_world: &mut World) -> Self {
        Self(
            tokio::runtime::Builder::new_current_thread()
                .build()
                .expect("Unable to create Runtime"),
        )
    }
}

#[derive(Resource, Default)]
pub(crate) struct MyOverlayState {
    pub(crate) user_gesture: bool,
    pub(crate) test_sound: bool,
    pub(crate) override_esp_addr: Option<EspServiceAddr>,
    pub(crate) data_latency: f64,
    pub(crate) black_background: bool,
}

#[derive(Component)]
pub(crate) struct MyCameraMarker;

#[derive(Component, Reflect)]
pub(crate) struct Health {
    pub(crate) max: f32,
    pub(crate) current: f32,
}

impl hpbar::Percentage for Health {
    fn value(&self) -> f32 {
        self.current / self.max
    }
}

#[derive(Component, Reflect)]
pub(crate) struct Mana {
    pub(crate) max: f32,
    pub(crate) current: f32,
}

impl hpbar::Percentage for Mana {
    fn value(&self) -> f32 {
        self.current / self.max
    }
}
