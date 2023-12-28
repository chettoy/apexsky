pub(crate) mod spectators;
pub(crate) mod utils;

use anyhow::Ok;
#[cfg(feature = "wasmedge")]
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    params, Vm, VmBuilder, WasmVal, WasmValue,
};
#[cfg(feature = "wasmer")]
use wasmer::{Instance, Module, Store, Value};
#[cfg(feature = "wasmer")]
use wasmer_wasix::WasiEnv;

#[derive(Debug)]
pub struct Skyapex {
    #[cfg(feature = "wasmedge")]
    vm: Vm,
    #[cfg(feature = "wasmer")]
    store: Store,
    #[cfg(feature = "wasmer")]
    instance: Instance,
    #[cfg(feature = "wasmer")]
    _tokio_runtime: tokio::runtime::Runtime,
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
            let preopens = vec![(".:/tmp")];
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
            let mut store = Store::default();
            let module = Module::new(&store, &mod_bytes)?;
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            let _guard = runtime.enter();
            let mut wasi_env = WasiEnv::builder("skyapex")
                .args(std::env::args())
                .envs(std::env::vars())
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
                _tokio_runtime: runtime,
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
