use std::path::PathBuf;

use super::sky;
use crate::common::host::IHostApi;

pub struct HostApi;

impl IHostApi for HostApi {
    fn add(lhs: i32, rhs: i32) -> i32 {
        sky!(.host.add)(lhs, rhs)
    }

    fn check_file_permission(path: &std::path::Path) -> bool {
        let Some(path) = path.to_str() else {
            return false;
        };
        sky!(.host.check_file_permission)(path.as_bytes().to_vec().into())
    }

    fn get_base_dir() -> PathBuf {
        sky!(.host.get_base_dir)().to_string().into()
    }

    fn get_config_dir() -> PathBuf {
        sky!(.host.get_config_dir)().to_string().into()
    }

    fn get_temp_dir() -> PathBuf {
        sky!(.host.get_temp_dir)().to_string().into()
    }

    fn get_locale() -> Option<String> {
        sky!(.host.get_locale)().into_rust().map(Into::into)
    }

    fn sky_module_args() -> Vec<String> {
        super::SKY_MODULE_ARGS.get().unwrap().clone()
    }

    fn sky_module_token() -> secrecy::SecretString {
        super::SKY_MODULE_TOKEN.get().unwrap().clone()
    }
}
