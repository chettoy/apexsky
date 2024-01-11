use super::Skyapex;

#[skyapex_impl]
pub trait AimbotUtils {
    fn triggerbot_threshold_fov(&mut self, zoom_fov: f32, distance: f32) -> f32;
    fn aimbot_smooth_x(&mut self, target_ptr: i64, current: f32, delta: f32, smooth: f32) -> f32;
    fn aimbot_smooth_y(&mut self, target_ptr: i64, current: f32, delta: f32, smooth: f32) -> f32;
}
