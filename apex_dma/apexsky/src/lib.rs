use global_state::GlobalState;

mod config;
mod global_state;
mod math;
mod menu;
mod pitches;
mod skynade;
mod i18n;

#[macro_use]
extern crate lazy_static;

use global_state::G_STATE;

#[no_mangle]
pub extern "C" fn __get_global_states() -> GlobalState {
    G_STATE.lock().unwrap().to_owned()
}

#[no_mangle]
pub extern "C" fn __update_global_states(state: GlobalState) {
    *G_STATE.lock().unwrap() = state;
}

// config

#[no_mangle]
pub extern "C" fn __load_settings() {
    let settings = crate::config::get_configuration().unwrap_or_else(|e| {
        println!("{}", e);
        println!("Fallback to defalut configuration.");
        crate::config::Config::default()
    });
    G_STATE.lock().unwrap().settings = settings;
}

#[no_mangle]
pub extern "C" fn save_settings() -> bool {
    let settings = G_STATE.lock().unwrap().settings.to_owned();
    crate::config::save_configuration(settings)
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

// misc

#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

#[no_mangle]
pub extern "C" fn print_run_as_root() {
    println!("Please run as root!");
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
