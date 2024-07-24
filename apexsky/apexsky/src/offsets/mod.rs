use crate::lock_mod;
use obfstr::obfstr as s;
use once_cell::sync::Lazy;
pub use skyapex_sdk::module::CustomOffsets;

pub static G_OFFSETS: Lazy<CustomOffsets> = Lazy::new(|| import_offsets());

#[no_mangle]
#[tracing::instrument]
pub extern "C" fn import_offsets() -> CustomOffsets {
    let offsets_file_path = std::env::current_dir().unwrap().join(s!("offsets.ini"));
    if offsets_file_path.exists() {
        tracing::warn!(
            "{}{}{}",
            s!("Reading offsets from `"),
            offsets_file_path.to_string_lossy(),
            s!("`")
        );
        CustomOffsets::from_file(&mut lock_mod!())
    } else {
        include_flate::flate!(static OFFSETS_INI: str from "resource/default/offsets.ini");
        CustomOffsets::from_string(&mut lock_mod!(), OFFSETS_INI.to_owned())
    }
}
