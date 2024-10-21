#[cfg(all(feature = "ohosky", not(feature = "skydream")))]
use crate::ohosky as skyapi;
#[cfg(feature = "skydream")]
use crate::skydream as skyapi;

pub mod dmalib;
pub mod host;
