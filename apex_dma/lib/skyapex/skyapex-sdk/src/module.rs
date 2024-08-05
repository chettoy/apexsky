mod aimbot_utils;
mod offsets_loader;
mod spectators;
mod utils;

use once_cell::sync::Lazy;
use std::ffi::CString;

pub use aimbot_utils::*;
pub use offsets_loader::*;
pub use spectators::*;
pub use utils::*;

use obfstr::obfstr as s;

#[cfg(feature = "wasmedge")]
use wasmedge_sdk::{
    params, plugin::PluginManager, wasi::WasiModule, AsInstance, Module, Store, Vm, WasmValue,
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

#[cfg(feature = "wasmtime")]
use wasmtime::{Config, Engine, Instance, Linker, Module, Store, Val};
#[cfg(feature = "wasmtime")]
use wasmtime_wasi::preview1::{self, WasiP1Ctx};
#[cfg(feature = "wasmtime")]
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder};

#[cfg(feature = "wasmedge")]
pub struct Skyapex {
    inner: SkyapexWasmedge<'static>,
}

#[cfg(feature = "wasmer")]
#[derive(Debug)]
pub struct Skyapex {
    store: Store,
    instance: Instance,
    _runtime: PluggableRuntime,
}

#[cfg(feature = "wasmtime")]
pub struct Skyapex {
    config: Config,
    engine: Engine,
    linker: Linker<WasiP1Ctx>,
    store: Store<WasiP1Ctx>,
    module: Module,
    instance: Instance,
    runtime: tokio::runtime::Runtime,
}

#[skyapex_impl]
trait PassData {
    fn new_buf(&mut self, size: i32) -> i32;
}

static S_LOAD: Lazy<String> = Lazy::new(|| s!("load").to_string());
static S_MEMORY: Lazy<String> = Lazy::new(|| s!("memory").to_string());
static S_PATH_HOST: Lazy<String> = Lazy::new(|| s!("/mnt/host").to_string());
static S_PATH_MNT: Lazy<String> = Lazy::new(|| s!("/mnt").to_string());
static S_PATH_TMP: Lazy<String> = Lazy::new(|| s!("/tmp").to_string());
static S_SKYAPEX: Lazy<String> = Lazy::new(|| s!("skyapex").to_string());

#[cfg(feature = "wasmedge")]
pub struct SkyapexWasmedge<'a> {
    // _leaked_wasi: std::sync::Mutex<Option<*mut WasiModule>>,
    // _leaked_wasi_nn: std::sync::Mutex<Option<*mut Option<wasmedge_sdk::Instance>>>,
    vm: Vm<'a, wasmedge_sdk::Instance>,
}

// impl<'a> Drop for SkyapexWasmedge<'a> {
//     fn drop(&mut self) {
//         if let Some(ptr) = self._leaked_wasi.lock().unwrap().take() {
//             unsafe { drop(Box::from_raw(ptr)) }
//         }
//         if let Some(ptr) = self._leaked_wasi_nn.lock().unwrap().take() {
//             unsafe { drop(Box::from_raw(ptr)) }
//         }
//     }
// }

