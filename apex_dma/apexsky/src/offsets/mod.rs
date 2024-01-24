use crate::{lock_mod, skyapex::offsets_loader::CustomOffsets};
use obfstr::obfstr as s;

#[no_mangle]
pub extern "C" fn import_offsets() -> CustomOffsets {
    include_flate::flate!(static OFFSETS_INI: str from "resource/default/offsets.ini");
    let offsets_file_path = std::env::current_dir().unwrap().join(s!("offsets.ini"));
    if !offsets_file_path.exists() {
        std::fs::write(offsets_file_path, OFFSETS_INI.to_owned())
            .expect(s!("Failed to write offsets.ini"));
    }
    CustomOffsets::load(&mut lock_mod!())
}
