use std::{path::PathBuf, sync::LazyLock as Lazy};

use obfstr::obfstr as s;
#[cfg(not(feature = "native"))]
use ohosky_api::common::host::IHostApi;

use crate::lock_config;

#[cfg(not(feature = "native"))]
use crate::skyapi;

pub fn get_config_file_path() -> PathBuf {
    static S_CONF_FILENAME: Lazy<String> = Lazy::new(|| s!("settings.toml").to_string());
    #[cfg(not(feature = "native"))]
    static CONFIG_DIR: Lazy<PathBuf> = Lazy::new(skyapi::HostApi::get_config_dir);
    #[cfg(feature = "native")]
    static CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| std::env::current_dir().unwrap());
    std::fs::create_dir_all(CONFIG_DIR.as_path()).expect(s!("Failed to create config directory"));
    CONFIG_DIR.join(&*S_CONF_FILENAME)
}

#[unsafe(no_mangle)]
pub extern "C" fn load_settings() {
    lock_config!() =
        crate::config::get_configuration(&get_config_file_path()).unwrap_or_else(|e| {
            static S_MSG: Lazy<String> =
                Lazy::new(|| s!("Fallback to defalut configuration.").to_string());
            tracing::warn!(%e, "{}", &*S_MSG);
            println!("{}", &*S_MSG);
            crate::config::Config::default()
        });
    tracing::debug!("{}", s!("load settings"));
}

#[unsafe(no_mangle)]
pub extern "C" fn save_settings() -> bool {
    crate::config::save_configuration(&get_config_file_path(), lock_config!().to_owned())
        .map(|()| {
            tracing::debug!("{}", s!("save settings"));
            true
        })
        .unwrap_or_else(|e| {
            tracing::warn!(%e, "{}", s!("Failed to save settings"));
            println!("{}", e);
            false
        })
}
