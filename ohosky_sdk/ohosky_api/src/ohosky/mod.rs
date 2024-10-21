pub mod api;
pub mod dmalib;
pub mod host;
pub mod msg;
pub mod rpc;
pub mod store;

pub use dmalib::MemAccess;
pub use host::HostApi;
pub use msg::SharedMessageChannel;
pub use rpc::{SharedRpcClient, SharedRpcService};
pub use store::SharedStoreApi;

pub trait ConvertWasmResultToAnyhow<T> {
    fn to_anyhow(self) -> anyhow::Result<T>;
}

impl<T> ConvertWasmResultToAnyhow<T> for Result<T, String> {
    fn to_anyhow(self) -> anyhow::Result<T> {
        self.map_err(|e| anyhow::anyhow!(e))
    }
}

pub trait ConvertAnyhowToWasmResult<T> {
    fn to_wasm_result(self) -> Result<T, String>;
}

impl<T> ConvertAnyhowToWasmResult<T> for anyhow::Result<T> {
    fn to_wasm_result(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}