#[cfg(feature = "wasmedge")]
impl<'a> SkyapexWasmedge<'a> {
    pub fn load(module_name: &str, module_bytes: Vec<u8>) -> anyhow::Result<Self> {
        use std::collections::HashMap;
        use wasmedge_sdk::config::{
            CommonConfigOptions, ConfigBuilder, RuntimeConfigOptions, StatisticsConfigOptions,
        };

        let common_options = CommonConfigOptions::default()
            .bulk_memory_operations(true)
            .multi_value(true)
            .mutable_globals(true)
            .non_trap_conversions(true)
            .reference_types(true)
            .sign_extension_operators(true)
            .simd(true);

        let stat_options = StatisticsConfigOptions::default()
            .count_instructions(true)
            .measure_cost(true)
            .measure_time(true);

        let runtime_options = RuntimeConfigOptions::default();

        let config = ConfigBuilder::new(common_options)
            .with_statistics_config(stat_options)
            .with_runtime_config(runtime_options)
            .build()?;

        PluginManager::load(None)?;

        let mut instances = HashMap::new();

        let args: Vec<String> = std::env::args().collect();
        let envs: Vec<String> = std::env::vars()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect();
        let preopens: Vec<String> = vec![
            s!("/").to_string(),
            S_PATH_TMP.to_owned(),
            S_PATH_MNT.to_owned(),
            format!("{}{}", &*S_PATH_HOST, s!(":.")),
        ];

        let wasi = WasiModule::create(
            Some(args.iter().map(String::as_str).collect()),
            Some(envs.iter().map(String::as_str).collect()),
            Some(preopens.iter().map(String::as_str).collect()),
        )?;
        let ptr_wasi = Box::into_raw(Box::new(wasi));
        let leaked_wasi = unsafe { &mut *ptr_wasi };

        let wasi_nn = PluginManager::load_plugin_wasi_nn().ok();
        let ptr_wasi_nn = Box::into_raw(Box::new(wasi_nn));
        let leaked_wasi_nn = unsafe { &mut *ptr_wasi_nn };

        instances.insert(
            leaked_wasi.name().unwrap().to_string(),
            leaked_wasi.as_mut(),
        );
        if let Some(wasi_nn) = leaked_wasi_nn {
            instances.insert(wasi_nn.name().unwrap().to_string(), wasi_nn);
        }

        let store = Store::new(Some(&config), instances)?;

        let mut vm = Vm::new(store);

        let module = Module::from_bytes(Some(&config), module_bytes)?;

        vm.register_module(Some(module_name), module)?;

        vm.run_func(Some(module_name), &*S_LOAD, params!())?;

        Ok(Self {
            // _leaked_wasi: std::sync::Mutex::new(Some(ptr_wasi)),
            // _leaked_wasi_nn: std::sync::Mutex::new(Some(ptr_wasi_nn)),
            vm,
        })
    }
}

