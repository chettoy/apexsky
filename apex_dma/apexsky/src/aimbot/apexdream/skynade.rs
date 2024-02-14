use crate::math;
use super::pitches;

pub fn skynade_angle(
    weapon_id: u32,
    weapon_mod_bitfield: u32,
    weapon_projectile_scale: f32,
    weapon_projectile_speed: f32,
    local_view_origin: &[f32; 3],
    target: &[f32; 3],
) -> Option<(f32, f32)> {
    const WEAP_ID_THERMITE_GRENADE: u32 = 167;
    const WEAP_ID_FRAG_GRENADE: u32 = 168;
    const WEAP_ID_ARC_STAR: u32 = 169;

    let (lob, pitches, z_offset): (bool, &[pitches::Pitch], f32) =
        match (weapon_mod_bitfield & 0x4 != 0, weapon_id) {
            (false, WEAP_ID_THERMITE_GRENADE) => (false, &pitches::GRENADE_PITCHES, 0.0),
            (false, WEAP_ID_FRAG_GRENADE) => (true, &pitches::GRENADE_PITCHES, 70.0),
            (false, WEAP_ID_ARC_STAR) => (false, &pitches::ARC_PITCHES, 25.0),
            (true, WEAP_ID_THERMITE_GRENADE) => (false, &pitches::GRENADIER_GRENADE_PITCHES, 0.0),
            (true, WEAP_ID_FRAG_GRENADE) => (false, &pitches::GRENADIER_GRENADE_PITCHES, 70.0),
            (true, WEAP_ID_ARC_STAR) => (false, &pitches::GRENADIER_ARC_PITCHES, 25.0),
            _ => return Default::default(),
        };
    // println!("weap={},z={}", weapon_id, z_offset);

    let g = 750.0 * weapon_projectile_scale;
    let v0 = weapon_projectile_speed;

    let delta = math::sub(*target, *local_view_origin);
    let delta = math::add(delta, math::muls(delta, 20.0 / math::len(delta)));
    let dx = f32::sqrt(delta[0] * delta[0] + delta[1] * delta[1]);
    let dy = delta[2] + z_offset;

    let calc_angle = if lob { lob_angle } else { optimal_angle };
    // println!("dx={},dy={},v0={},g={}", dx, dy, v0, g);
    if let Some(launch_pitch) = calc_angle(dx, dy, v0, g) {
        let view_pitch = pitches::launch2view(pitches, launch_pitch);
        let view_yew = math::qangle(math::sub(*target, *local_view_origin))[1].to_radians();
        // println!("skynade({},{})", view_pitch, view_yew);
        return Some((view_pitch, view_yew));
    } else {
        return None;
    }

    fn optimal_angle(x: f32, y: f32, v0: f32, g: f32) -> Option<f32> {
        let root = v0 * v0 * v0 * v0 - g * (g * x * x + 2.0 * y * v0 * v0);
        if root < 0.0 {
            return None;
        }
        let root = f32::sqrt(root);
        let slope = (v0 * v0 - root) / (g * x);
        Some(f32::atan(slope))
    }
    fn lob_angle(x: f32, y: f32, v0: f32, g: f32) -> Option<f32> {
        let root = v0 * v0 * v0 * v0 - g * (g * x * x + 2.0 * y * v0 * v0);
        if root < 0.0 {
            return None;
        }
        let root = f32::sqrt(root);
        let slope = (v0 * v0 + root) / (g * x);
        Some(f32::atan(slope))
    }
}
