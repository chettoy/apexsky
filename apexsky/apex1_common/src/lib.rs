#![feature(iterator_try_collect)]

pub mod aimbot;
#[cfg(any(feature = "skydream", feature = "ohosky", feature = "native"))]
pub mod common;
pub mod config;
pub mod data;
pub mod global;
#[cfg(any(feature = "skydream", feature = "ohosky", feature = "native"))]
pub mod global_state;
pub mod love_players;
pub mod offsets;
pub mod pb;
pub mod utils;

#[cfg(all(feature = "ohosky", not(feature = "skydream")))]
pub use ohosky_api::ohosky as skyapi;
#[cfg(feature = "skydream")]
pub use ohosky_api::skydream as skyapi;

#[macro_export]
macro_rules! noobfstr {
    ($s:expr) => {
        $s
    };
}
