use std::sync::RwLock;

use super::apexdream::*;
use super::{AimAngles, Aimbot, AimbotSettings, TriggerBot};
use crate::Vec4;

lazy_static! {
    pub static ref G_AIMBOT: RwLock<Aimbot> = RwLock::new(Aimbot::new());
}

macro_rules! aimbot_read {
    () => {
        G_AIMBOT.read().unwrap()
    };
}

macro_rules! aimbot_write {
    () => {
        G_AIMBOT.write().unwrap()
    };
}

#[no_mangle]
pub extern "C" fn skynade_angle(
    weapon_id: u32,
    weapon_mod_bitfield: u32,
    weapon_projectile_scale: f32,
    weapon_projectile_speed: f32,
    local_view_origin_x: f32,
    local_view_origin_y: f32,
    local_view_origin_z: f32,
    target_x: f32,
    target_y: f32,
    target_z: f32,
) -> Vec4 {
    if let Some(tup) = skynade::skynade_angle(
        weapon_id,
        weapon_mod_bitfield,
        weapon_projectile_scale,
        weapon_projectile_speed,
        &[
            local_view_origin_x,
            local_view_origin_y,
            local_view_origin_z,
        ],
        &[target_x, target_y, target_z],
    ) {
        tup.into()
    } else {
        Vec4::default()
    }
}

#[no_mangle]
pub extern "C" fn linear_predict(
    weapon_projectile_grav: f32,
    weapon_projectile_speed: f32,
    local_x: f32,
    local_y: f32,
    local_z: f32,
    target_x: f32,
    target_y: f32,
    target_z: f32,
    vel_x: f32,
    vel_y: f32,
    vel_z: f32,
) -> Vec4 {
    use solver::{solve, LinearPredictor};
    struct Weapon(f32, f32);
    impl solver::ProjectileWeapon for Weapon {
        fn projectile_speed(&self) -> f32 {
            self.0
        }
        fn projectile_gravity(&self) -> f32 {
            self.1
        }
    }

    let pos_origin = [local_x, local_y, local_z];
    let pos_target = [target_x, target_y, target_z];
    let vel = [vel_x, vel_y, vel_z];
    let weapon = Weapon(weapon_projectile_speed, weapon_projectile_grav);

    let predictor = LinearPredictor {
        origin: pos_target,
        velocity: vel,
    };

    if let Some(sol) = solve(&pos_origin, &weapon, &predictor) {
        // let hit = predictor.predict_position(sol.time);
        let pitch = -sol.pitch.to_degrees();
        let yaw = sol.yaw.to_degrees();
        (pitch, yaw).into()
    } else {
        Vec4::default()
    }
}

#[no_mangle]
pub extern "C" fn aimbot_get_state() -> Aimbot {
    aimbot_read!().clone()
}

#[no_mangle]
pub extern "C" fn aimbot_get_settings() -> AimbotSettings {
    aimbot_read!().get_settings()
}

#[no_mangle]
pub extern "C" fn aimbot_settings(settings: &AimbotSettings) {
    aimbot_write!().settings(settings.clone())
}

#[no_mangle]
pub extern "C" fn aimbot_is_aiming() -> bool {
    aimbot_read!().is_aiming()
}

#[no_mangle]
pub extern "C" fn aimbot_is_grenade() -> bool {
    aimbot_read!().is_grenade()
}

#[no_mangle]
pub extern "C" fn aimbot_is_headshot() -> bool {
    aimbot_read!().is_headshot()
}

#[no_mangle]
pub extern "C" fn aimbot_is_semi_auto() -> bool {
    aimbot_read!().is_semi_auto()
}

#[no_mangle]
pub extern "C" fn aimbot_is_locked() -> bool {
    aimbot_read!().is_locked()
}

#[no_mangle]
pub extern "C" fn aimbot_is_triggerbot_ready() -> bool {
    aimbot_read!().is_triggerbot_ready()
}

#[no_mangle]
pub extern "C" fn aimbot_get_max_fov() -> f32 {
    aimbot_read!().get_max_fov()
}

