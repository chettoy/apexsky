use std::ffi::CStr;

use global_state::CGlobalState;

mod aimbot;
mod config;
mod global_state;
mod i18n;
mod love_players;
mod math;
mod menu;
mod pitches;
mod skyapex;
mod skynade;
mod solver;
mod system;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate apexsky_derive;

use crate::skyapex::utils::Utils;
use global_state::G_CONTEXT;
use global_state::G_STATE;

// state sync

#[no_mangle]
pub extern "C" fn __get_global_states() -> CGlobalState {
    G_STATE.lock().unwrap().to_owned().into()
}

#[no_mangle]
pub extern "C" fn __update_global_states(state: CGlobalState) {
    let global_state = &mut G_STATE.lock().unwrap();
    global_state.config.settings = state.settings;
    global_state.terminal_t = state.terminal_t;
}

// config

#[no_mangle]
pub extern "C" fn __load_settings() {
    lock_config!() = crate::config::get_configuration().unwrap_or_else(|e| {
        println!("{}", e);
        println!("Fallback to defalut configuration.");
        crate::config::Config::default()
    });
}

#[no_mangle]
pub extern "C" fn save_settings() -> bool {
    crate::config::save_configuration(lock_config!().to_owned())
        .map(|()| true)
        .unwrap_or_else(|e| {
            println!("{}", e);
            false
        })
}

// menu

#[no_mangle]
pub extern "C" fn run_tui_menu() {
    crate::menu::main().unwrap_or_else(|e| {
        println!("{}", e);
    });
}

// love player

#[no_mangle]
pub extern "C" fn check_love_player(puid: u64, euid: u64, name: *const i8) -> bool {
    let c_str = unsafe { CStr::from_ptr(name) };
    let name_str = c_str.to_string_lossy();
    love_players::check_my_heart(&mut lock_config!(), puid, euid, &name_str)
}

// check spec

#[no_mangle]
pub extern "C" fn init_spec_checker(local_player_ptr: u64) {
    use skyapex::spectators::SpecCheck;
    lock_mod!().init_spec_checker(local_player_ptr);
}

#[no_mangle]
pub extern "C" fn tick_yew(target_ptr: u64, yew: f32) {
    use skyapex::spectators::SpecCheck;
    lock_mod!().tick_yew(target_ptr, yew);
}

#[no_mangle]
pub extern "C" fn is_spec(target_ptr: u64) -> bool {
    use skyapex::spectators::SpecCheck;
    lock_mod!().is_spec(target_ptr)
}

// misc

#[no_mangle]
pub extern "C" fn add(left: i32, right: i32) -> i32 {
    lock_mod!().add(left, right)
}

#[no_mangle]
pub extern "C" fn print_run_as_root() {
    lock_mod!().print_run_as_root();
}

#[no_mangle]
pub extern "C" fn kbd_backlight_blink(count: i32) -> bool {
    if count < 1 || count > 10 || !lock_config!().settings.kbd_backlight_control {
        return false;
    }
    (|| -> anyhow::Result<()> {
        G_CONTEXT.lock().unwrap().kbd_blink(count.try_into()?)?;
        Ok(())
    })()
    .is_ok()
}

// skynade

#[repr(C)]
pub struct Vector2D {
    x: f32,
    y: f32,
}

// Conversion functions
impl From<(f32, f32)> for Vector2D {
    fn from(tup: (f32, f32)) -> Vector2D {
        Vector2D { x: tup.0, y: tup.1 }
    }
}

impl From<Vector2D> for (f32, f32) {
    fn from(tup: Vector2D) -> (f32, f32) {
        (tup.x, tup.y)
    }
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
) -> Vector2D {
    skynade::skynade_angle(
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
    )
    .into()
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
) -> Vector2D {
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
        Vector2D { x: pitch, y: yaw }
    } else {
        Vector2D { x: 0.0, y: 0.0 }
    }
}

// Aimbot
pub use aimbot::aimbot_add_select_target;
pub use aimbot::aimbot_cancel_locking;
pub use aimbot::aimbot_finish_select_target;
pub use aimbot::aimbot_get_aim_entity;
pub use aimbot::aimbot_get_aim_key_state;
pub use aimbot::aimbot_get_gun_safety;
pub use aimbot::aimbot_get_held_id;
pub use aimbot::aimbot_get_max_fov;
pub use aimbot::aimbot_get_settings;
pub use aimbot::aimbot_get_weapon_id;
pub use aimbot::aimbot_is_aiming;
pub use aimbot::aimbot_is_grenade;
pub use aimbot::aimbot_is_headshot;
pub use aimbot::aimbot_is_locked;
pub use aimbot::aimbot_lock_target;
pub use aimbot::aimbot_new;
pub use aimbot::aimbot_set_gun_safety;
pub use aimbot::aimbot_settings;
pub use aimbot::aimbot_start_select_target;
pub use aimbot::aimbot_target_distance_check;
pub use aimbot::aimbot_update;
pub use aimbot::aimbot_update_aim_key_state;
pub use aimbot::aimbot_update_attack_state;
pub use aimbot::aimbot_update_held_id;
pub use aimbot::aimbot_update_triggerbot_key_state;
pub use aimbot::aimbot_update_weapon_info;
pub use aimbot::aimbot_update_zoom_state;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_settings() {
        __load_settings();
    }

    #[test]
    fn module_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
