use crate::common::host::IHostApi;

use super::skyapi::HostApi;

#[cxx::bridge]
mod ffi {
    struct Host {
        _handle: u32,
    }

    extern "Rust" {
        fn add(self: &Host, lhs: i32, rhs: i32) -> i32;
        fn check_file_permission(self: &Host, path: &str) -> bool;
        fn get_base_dir(self: &Host) -> String;
        fn get_config_dir(self: &Host) -> String;
        fn get_temp_dir(self: &Host) -> String;
        fn get_locale(self: &Host) -> String;
    }
}

impl ffi::Host {
    fn add(&self, lhs: i32, rhs: i32) -> i32 {
        HostApi::add(lhs, rhs)
    }

    fn check_file_permission(&self, path: &str) -> bool {
        HostApi::check_file_permission(path.as_ref())
    }

    fn get_base_dir(&self) -> String {
        HostApi::get_base_dir().to_str().unwrap().to_string()
    }

    fn get_config_dir(&self) -> String {
        HostApi::get_config_dir().to_str().unwrap().to_string()
    }

    fn get_temp_dir(&self) -> String {
        HostApi::get_temp_dir().to_str().unwrap().to_string()
    }

    fn get_locale(&self) -> String {
        HostApi::get_locale().unwrap_or_default()
    }
}
