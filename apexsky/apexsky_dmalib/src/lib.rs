#![feature(thread_sleep_until, try_find)]

pub mod access;
mod mem;

pub use mem::MemConnector;

#[derive(thiserror::Error, Debug)]
pub enum AccessError {
    #[error("AccessErr: open_os {0:?}")]
    Connector(String),
    #[error("AccessErr: error {0:?}")]
    AnyError(#[from] anyhow::Error),
}
