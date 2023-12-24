use anyhow::Ok;
#[cfg(feature = "wasmedge")]
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    params, Vm, VmBuilder, WasmVal, WasmValue,
};
#[cfg(feature = "wasmer")]
use wasmer::{imports, Instance, Module, Store, Value};
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
    tokio_runtime: tokio::runtime::Runtime,
}

impl Skyapex {
    pub fn load() -> anyhow::Result<Self> {
        let mod_bytes = wat::parse_str(include_str!("../resource/mod/skyapex.wat"))?;
        #[cfg(feature = "wasmedge")]
        {
            let config = ConfigBuilder::new(CommonConfigOptions::default())
                .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
                .build()?;
            let vm = VmBuilder::new()
                .with_config(config)
                .build()?
                .register_module_from_bytes("skyapex", mod_bytes)?;
            vm.run_func(Some("skyapex"), "load", params!())?;

            Ok(Skyapex { vm })
        }
        #[cfg(feature = "wasmer")]
        {
            let mut store = Store::default();
            let module = Module::new(&store, &mod_bytes)?;
            let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            let _guard = tokio_runtime.enter();
            let mut wasi_env = WasiEnv::builder("skyapex")
                // .args(&["test"])
                // .env("KEY", "Value")
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
                tokio_runtime,
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

    pub fn add(&mut self, left: i32, right: i32) -> i32 {
        #[cfg(feature = "wasmedge")]
        {
            self.run_func("add", params!(left, right)).unwrap()[0].to_i32()
        }
        #[cfg(feature = "wasmer")]
        {
            self.run_func("add", &[Value::I32(left), Value::I32(right)])
                .unwrap()[0]
                .unwrap_i32()
        }
    }

    pub fn print_run_as_root(&mut self) {
        #[cfg(feature = "wasmedge")]
        self.run_func("print_run_as_root", params!()).unwrap();
        #[cfg(feature = "wasmer")]
        self.run_func("print_run_as_root", &[]).unwrap();
    }
}
