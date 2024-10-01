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

use obfstr::obfstr as s;
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

/// Checks if the application is installed locally on a Unix-like system.
///
/// This function determines whether the application is installed in a standard location typically found on Unix-like systems, such as `/usr/bin` or `/bin`.
/// It uses Rust's built-in platform detection (`cfg!(unix)`) to ensure it only checks for Unix-like systems and then verifies if the executable starts with these paths.
pub fn is_installed_locally() -> bool {
    static INSTALLED_LOCALLY: Lazy<bool> = Lazy::new(|| {
        cfg!(unix)
            && std::env::current_exe()
                .is_ok_and(|exe| exe.starts_with(s!("/usr/bin")) || exe.starts_with(s!("/bin")))
    });
    // Return the value stored in INSTALLED_LOCALLY
    *INSTALLED_LOCALLY
}

/// Retrieves the base directory where data is stored for the application.
///
/// The function determines the base directory based on whether the application is installed locally or not.
/// If the application is installed locally, it uses a common user directory for shared data (e.g., `~/.local/share`).
/// If the application is not installed locally, it uses the current working directory.
/// The function returns the base directory as a PathBuf.
pub fn get_base_dir() -> PathBuf {
    static DATA_DIR: Lazy<PathBuf> = Lazy::new(|| {
        if is_installed_locally() {
            // If the application is installed locally, use the common user directory for shared data
            apexsky_utils::get_runner_home_dir()
                .unwrap()
                .join(".local")
                .join("share")
                .join(s!("apexsky"))
        } else {
            // If the application is not installed locally, use the current directory
            std::env::current_dir().unwrap()
        }
    });

    // Return the base data directory as a PathBuf
    DATA_DIR.to_path_buf()
}
