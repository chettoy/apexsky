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
        sky!(.host.check_file_permission)(path.into())
    }

    fn get_base_dir() -> PathBuf {
        let ret = sky!(.host.get_base_dir)();
        String::from_utf8(ret.to_vec()).unwrap().into()
    }

    fn get_config_dir() -> PathBuf {
        let ret = sky!(.host.get_config_dir)();
        String::from_utf8(ret.to_vec()).unwrap().into()
    }

    fn get_temp_dir() -> PathBuf {
        let ret = sky!(.host.get_temp_dir)();
        String::from_utf8(ret.to_vec()).unwrap().into()
    }

    fn get_locale() -> Option<String> {
        sky!(.host.get_locale)()
            .into_rust()
            .map(|ret| String::from_utf8(ret.to_vec()).unwrap())
    }

    fn sky_module_args() -> Vec<String> {
        super::SKY_MODULE_ARGS.get().unwrap().clone()
    }

    fn sky_module_token() -> secrecy::SecretString {
        super::SKY_MODULE_TOKEN.get().unwrap().clone()
    }
}
