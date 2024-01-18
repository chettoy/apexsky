use super::Skyapex;

#[skyapex_impl]
pub trait AimbotUtils {
    fn aimbot_smooth_x(&mut self, target_ptr: i64, current: f32, delta: f32, smooth: f32) -> f32;
    fn aimbot_smooth_y(&mut self, target_ptr: i64, current: f32, delta: f32, smooth: f32) -> f32;
    fn triggerbot_cross_hair_ready(
        &mut self,
        view_pitch: f32,
        view_yew: f32,
        delta_pitch: f32,
        delta_yew: f32,
        delta_pitch_min: f32,
        delta_pitch_max: f32,
        delta_yew_min: f32,
        delta_yew_max: f32,
        distance: f32,
        zoom_fov: f32,
    ) -> i32;
}
