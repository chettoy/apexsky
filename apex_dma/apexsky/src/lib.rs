use std::ffi::CStr;

use global_state::CGlobalState;

mod config;
mod global_state;
mod i18n;
mod love_players;
mod math;
mod menu;
mod pitches;
mod skynade;
mod system;

#[macro_use]
extern crate lazy_static;

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

// misc

#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

#[no_mangle]
pub extern "C" fn print_run_as_root() {
    println!("Please run as root!");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
