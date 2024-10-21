pub mod api;
pub mod dmalib;
pub mod host;
pub mod msg;
pub mod rpc;
pub mod store;

use std::sync::OnceLock;

pub static SKY_HOST_API: OnceLock<api::SkydreamApi> = OnceLock::new();
pub static SKY_MODULE_ARGS: OnceLock<Vec<String>> = OnceLock::new();
pub static SKY_MODULE_TOKEN: OnceLock<secrecy::SecretString> = OnceLock::new();

macro_rules! sky {
    ($(.$field:ident)+) => {
        ($crate::skydream::SKY_HOST_API.get().unwrap()$(.$field)+)
    };
}
use sky;

pub use dmalib::MemAccess;
pub use host::HostApi;
pub use msg::{SharedMessageChannel, SharedValueWatcher, SharedWatchValue};
pub use rpc::{SharedRpcClient, SharedRpcService};
pub use store::SharedStoreApi;

#[safer_ffi::derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FfiResult<T> {
    inner: safer_ffi::option::TaggedOption<T>,
    err: safer_ffi::option::TaggedOption<safer_ffi::String>,
}

impl<T> From<FfiResult<T>> for anyhow::Result<T> {
    fn from(val: FfiResult<T>) -> Self {
        if let Some(inner) = val.inner.into_rust() {
            Ok(inner)
        } else if let Some(e) = val.err.into_rust() {
            Err(anyhow::anyhow!(e))
        } else {
            Err(anyhow::anyhow!("null"))
        }
    }
}

impl<T> From<anyhow::Result<T>> for FfiResult<T> {
    fn from(val: anyhow::Result<T>) -> Self {
        match val {
            Ok(inner) => FfiResult {
                inner: Some(inner).into(),
                err: None.into(),
            },
            Err(e) => FfiResult {
                inner: None.into(),
                err: Some(e.to_string().into()).into(),
            },
        }
    }
}

impl<T> From<anyhow::Error> for FfiResult<T> {
    fn from(val: anyhow::Error) -> Self {
        FfiResult {
            inner: None.into(),
            err: Some(val.to_string().into()).into(),
        }
    }
}

impl<T> FfiResult<T> {
    pub fn into_anyhow(self) -> anyhow::Result<T> {
        self.into()
    }

    pub fn err_str(err: &str) -> Self {
        Self {
            inner: None.into(),
            err: Some(err.to_string().into()).into(),
        }
    }
}
