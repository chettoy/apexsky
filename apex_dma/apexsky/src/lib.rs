#![recursion_limit = "1024"]

#[macro_use]
extern crate lazy_static;

pub mod aimbot;
pub mod apexdream;
pub mod config;
pub mod ffi;
pub mod games;
pub mod global_state;
pub mod i18n;
pub mod love_players;
pub mod mem;
pub mod menu;
pub mod offsets;
pub mod pb;
pub mod system;
pub mod web_map_radar;

pub use ffi::*;

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
