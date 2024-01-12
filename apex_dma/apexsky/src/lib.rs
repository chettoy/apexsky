use global_state::CGlobalState;
use serde::{Deserialize, Serialize};

mod aimbot;
mod config;
mod global_state;
mod i18n;
mod love_players;
mod math;
mod menu;
mod offsets;
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
pub use love_players::check_love_player;

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

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

// Conversion functions
impl From<(f32, f32)> for Vec4 {
    fn from(tup: (f32, f32)) -> Vec4 {
        Vec4 {
            x: tup.0,
            y: tup.1,
            z: 0.0,
            w: 1.0,
        }
    }
}

// Aimbot
pub use aimbot::ffi::*;

// OffsetsLoader
pub use offsets::export_offsets;

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
