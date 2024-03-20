use crate::aimbot;
use crate::global_state;
use crate::lock_config;
use crate::lock_mod;
use crate::love_players;
use crate::offsets;
use obfstr::obfstr as s;
use once_cell::sync::Lazy;
use skyapex_sdk::module::Utils;

use global_state::CGlobalState;
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
    global_state.update(state);
}

// config

#[no_mangle]
pub extern "C" fn __load_settings() {
    lock_config!() = crate::config::get_configuration().unwrap_or_else(|e| {
        static S_MSG: Lazy<String> =
            Lazy::new(|| s!("Fallback to defalut configuration.").to_string());
        tracing::warn!(%e, "{}", &*S_MSG);
        println!("{}", &*S_MSG);
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
    use skyapex_sdk::module::SpecCheck;
    lock_mod!().init_spec_checker(local_player_ptr);
}

#[no_mangle]
pub extern "C" fn tick_yaw(target_ptr: u64, yew: f32) {
    use skyapex_sdk::module::SpecCheck;
    lock_mod!().tick_yaw(target_ptr, yew);
}

#[no_mangle]
pub extern "C" fn is_spec(target_ptr: u64) -> bool {
    use skyapex_sdk::module::SpecCheck;
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

// Aimbot
pub use aimbot::ffi::*;

// OffsetsLoader
pub use offsets::import_offsets;

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
