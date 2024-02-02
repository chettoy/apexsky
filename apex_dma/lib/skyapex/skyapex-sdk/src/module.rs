mod aimbot_utils;
mod offsets_loader;
mod spectators;
mod utils;

use std::ffi::CString;

pub use aimbot_utils::*;
pub use offsets_loader::*;
pub use spectators::*;
pub use utils::*;

use anyhow::Ok;
use obfstr::obfstr as s;
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

#[skyapex_impl]
trait PassData {
    fn new_buf(&mut self, size: i32) -> i32;
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

        include_flate::flate!(static SKYAPEX_WAT: str from "mod/skyapex.wat");
        let mod_bytes = wat::parse_str(SKYAPEX_WAT.as_str())?;
        #[cfg(feature = "wasmedge")]
        {
            let config = ConfigBuilder::new(CommonConfigOptions::default())
                .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
                .build()?;
            let mut vm = VmBuilder::new()
                .with_config(config)
                // .with_plugin_wasi_nn()
                .build()?
                .register_module_from_bytes(s!("skyapex"), mod_bytes)?;
            let args: Vec<String> = std::env::args().collect();
            let envs: Vec<String> = std::env::vars()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect();
            let preopens = vec![
                s!("/").to_string(),
                s!("/tmp").to_string(),
                s!("/mnt").to_string(),
                s!("/mnt/host:.").to_string(),
            ];
            let wasi_module = vm.wasi_module_mut().expect(s!("Not found wasi module"));
            wasi_module.initialize(
                Some(args.iter().map(String::as_str).collect()),
                Some(envs.iter().map(String::as_str).collect()),
                Some(preopens.iter().map(String::as_str).collect()),
            );
            vm.run_func(Some(s!("skyapex")), s!("load"), params!())?;

            Ok(Skyapex { vm })
        }
        #[cfg(feature = "wasmer")]
        {
            use std::path::Path;
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

            let mut wasi_env = WasiEnv::builder(s!("skyapex"))
                .args(std::env::args())
                .envs(std::env::vars().filter(|(k, _v)| !k.starts_with("=")))
                .sandbox_fs({
                    // If we preopen anything from the host then shallow copy it over
                    let root_fs = RootFileSystemBuilder::new()
                        .with_tty(Box::new(DeviceFile::new(
                            wasmer_wasix::types::__WASI_STDIN_FILENO,
                        )))
                        .build();
                    let fs_backing: Arc<dyn FileSystem + Send + Sync> =
                        Arc::new(PassthruFileSystem::new(default_fs_backing()));
                    root_fs.remove_dir(Path::new(s!("/tmp")))?;
                    root_fs.create_dir(Path::new(s!("/mnt")))?;
                    for (host, guest) in vec![
                        (std::env::temp_dir(), s!("/tmp")),
                        (std::env::current_dir()?.to_path_buf(), s!("/mnt/host")),
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
                .get_function(s!("load"))?
                .call(&mut store, &[])?;
            Ok(Skyapex {
                store,
                instance,
                _runtime: runtime,
            })
        }
    }

    pub fn pass_string(&mut self, data: String) -> anyhow::Result<i32> {
        let cstr = CString::new(data).expect(s!("CString::new failed"));
        let data = cstr.as_bytes_with_nul();

        let new_ptr = self.new_buf(data.len().try_into()?);

        // Write the string into the lineary memory
        #[cfg(feature = "wasmedge")]
        {
            let mut memory = self
                .vm
                .named_module_mut(s!("skyapex"))?
                .memory(s!("memory"))?;
            memory.write(data, new_ptr as u32)?;
        }
        #[cfg(feature = "wasmer")]
        {
            let memory = self.instance.exports.get_memory(s!("memory"))?;
            let mem_view = memory.view(&self.store);
            mem_view.write(new_ptr as u64, data)?;
        }

        Ok(new_ptr)
    }

    #[cfg(feature = "wasmedge")]
    fn run_func(
        &self,
        func_name: impl AsRef<str>,
        args: impl IntoIterator<Item = WasmValue>,
    ) -> anyhow::Result<Vec<WasmValue>> {
        let res = self.vm.run_func(Some(s!("skyapex")), func_name, args)?;
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