#[no_mangle]
pub extern "C" fn aimbot_get_held_id() -> i32 {
    aimbot_read!().get_held_id()
}

#[no_mangle]
pub extern "C" fn aimbot_update_held_id(held_id: i32) {
    aimbot_write!().update_held_id(held_id)
}

#[no_mangle]
pub extern "C" fn aimbot_get_weapon_id() -> i32 {
    aimbot_read!().get_weapon_id()
}

#[no_mangle]
pub extern "C" fn aimbot_update_weapon_info(
    weapon_id: i32,
    bullet_speed: f32,
    bullet_gravity: f32,
    weapon_zoom_fov: f32,
    weapon_mod_bitfield: i32,
) {
    aimbot_write!().update_weapon_info(
        weapon_id,
        bullet_speed,
        bullet_gravity,
        weapon_zoom_fov,
        weapon_mod_bitfield,
    )
}

#[no_mangle]
pub extern "C" fn aimbot_get_gun_safety() -> bool {
    aimbot_read!().get_gun_safety()
}

#[no_mangle]
pub extern "C" fn aimbot_set_gun_safety(gun_safety: bool) {
    aimbot_write!().set_gun_safety(gun_safety)
}

#[no_mangle]
pub extern "C" fn aimbot_get_aim_key_state() -> i32 {
    aimbot_read!().get_aim_key_state()
}

#[no_mangle]
pub extern "C" fn aimbot_update_aim_key_state(aim_key_state: i32) {
    aimbot_write!().update_aim_key_state(aim_key_state)
}

#[no_mangle]
pub extern "C" fn aimbot_update_triggerbot_key_state(triggerbot_key_state: i32) {
    aimbot_write!().update_triggerbot_key_state(triggerbot_key_state)
}

#[no_mangle]
pub extern "C" fn aimbot_update_attack_state(attack_state: i32) {
    aimbot_write!().update_attack_state(attack_state)
}

#[no_mangle]
pub extern "C" fn aimbot_update_zoom_state(zoom_state: i32) {
    aimbot_write!().update_zoom_state(zoom_state)
}

#[no_mangle]
pub extern "C" fn aimbot_get_aim_entity() -> u64 {
    aimbot_read!().get_aim_entity()
}

#[no_mangle]
pub extern "C" fn aimbot_target_distance_check(distance: f32) -> bool {
    aimbot_read!().target_distance_check(distance)
}

#[no_mangle]
pub extern "C" fn aimbot_start_select_target() {
    aimbot_write!().start_select_target()
}

#[no_mangle]
pub extern "C" fn aimbot_add_select_target(
    fov: f32,
    distance: f32,
    visible: bool,
    love: bool,
    target_ptr: u64,
) {
    aimbot_write!().add_select_target(fov, distance, visible, love, target_ptr)
}

#[no_mangle]
pub extern "C" fn aimbot_finish_select_target() {
    aimbot_write!().finish_select_target()
}

#[no_mangle]
pub extern "C" fn aimbot_lock_target(target_ptr: u64) {
    aimbot_write!().lock_target(target_ptr)
}

#[no_mangle]
pub extern "C" fn aimbot_cancel_locking() {
    aimbot_write!().cancel_locking()
}

#[no_mangle]
pub extern "C" fn aimbot_update(local_entity: u64, game_fps: f32) {
    let mut aimbot = aimbot_write!();
    aimbot.local_entity = local_entity;
    aimbot.game_fps = game_fps;
    aimbot.update();
}

#[no_mangle]
pub extern "C" fn aimbot_smooth_aim_angles(aim_angles: &AimAngles, smooth_factor: f32) -> Vec4 {
    aimbot_read!()
        .smooth_aim_angles(aim_angles, smooth_factor)
        .into()
}

#[no_mangle]
pub extern "C" fn aimbot_poll_trigger_action() -> i32 {
    aimbot_write!().poll_trigger_action()
}

#[no_mangle]
pub extern "C" fn aimbot_triggerbot_update(aim_angles: &AimAngles, force_attack_state: i32) {
    aimbot_write!().triggerbot_update(aim_angles, force_attack_state)
}
