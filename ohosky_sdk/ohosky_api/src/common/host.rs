use obfstr::obfstr as s;
use std::path::PathBuf;

pub trait IHostApi {
    fn add(lhs: i32, rhs: i32) -> i32;
    fn check_file_permission(path: &std::path::Path) -> bool;
    fn get_base_dir() -> PathBuf;
    fn get_config_dir() -> PathBuf;
    fn get_log_dir() -> PathBuf {
        Self::get_base_dir().join(s!("log"))
    }
    fn get_temp_dir() -> PathBuf;
    fn get_locale() -> Option<String>;

    fn sky_module_args() -> Vec<String>;
    fn sky_module_token() -> secrecy::SecretString;
}
