use std::ffi::CStr;

use libc::c_char;

mod math;
mod pitches;
mod skynade;

#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

#[no_mangle]
pub extern "C" fn print_run_as_root() {
    println!("Please run as root!");
}

#[repr(C)]
pub struct Vector2D {
    x: f32,
    y: f32,
}

// Conversion functions
impl From<(f32, f32)> for Vector2D {
    fn from(tup: (f32, f32)) -> Vector2D {
        Vector2D { x: tup.0, y: tup.1 }
    }
}

impl From<Vector2D> for (f32, f32) {
    fn from(tup: Vector2D) -> (f32, f32) {
        (tup.x, tup.y)
    }
}

#[no_mangle]
pub extern "C" fn skynade_angle(
    weapon_id: u32,
    weapon_mod_bitfield: u32,
    weapon_projectile_scale: f32,
    weapon_projectile_speed: f32,
    local_view_origin_x: f32,
    local_view_origin_y: f32,
    local_view_origin_z: f32,
    target_x: f32,
    target_y: f32,
    target_z: f32,
) -> Vector2D {
    skynade::skynade_angle(
        weapon_id,
        weapon_mod_bitfield,
        weapon_projectile_scale,
        weapon_projectile_speed,
        &[
            local_view_origin_x,
            local_view_origin_y,
            local_view_origin_z,
        ],
        &[target_x, target_y, target_z],
    )
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
