use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

// Conversion functions
impl From<(f32, f32)> for Vec4 {
    fn from(tup: (f32, f32)) -> Vec4 {
        Vec4 {
            x: tup.0,
            y: tup.1,
            z: 0.0,
            w: 1.0,
        }
    }
}

#[unsafe(no_mangle)]
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
) -> Vec4 {
    if let Some(tup) = super::skynade::skynade_angle(
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
    ) {
        tup.into()
    } else {
        Vec4::default()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn linear_predict(
    weapon_projectile_grav: f32,
    weapon_projectile_speed: f32,
    local_x: f32,
    local_y: f32,
    local_z: f32,
    target_x: f32,
    target_y: f32,
    target_z: f32,
    vel_x: f32,
    vel_y: f32,
    vel_z: f32,
) -> Vec4 {
    use crate::aimbot::ext::solver::{solve, LinearPredictor, ProjectileWeapon};
    struct Weapon(f32, f32);
    impl ProjectileWeapon for Weapon {
        fn projectile_speed(&self) -> f32 {
            self.0
        }
        fn projectile_gravity(&self) -> f32 {
            self.1
        }
    }

    let pos_origin = [local_x, local_y, local_z];
    let pos_target = [target_x, target_y, target_z];
    let vel = [vel_x, vel_y, vel_z];
    let weapon = Weapon(weapon_projectile_speed, weapon_projectile_grav);

    let predictor = LinearPredictor {
        origin: pos_target,
        velocity: vel,
    };

    if let Some(sol) = solve(&pos_origin, &weapon, &predictor) {
        // let hit = predictor.predict_position(sol.time);
        let pitch = -sol.pitch.to_degrees();
        let yaw = sol.yaw.to_degrees();
        (pitch, yaw).into()
    } else {
        Vec4::default()
    }
}
