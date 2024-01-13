use crate::{
    lock_mod,
    skyapex::offsets_loader::CustomOffsets,
};

#[no_mangle]
pub fn import_offsets() -> CustomOffsets {
    let default_offsets = include_str!("../../resource/default/offsets.ini");
    let offsets_file_path = std::env::current_dir().unwrap().join("offsets.ini");
    if !offsets_file_path.exists() {
        std::fs::write(offsets_file_path, default_offsets).expect("Failed to write offsets.ini");
    }
    CustomOffsets::load(&mut lock_mod!())
}
