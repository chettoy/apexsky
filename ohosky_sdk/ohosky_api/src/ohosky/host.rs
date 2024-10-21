use std::path::PathBuf;

use crate::common::host::IHostApi;

use super::api::bindings::ohosky::main::host;

pub struct HostApi;

impl IHostApi for HostApi {
    fn add(lhs: i32, rhs: i32) -> i32 {
        host::add(lhs, rhs)
    }

    fn check_file_permission(path: &std::path::Path) -> bool {
        let Some(path) = path.to_str() else {
            return false;
        };
        host::check_file_permission(path.as_bytes())
    }

    fn get_base_dir() -> PathBuf {
        host::get_base_dir().into()
    }

    fn get_config_dir() -> PathBuf {
        host::get_config_dir().into()
    }

    fn get_temp_dir() -> PathBuf {
        host::get_temp_dir().into()
    }

    fn get_locale() -> Option<String> {
        host::get_locale()
    }

    fn sky_module_args() -> Vec<String> {
        host::sky_module_args()
    }

    fn sky_module_token() -> secrecy::SecretString {
        host::sky_module_token().into()
    }
}
