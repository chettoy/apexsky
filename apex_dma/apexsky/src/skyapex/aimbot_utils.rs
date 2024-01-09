use super::Skyapex;

#[skyapex_impl]
pub trait AimbotUtils {
    fn triggerbot_threshold_fov(&mut self, zoom_fov: f32, distance: f32) -> f32;
}
