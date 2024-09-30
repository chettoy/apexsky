#![feature(iterator_try_collect, thread_sleep_until, try_find)]

pub mod access;
mod mem;

pub use access::ConnectConfig;
pub use mem::MemConnector;

#[derive(thiserror::Error, Debug)]
pub enum AccessError {
    #[error("AccessErr: open_os {0:?} {1:?}")]
    Connector(MemConnector, anyhow::Error),
    #[error("AccessErr: invalid time_date_stamp 0x{0:x} (expect: 0x{1:x})")]
    InvalidTimeDateStamp(u32, u32),
    #[error("AccessErr: error {0:?}")]
    AnyError(#[from] anyhow::Error),
}
