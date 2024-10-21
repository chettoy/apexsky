pub use async_ffi;
use async_ffi::FfiFuture;
pub use safer_ffi;
use safer_ffi::derive_ReprC;

use super::FfiResult;

#[derive_ReprC]
#[repr(C)]
pub struct CDmalibAccessTarget {
    pub target_process_name: safer_ffi::String,
    pub override_module_base: u64,
    pub check_time_date_stamp: u32,
    pub speed_test: bool,
    pub cache_phys_addr: bool,
}

#[allow(clippy::type_complexity)]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DmalibApiExports {
    pub mem_open: extern "C" fn(config: CDmalibAccessTarget) -> u32,
    pub mem_close: extern "C" fn(handle: u32),
    pub mem_baseaddr_async: extern "C" fn(
        handle: u32,
        priority: i32,
    ) -> FfiFuture<safer_ffi::option::TaggedOption<u64>>,
    pub mem_baseaddr_blocking:
        extern "C" fn(handle: u32, priority: i32) -> safer_ffi::option::TaggedOption<u64>,
    pub mem_read_async: extern "C" fn(
        handle: u32,
        addr: u64,
        len: usize,
        priority: i32,
        req_id: usize,
    ) -> FfiFuture<safer_ffi::Vec<u8>>,
    pub mem_read_blocking: extern "C" fn(
        handle: u32,
        addr: u64,
        len: usize,
        priority: i32,
        req_id: usize,
    ) -> safer_ffi::Vec<u8>,
    pub mem_read_batch_blocking: extern "C" fn(
        handle: u32,
        batch: safer_ffi::Vec<safer_ffi::Tuple2<u64, safer_ffi::slice::Mut<u8>>>,
        priority: i32,
        req_id: usize,
    ) -> usize,
    pub mem_write_async: extern "C" fn(
        handle: u32,
        addr: u64,
        data: safer_ffi::Vec<u8>,
        priority: i32,
        req_id: usize,
    ) -> FfiFuture<bool>,
    pub mem_write_blocking: extern "C" fn(
        handle: u32,
        addr: u64,
        data: safer_ffi::Vec<u8>,
        priority: i32,
        req_id: usize,
    ) -> bool,
    pub mem_find_sig_async: extern "C" fn(
        handle: u32,
        sig: safer_ffi::String,
        start: u64,
        end: u64,
    ) -> FfiFuture<
        FfiResult<safer_ffi::option::TaggedOption<safer_ffi::Tuple2<u64, safer_ffi::Vec<u8>>>>,
    >,
    pub mem_find_sig_blocking: extern "C" fn(
        handle: u32,
        sig: safer_ffi::String,
        start: u64,
        end: u64,
    ) -> FfiResult<
        safer_ffi::option::TaggedOption<safer_ffi::Tuple2<u64, safer_ffi::Vec<u8>>>,
    >,
    pub mem_dump_async: extern "C" fn(handle: u32) -> FfiFuture<FfiResult<safer_ffi::Vec<u8>>>,
    pub mem_dump_blocking: extern "C" fn(handle: u32) -> FfiResult<safer_ffi::Vec<u8>>,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct HostApiExports {
    pub add: extern "C" fn(a: i32, b: i32) -> i32,
    pub check_file_permission: extern "C" fn(path: safer_ffi::Vec<u8>) -> bool,
    pub get_base_dir: extern "C" fn() -> safer_ffi::String,
    pub get_config_dir: extern "C" fn() -> safer_ffi::String,
    pub get_temp_dir: extern "C" fn() -> safer_ffi::String,
    pub get_locale: extern "C" fn() -> safer_ffi::option::TaggedOption<safer_ffi::String>,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MsgApiExports {
    pub reg_msg: extern "C" fn(name: safer_ffi::String) -> u64,
    pub send_async: extern "C" fn(id: u64, value: safer_ffi::Vec<u8>) -> FfiFuture<bool>,
    pub send_blocking: extern "C" fn(id: u64, value: safer_ffi::Vec<u8>) -> bool,
    pub recv_async:
        extern "C" fn(id: u64) -> FfiFuture<safer_ffi::option::TaggedOption<safer_ffi::Vec<u8>>>,
    pub recv_blocking:
        extern "C" fn(id: u64) -> safer_ffi::option::TaggedOption<safer_ffi::Vec<u8>>,
    pub try_recv: extern "C" fn(
        id: u64,
    ) -> safer_ffi::option::TaggedOption<
        safer_ffi::option::TaggedOption<safer_ffi::Vec<u8>>,
    >,
    pub subscribe: extern "C" fn(
        name: safer_ffi::String,
        unique_watcher: safer_ffi::String,
    ) -> safer_ffi::Tuple2<u64, u64>,
    pub unsubscribe: extern "C" fn(unique_id: u64) -> bool,
    pub update_value: extern "C" fn(sender_id: u64, value: safer_ffi::Vec<u8>) -> bool,
    pub fetch_value: extern "C" fn(watcher_id: u64) -> safer_ffi::Vec<u8>,
    pub next_value_async: extern "C" fn(watcher_id: u64) -> FfiFuture<safer_ffi::Vec<u8>>,
    pub try_next_value:
        extern "C" fn(watcher_id: u64) -> safer_ffi::option::TaggedOption<safer_ffi::Vec<u8>>,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RpcApiExports {
    pub reg_rpc: extern "C" fn(name: safer_ffi::String) -> u64,
    pub use_rpc: extern "C" fn(name: safer_ffi::String) -> u64,
    pub recv_async: extern "C" fn(id: u64) -> FfiFuture<FfiResult<safer_ffi::Vec<u8>>>,
    pub recv_blocking: extern "C" fn(id: u64) -> FfiResult<safer_ffi::Vec<u8>>,
    pub try_recv:
        extern "C" fn(id: u64) -> FfiResult<safer_ffi::option::TaggedOption<safer_ffi::Vec<u8>>>,
    pub reply: extern "C" fn(id: u64, ret_value: safer_ffi::Vec<u8>) -> bool,
    pub is_online: extern "C" fn(id: u64) -> FfiResult<bool>,
    pub wait_online_async: extern "C" fn(id: u64) -> FfiFuture<FfiResult<bool>>,
    pub call_async: extern "C" fn(
        id: u64,
        arg_value: safer_ffi::Vec<u8>,
    ) -> FfiFuture<FfiResult<safer_ffi::Vec<u8>>>,
    pub call_blocking:
        extern "C" fn(id: u64, arg_value: safer_ffi::Vec<u8>) -> FfiResult<safer_ffi::Vec<u8>>,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct StoreApiExports {
    pub set: extern "C" fn(id: u64, value: safer_ffi::Vec<u8>),
    pub get: extern "C" fn(id: u64) -> safer_ffi::option::TaggedOption<safer_ffi::Vec<u8>>,
    pub has: extern "C" fn(id: u64) -> bool,
    pub del: extern "C" fn(id: u64) -> bool,
    pub child_set: extern "C" fn(id: u64, child_id: u64, value: safer_ffi::Vec<u8>),
    pub child_get: extern "C" fn(
        id: u64,
        child_id: u64,
    ) -> safer_ffi::option::TaggedOption<safer_ffi::Vec<u8>>,
    pub child_has: extern "C" fn(id: u64, child_id: u64) -> bool,
    pub child_del: extern "C" fn(id: u64, child_id: u64) -> bool,
    pub insert_children:
        extern "C" fn(id: u64, entries: safer_ffi::Vec<safer_ffi::Tuple2<u64, safer_ffi::Vec<u8>>>),
    pub swap_children:
        extern "C" fn(id: u64, entries: safer_ffi::Vec<safer_ffi::Tuple2<u64, safer_ffi::Vec<u8>>>),
    pub get_children:
        extern "C" fn(id: u64) -> safer_ffi::Vec<safer_ffi::Tuple2<u64, safer_ffi::Vec<u8>>>,
    pub list_children: extern "C" fn(id: u64) -> safer_ffi::Vec<u64>,
    pub clear_children: extern "C" fn(id: u64) -> usize,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SkydreamApi {
    pub dmalib: DmalibApiExports,
    pub host: HostApiExports,
    pub msg: MsgApiExports,
    pub rpc: RpcApiExports,
    pub store: StoreApiExports,
}