impl Skyapex {
    pub fn load() -> anyhow::Result<Self> {
        include_flate::flate!(static SKYAPEX_WAT: str from "mod/skyapex.wat");
        let mod_bytes = wat::parse_str(SKYAPEX_WAT.as_str())?;
        #[cfg(feature = "wasmedge")]
        {
            Ok(Self {
                inner: SkyapexWasmedge::load(&*S_SKYAPEX, mod_bytes)?,
            })
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

            let mut wasi_env = WasiEnv::builder(&*S_SKYAPEX)
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
                    root_fs.remove_dir(Path::new(&*S_PATH_TMP))?;
                    root_fs.create_dir(Path::new(&*S_PATH_MNT))?;
                    for (host, guest) in vec![
                        (std::env::temp_dir(), &*S_PATH_TMP),
                        (std::env::current_dir()?.to_path_buf(), &*S_PATH_HOST),
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
                .get_function(&*S_LOAD)?
                .call(&mut store, &[])?;
            Ok(Skyapex {
                store,
                instance,
                _runtime: runtime,
            })
        }
        #[cfg(feature = "wasmtime")]
        {
            async fn load(
                mod_bytes: Vec<u8>,
            ) -> anyhow::Result<(
                Config,
                Engine,
                Linker<WasiP1Ctx>,
                Store<WasiP1Ctx>,
                Module,
                Instance,
            )> {
                // Construct the wasm engine with async support enabled.
                let mut config = Config::new();
                config.async_support(true);
                let engine = Engine::new(&config)?;

                // Add the WASI preview1 API to the linker (will be implemented in terms of
                // the preview2 API)
                let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
                preview1::add_to_linker_async(&mut linker, |t| t)?;

                // Add capabilities (e.g. filesystem access) to the WASI preview2 context
                // here. Here only stdio is inherited, but see docs of `WasiCtxBuilder` for
                // more.
                let wasi_ctx = WasiCtxBuilder::new()
                    .allow_ip_name_lookup(true)
                    .inherit_args()
                    .inherit_env()
                    .inherit_network()
                    .inherit_stderr()
                    .inherit_stdin()
                    .inherit_stdio()
                    .preopened_dir("/", "/", DirPerms::READ, FilePerms::READ)?
                    .preopened_dir(
                        std::env::temp_dir(),
                        &*S_PATH_TMP,
                        DirPerms::all(),
                        FilePerms::all(),
                    )?
                    .preopened_dir("..", &*S_PATH_MNT, DirPerms::all(), FilePerms::all())?
                    .preopened_dir(
                        std::env::current_dir()?.to_path_buf(),
                        &*S_PATH_HOST,
                        DirPerms::all(),
                        FilePerms::all(),
                    )?
                    .build_p1();

                let mut store = Store::new(&engine, wasi_ctx);

                let module = Module::from_binary(&engine, &mod_bytes)?;

                // Instantiate module
                let instance = linker.instantiate_async(&mut store, &module).await?;
                linker.instance(&mut store, &*S_SKYAPEX, instance.clone())?;

                // // Instantiate module
                // let linker = linker
                //     .module_async(&mut store, &*S_SKYAPEX, &module)
                //     .await?;

                // // Invoke the WASI program default function.
                // let func = linker
                //     .get_default(&mut store, &*S_SKYAPEX)?
                //     .typed::<(), ()>(&store)?;
                // func.call_async(&mut store, ()).await?;

                let func = instance.get_typed_func::<(), ()>(&mut store, &*S_LOAD)?;
                func.call_async(&mut store, ()).await?;

                Ok((config, engine, linker, store, module, instance))
            }

            tokio::task::block_in_place(|| {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                runtime.block_on(load(mod_bytes)).map(
                    |(config, engine, linker, store, module, instance)| Self {
                        config,
                        engine,
                        linker,
                        store,
                        module,
                        instance,
                        runtime,
                    },
                )
            })
        }
    }

    pub fn pass_string(&mut self, data: String) -> anyhow::Result<i32> {
        let cstr = CString::new(data)?;
        let data = cstr.as_bytes_with_nul();

        let new_ptr = self.new_buf(data.len().try_into()?);

        // Write the string into the lineary memory
        #[cfg(feature = "wasmedge")]
        {
            let (module_instance, _executor) = self
                .inner
                .vm
                .store_mut()
                .get_named_wasm_and_executor(&*S_SKYAPEX)
                .ok_or(anyhow::anyhow!(s!("err get named wasm").to_string()))?;
            let mut memory = module_instance.get_memory_mut(&*S_MEMORY)?;
            memory
                .write(new_ptr as usize, data)
                .ok_or(anyhow::anyhow!(s!("err write wasm memory").to_string()))?;
        }
        #[cfg(feature = "wasmer")]
        {
            let memory = self.instance.exports.get_memory(&*S_MEMORY)?;
            let mem_view = memory.view(&self.store);
            mem_view.write(new_ptr as u64, data)?;
        }
        #[cfg(feature = "wasmtime")]
        {
            let memory = self
                .instance
                .get_memory(&mut self.store, &*S_MEMORY)
                .ok_or(anyhow::anyhow!(s!("err get wasm memory").to_string()))?;
            memory.write(&mut self.store, new_ptr as usize, data)?;
        }

        Ok(new_ptr)
    }

    #[cfg(feature = "wasmedge")]
    fn run_func(
        &mut self,
        func_name: impl AsRef<str>,
        args: impl IntoIterator<Item = WasmValue>,
    ) -> anyhow::Result<Vec<WasmValue>> {
        let res = self.inner.vm.run_func(Some(&*S_SKYAPEX), func_name, args)?;
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
    // #[cfg(feature = "wasmtime")]
    // fn run_func(
    //     &mut self,
    //     func_name: impl AsRef<str>,
    //     args: &[Val],
    //     results: &mut [Val],
    // ) -> anyhow::Result<()> {
    //     let Some(func) = self.instance.get_func(&mut self.store, func_name.as_ref()) else {
    //         anyhow::bail!("`{}`{}", func_name.as_ref(), " not a function");
    //     };

    //     tokio::task::block_in_place(|| {
    //         self.runtime
    //             .block_on(func.call_async(&mut self.store, args, results))
    //     })?;

    //     Ok(())
    // }
    #[cfg(feature = "wasmtime")]
    fn run_func_typed<Params, Results>(
        &mut self,
        func_name: impl AsRef<str>,
        args: Params,
    ) -> anyhow::Result<Results>
    where
        Params: wasmtime::WasmParams,
        Results: wasmtime::WasmResults,
    {
        let func = self
            .instance
            .get_typed_func::<Params, Results>(&mut self.store, func_name.as_ref())?;

        let ret = tokio::task::block_in_place(|| {
            self.runtime
                .block_on(func.call_async(&mut self.store, args))
        })?;

        Ok(ret)
    }
}
