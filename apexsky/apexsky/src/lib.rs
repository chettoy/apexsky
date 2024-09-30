pub mod aimbot;
pub mod config;
pub mod ffi;
pub mod global_state;
pub mod i18n;
pub mod love_players;
pub mod menu;
pub mod offsets;
pub mod system;

use std::path::PathBuf;

pub use ffi::*;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
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

#[macro_export]
macro_rules! noobfstr {
    ($str:expr) => {
        $str
    };
}

pub fn get_runner_home_dir() -> Option<PathBuf> {
    std::env::var(obfstr::obfstr!("SUDO_HOME"))
        .ok()
        .map(PathBuf::from)
        .or(dirs::home_dir())
}

pub fn get_base_dir() -> PathBuf {
    static DATA_DIR: Lazy<PathBuf> = Lazy::new(|| {
        if cfg!(unix)
            && std::fs::exists(obfstr::obfstr!("/usr/share/apexsky/")).is_ok_and(|exists| exists)
        {
            get_runner_home_dir()
                .unwrap()
                .join(".local")
                .join("share")
                .join(obfstr::obfstr!("apexsky"))
        } else {
            std::env::current_dir().unwrap()
        }
    });

    DATA_DIR.to_path_buf()
}
