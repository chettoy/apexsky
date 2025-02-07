pub mod common;
#[cfg(feature = "cxx")]
pub mod cxx;
#[cfg(feature = "ohosky")]
pub mod ohosky;
#[cfg(feature = "skydream")]
pub mod skydream;

#[macro_export]
#[cfg(feature = "skydream")]
macro_rules! skydream_main {
    ($main:ident) => {
        use $crate::skydream::api::safer_ffi;

        #[unsafe(no_mangle)]
        pub extern "C" fn _skydream_start(
            host_api: $crate::skydream::api::SkydreamApi,
            module_token: safer_ffi::String,
            module_args: safer_ffi::Vec<safer_ffi::String>,
        ) {
            $crate::skydream::SKY_HOST_API.set(host_api).unwrap();
            $crate::skydream::SKY_MODULE_TOKEN
                .set(Into::<String>::into(module_token).into())
                .unwrap();
            $crate::skydream::SKY_MODULE_ARGS
                .set(
                    Into::<Vec<_>>::into(module_args)
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                )
                .unwrap();
            ($main)();
        }
    };
}

#[macro_export]
#[cfg(feature = "ohosky")]
macro_rules! ohosky_main {
    ($main:ident) => {
        use $crate::common::host::IHostApi;
        use $crate::ohosky::api::bindings;
        use $crate::ohosky::HostApi;

        struct Component;

        impl bindings::Guest for Component {
            fn add(x: i32, y: i32) -> i32 {
                HostApi::add(x, y)
            }

            fn load() {
                ($main)();
            }
        }

        bindings::export!(Component with_types_in bindings);
    };
}

#[cfg(all(feature = "skydream", feature = "cxx"))]
unsafe extern "C" {
    fn sky_main();
}
#[cfg(all(feature = "skydream", feature = "cxx"))]
fn call_sky_main() {
    unsafe { sky_main() }
}
#[cfg(all(feature = "skydream", feature = "cxx"))]
skydream_main!(call_sky_main);
