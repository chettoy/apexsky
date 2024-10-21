use once_cell::sync::Lazy;
use std::sync::{atomic::AtomicBool, Mutex};

use crate::system::SysContext;

pub use apex1_common::global_state::*;

pub static G_CONTEXT: Lazy<Mutex<SysContext>> =
    Lazy::new(|| Mutex::new(SysContext::new().unwrap()));

pub static G_TUI_QUIT: AtomicBool = AtomicBool::new(false);
pub static G_TUI_FORCE_UPDATE: AtomicBool = AtomicBool::new(false);
