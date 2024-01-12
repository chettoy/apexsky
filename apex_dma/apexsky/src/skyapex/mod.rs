pub mod aimbot_utils;
pub mod offsets_loader;
pub mod spectators;
pub mod utils;

use anyhow::Ok;
#[cfg(feature = "wasmedge")]
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    params, Vm, VmBuilder, WasmValue,
};
#[cfg(feature = "wasmer")]
use wasmer::{Instance, Module, Store, Value};
#[cfg(feature = "wasmer")]
use wasmer_wasix::{
    capabilities::Capabilities,
    default_fs_backing,
    os::tty_sys::SysTty,
    runtime::task_manager::tokio::TokioTaskManager,
    virtual_fs::{DeviceFile, FileSystem, PassthruFileSystem, RootFileSystemBuilder},
    virtual_net, PluggableRuntime, WasiEnv,
};

#[derive(Debug)]
pub struct Skyapex {
    #[cfg(feature = "wasmedge")]
    vm: Vm,
    #[cfg(feature = "wasmer")]
    store: Store,
    #[cfg(feature = "wasmer")]
    instance: Instance,
    #[cfg(feature = "wasmer")]
    _runtime: PluggableRuntime,
}

impl Skyapex {
    pub fn load() -> anyhow::Result<Self> {
        // {
        //     ctrlc::set_handler(|| {
        //         use super::Utils;
        //         crate::lock_mod!().clean_up();
        //         std::process::exit(0);
        //     })
        //     .unwrap();
        // }

        let mod_bytes = wat::parse_str(include_str!("../../resource/mod/skyapex.wat"))?;
        #[cfg(feature = "wasmedge")]
        {
            let config = ConfigBuilder::new(CommonConfigOptions::default())
                .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
                .build()?;
            let mut vm = VmBuilder::new()
                .with_config(config)
                // .with_plugin_wasi_nn()
                .build()?
                .register_module_from_bytes("skyapex", mod_bytes)?;
            let args: Vec<String> = std::env::args().collect();
            let envs: Vec<String> = std::env::vars()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect();
            let preopens = vec!["/", "/tmp", "/mnt", "/mnt/host:."];
            let wasi_module = vm.wasi_module_mut().expect("Not found wasi module");
            wasi_module.initialize(
                Some(args.iter().map(|s| s.as_str()).collect()),
                Some(envs.iter().map(|s| s.as_str()).collect()),
                Some(preopens),
            );
            vm.run_func(Some("skyapex"), "load", params!())?;

            Ok(Skyapex { vm })
        }
        #[cfg(feature = "wasmer")]
        {
            use std::path::{Path, PathBuf};
            use std::sync::Arc;

            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();

            let _guard = runtime.enter();

            let mut store = Store::default();
            let module = Module::new(&store, &mod_bytes)?;

            let engine = store.engine().clone();
            let runtime = {
                let mut rt = PluggableRuntime::new(Arc::new(TokioTaskManager::new(runtime)));
                rt.set_networking_implementation(virtual_net::host::LocalNetworking::default());
                let tty = Arc::new(SysTty::default());
                rt.set_tty(tty);
                rt.set_engine(Some(engine));
                rt
            };

            let mut wasi_env = WasiEnv::builder("skyapex")
                .args(std::env::args())
                .envs(std::env::vars())
                .sandbox_fs({
                    // If we preopen anything from the host then shallow copy it over
                    let root_fs = RootFileSystemBuilder::new()
                        .with_tty(Box::new(DeviceFile::new(
                            wasmer_wasix::types::__WASI_STDIN_FILENO,
                        )))
                        .build();
                    let fs_backing: Arc<dyn FileSystem + Send + Sync> =
                        Arc::new(PassthruFileSystem::new(default_fs_backing()));
                    root_fs.remove_dir(Path::new("/tmp"))?;
                    root_fs.create_dir(Path::new("/mnt"))?;
                    for (host, guest) in vec![
                        (PathBuf::from("/tmp"), "/tmp"),
                        (std::env::current_dir()?.to_path_buf(), "/mnt/host"),
                    ] {
                        let host = if !host.is_absolute() {
                            Path::new("/").join(host)
                        } else {
                            host
                        };
                        root_fs.mount(guest.into(), &fs_backing, host).unwrap();
                    }
                    root_fs
                })
                .preopen_dir(Path::new("/"))?
                .capabilities({
                    let mut caps = Capabilities::default();
                    caps.http_client = wasmer_wasix::http::HttpClientCapabilityV1::new_allow_all();
                    caps.threading.enable_asynchronous_threading = false;
                    caps
                })
                .finalize(&mut store)?;
            let import_object = wasi_env.import_object(&mut store, &module)?;
            let instance = Instance::new(&mut store, &module, &import_object)?;
            wasi_env.initialize(&mut store, instance.clone())?;
            instance
                .exports
                .get_function("load")?
                .call(&mut store, &[])?;
            Ok(Skyapex {
                store,
                instance,
                _runtime: runtime,
            })
        }
    }

    #[cfg(feature = "wasmedge")]
    fn run_func(
        &self,
        func_name: impl AsRef<str>,
        args: impl IntoIterator<Item = WasmValue>,
    ) -> anyhow::Result<Vec<WasmValue>> {
        let res = self.vm.run_func(Some("skyapex"), func_name, args)?;
        Ok(res)
    }
    #[cfg(feature = "wasmer")]
    fn run_func(
        &mut self,
        func_name: impl AsRef<str>,
        args: &[Value],
    ) -> anyhow::Result<Box<[Value]>> {
        let func = self.instance.exports.get_function(func_name.as_ref())?;
        let res = func.call(&mut self.store, args)?;
        Ok(res)
    }
}
