//mod config_store;
mod global_store;
mod records;

pub use self::global_store::{ISharedStore, Record, Store, StoreBackend};
pub use records::*;

macro_rules! define_msg_name {
    ($export_name:ident@$struct_name:ident {$($field:ident,)*}) => {
        pub struct $struct_name {
            $(
                pub $field: String
            ),*
        }

        pub static $export_name: std::sync::LazyLock<$struct_name> =
            std::sync::LazyLock::new(|| $struct_name {
                $(
                    $field: obfstr::obfstr!(concat!("apex1.", stringify!($field))).to_string()
                ),*
            });
    };
}

macro_rules! define_global {
    ({$($field:ident($type:ty): $key:expr,)*}) => {
        $(
            pub struct $field;
            impl $crate::global::Record<$type> for $field {
                fn name() -> String {
                    obfstr::obfstr!($key).to_string()
                }
                fn name_id() -> u64 {
                    ((obfstr::hash!($key) as u64) << 32) + obfstr::random!(u32) as u64
                }
            }
        )*
    };
}

use {define_global, define_msg_name};
