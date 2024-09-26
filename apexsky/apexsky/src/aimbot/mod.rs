pub mod ext;
pub mod ffi;
mod skynade;

use rand::Rng;
use serde::{Deserialize, Serialize};
use skyapex_sdk::module::AimbotUtils;
use std::{
    fmt::Debug,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::{instrument, trace};

use crate::noobfstr as s;
use crate::{lock_mod, Vec4};

use self::ext::math;

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct AimbotSettings {
    pub gamepad: bool,
    pub aim_mode: i32,
    pub auto_shoot: bool,
    pub ads_fov: f32,
    pub non_ads_fov: f32,
    pub auto_nade_aim: bool,
    pub no_recoil: bool,
    pub bone: i32,
    pub bone_nearest: bool,
    pub bone_auto: bool,
    pub max_dist: f32,
    pub aim_dist: f32,
    pub headshot_dist: f32,
    pub skynade_dist: f32,
    pub smooth: f32,
    pub skynade_smooth: f32,
    pub looting_smooth: f32,
    pub recoil_smooth_x: f32,
    pub recoil_smooth_y: f32,
}

impl Default for AimbotSettings {
    fn default() -> Self {
        Self {
            gamepad: false, // auto
            aim_mode: 10,   // avo (experiment 0x8, aim assist 0x4, vis check 0x2, on/off 0x1)
            auto_shoot: true,
            ads_fov: 12.0,
            non_ads_fov: 50.0,
            auto_nade_aim: true,
            no_recoil: false,
            bone: 2,
            bone_nearest: true,
            bone_auto: false,
            max_dist: 3800.0 * 40.0,
            aim_dist: 500.0 * 40.0,
            headshot_dist: 15.0 * 40.0,
            skynade_dist: 150.0 * 40.0,
            smooth: 200.0,
            skynade_smooth: 250.0 * 0.6667,
            looting_smooth: 80.0,
            recoil_smooth_x: 30.0,
            recoil_smooth_y: 30.0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct AimAngles {
    pub valid: bool,
    pub hitscan: bool,
    pub view_pitch: f32,
    pub view_yaw: f32,
    pub delta_pitch: f32,
    pub delta_yaw: f32,
    pub delta_pitch_min: f32,
    pub delta_pitch_max: f32,
    pub delta_yaw_min: f32,
    pub delta_yaw_max: f32,
    pub distance: f32,
}

impl Default for AimAngles {
    fn default() -> Self {
        Self {
            valid: false,
            hitscan: false,
            view_pitch: Default::default(),
            view_yaw: Default::default(),
            delta_pitch: Default::default(),
            delta_yaw: Default::default(),
            delta_pitch_min: Default::default(),
            delta_pitch_max: Default::default(),
            delta_yaw_min: Default::default(),
            delta_yaw_max: Default::default(),
            distance: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug)]
enum TriggerState {
    Idle = 0,
    WaitTrigger = 1,
    Trigger = 2,
    WaitRelease = 3,
    WaitTriggerLooting = 4,
    TriggerLooting = 5,
    WaitReleaseLooting = 6,
}

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CurrentWeaponInfo {
    pub weapon_id: i32,
    pub bullet_speed: f32,
    pub bullet_gravity: f32,
    pub weapon_zoom_fov: f32,
    pub weapon_mod_bitfield: u32,
    pub weapon_headshot: bool,
    pub weapon_semi_auto: bool,
}

impl Default for CurrentWeaponInfo {
    fn default() -> Self {
        Self {
            weapon_id: 0,
            bullet_speed: 0.0,
            bullet_gravity: 0.0,
            weapon_zoom_fov: 0.0,
            weapon_mod_bitfield: 0,
            weapon_headshot: false,
            weapon_semi_auto: false,
        }
    }
}

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Aimbot {
    settings: AimbotSettings,
    aiming: bool,
    gun_safety: bool,
    lock: bool,
    triggerbot_ready: bool,
    attack_state: i32,
    zoom_state: i32,
    aim_key_state: i32,
    triggerbot_key_state: i32,
    held_id: i32,
    held_grenade: bool,
    weapon_info: CurrentWeaponInfo,
    max_fov: f32,
    target_score_max: f32,
    local_entity: u64,
    aim_entity: u64,
    tmp_aimentity: u64,
    locked_aimentity: u64,
    love_aimentity: bool,
    game_fps: f32,
    triggerbot_state: TriggerState,
    triggerbot_trigger_time: u64,
    triggerbot_release_time: u64,
    quick_looting_state: i32,
    quick_looting_ready: bool,
}

impl Default for Aimbot {
    fn default() -> Self {
        Self {
            settings: AimbotSettings::default(),
            aiming: false,
            gun_safety: true,
            lock: false,
            triggerbot_ready: false,
            attack_state: 0,
            zoom_state: 0,
            aim_key_state: 0,
            triggerbot_key_state: 0,
            held_id: -999,
            held_grenade: false,
            weapon_info: CurrentWeaponInfo::default(),
            max_fov: 10.0,
            target_score_max: 0.0,
            local_entity: 0,
            aim_entity: 0,
            tmp_aimentity: 0,
            locked_aimentity: 0,
            love_aimentity: false,
            game_fps: 75.0,
            triggerbot_state: TriggerState::Idle,
            triggerbot_trigger_time: 0,
            triggerbot_release_time: 0,
            quick_looting_state: 0,
            quick_looting_ready: false,
        }
    }
}

pub trait TriggerBot {
    fn is_triggerbot_ready(&self) -> bool;
    fn poll_trigger_action(&mut self) -> i32;
    fn poll_looting_action(&mut self) -> i32;
    fn triggerbot_update(
        &mut self,
        aim_entity: Option<Arc<dyn AimEntity>>,
        aim_angles: &AimAngles,
        force_attack_state: i32,
    );
}

pub struct HitboxData {
    pub bone: i32,
    pub group: i32,
    pub bbmin: [f32; 3],
    pub bbmax: [f32; 3],
    pub bone_origin: [f32; 3],
    pub bone_parent: i32,
    pub radius: f32,
}

pub trait AimEntity: Debug + Send + Sync {
    fn get_entity_ptr(&self) -> u64;
    fn get_view_angles(&self) -> [f32; 3];
    fn get_cam_pos(&self) -> [f32; 3];
    fn get_sway_angles(&self) -> [f32; 3];
    fn get_abs_velocity(&self) -> [f32; 3];
    fn get_bone_position_by_hitbox(&self, id: u32) -> [f32; 3];
    fn get_bones_data(&self) -> Vec<HitboxData>;
    fn get_hitbox(&self) -> Vec<([f32; 3], ([f32; 3], [f32; 3]))>;
    fn get_position(&self) -> [f32; 3];
    fn get_recoil_angles(&self) -> [f32; 3];
    fn get_view_offset(&self) -> [f32; 3];
    fn get_team_num(&self) -> i32;
    fn get_health(&self) -> i32;
    fn get_shield_health(&self) -> i32;
    fn get_max_health(&self) -> i32;
    fn get_max_shield_health(&self) -> i32;
    fn get_visible_duration(&self) -> f64;
    fn is_alive(&self) -> bool;
    fn is_knocked(&self) -> bool;
    fn is_player(&self) -> bool;
    fn is_visible(&self) -> bool;
    fn is_loot(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct HitScanReport {
    pub hit: bool,
    pub nearest_hitbox: Option<([f32; 3], ([f32; 3], [f32; 3]))>,
    pub nearest_bone_pos: Option<[f32; 3]>,
}

impl Aimbot {
    pub fn new() -> Self {
        Aimbot::default()
    }

    pub fn get_settings(&self) -> &AimbotSettings {
        &self.settings
    }

    pub fn settings(&mut self, settings: AimbotSettings) {
        self.settings = settings;
    }

    pub fn is_aiming(&self) -> bool {
        self.aiming
    }

    pub fn is_grenade(&self) -> bool {
        self.held_grenade
    }

    pub fn is_headshot(&self) -> bool {
        self.weapon_info.weapon_headshot
    }

    pub fn is_semi_auto(&self) -> bool {
        self.weapon_info.weapon_semi_auto
    }

    pub fn is_locked(&self) -> bool {
        self.lock
    }

    pub fn get_max_fov(&self) -> f32 {
        self.max_fov
    }

    pub fn get_held_id(&self) -> i32 {
        self.held_id
    }

    #[instrument(skip(self))]
    pub fn update_held_id(&mut self, held_id: i32) {
        let held_id: u8 = if held_id < 0 {
            (255 + held_id).try_into().unwrap()
        } else {
            held_id.try_into().unwrap()
        };
        self.held_id = held_id as i32;
        self.held_grenade = held_id == 5;
        //tracing::trace!(?self.held_id, ?self.held_grenade, "{}", s!("711aac39-e83c-4788"));
    }

    pub fn get_weapon_id(&self) -> i32 {
        self.weapon_info.weapon_id
    }

    pub fn update_weapon_info(&mut self, weapon_info: CurrentWeaponInfo) {
        self.weapon_info = weapon_info;
    }

    pub fn get_gun_safety(&self) -> bool {
        self.gun_safety
    }

    pub fn set_gun_safety(&mut self, gun_safety: bool) {
        self.gun_safety = gun_safety;
    }

    pub fn get_aim_key_state(&self) -> i32 {
        self.aim_key_state
    }

    #[tracing::instrument]
    pub fn update_aim_key_state(&mut self, aim_key_state: i32) {
        self.aim_key_state = aim_key_state;
    }

    #[tracing::instrument]
    pub fn update_triggerbot_key_state(&mut self, triggerbot_key_state: i32) {
        self.triggerbot_key_state = triggerbot_key_state;
    }

    #[tracing::instrument]
    pub fn update_quick_looting_key_state(&mut self, quick_looting_key_state: i32) {
        self.quick_looting_state = quick_looting_key_state;
    }

    #[tracing::instrument]
    pub fn update_attack_state(&mut self, attack_state: i32) {
        self.attack_state = attack_state;
    }

    pub fn get_zoom_state(&self) -> i32 {
        self.zoom_state
    }

    #[tracing::instrument]
    pub fn update_zoom_state(&mut self, zoom_state: i32) {
        self.zoom_state = zoom_state;
    }

    #[tracing::instrument]
    pub fn get_aim_entity(&self) -> u64 {
        self.aim_entity
    }

    #[tracing::instrument]
    pub fn target_distance_check(&self, distance: f32) -> bool {
        if self.is_grenade() {
            distance <= self.settings.skynade_dist
        } else {
            distance <= self.settings.aim_dist
        }
    }

    #[tracing::instrument]
    fn calc_target_score(
        &self,
        fov: f32,
        distance: f32,
        visible: bool,
        _is_npc: bool,
        is_loot: bool,
    ) -> f32 {
        // Reduce weight for invisible targets
        const VIS_WEIGHTS: f32 = 12.5;
        // Increase weight for targets that are too close
        const CLOSE_WEIGHTS: f32 = 30.0 * 30.0 * 100.0; /* equals to 30 fov */
        // Increase weight for loots
        const LOOT_WEIGHTS: f32 = 30.0 * 30.0 * 100.0; /* equals to 30 fov */

        let score = (fov * fov) * 100.0
            + (distance * 0.025) * 10.0
            + (if visible { 0.0 } else { VIS_WEIGHTS })
            + (if distance < 3.0 * 40.0 {
                0.0
            } else {
                CLOSE_WEIGHTS
            })
            + (if is_loot { 0.0 } else { LOOT_WEIGHTS });
        /*
         fov:dist:score
          1  10m  100
          2  40m  400
          3  90m  900
          4  160m 1600
        */
        score
    }

    #[tracing::instrument]
    pub fn start_select_target(&mut self) {
        self.target_score_max =
            self.calc_target_score(50.0, self.settings.aim_dist, false, false, false);
        self.tmp_aimentity = 0;
    }

    #[tracing::instrument]
    pub fn add_select_target(
        &mut self,
        fov: f32,
        distance: f32,
        visible: bool,
        love: bool,
        is_npc: bool,
        is_loot: bool,
        target_ptr: u64,
    ) {
        if !self.target_distance_check(distance) {
            return;
        }

        let score = self.calc_target_score(fov, distance, visible, is_npc, is_loot);

        if score < self.target_score_max {
            self.target_score_max = score;
            self.tmp_aimentity = target_ptr;
            trace!("{}", s!("target selected"));
        }

        if self.aim_entity == target_ptr {
            self.love_aimentity = love;
            self.quick_looting_ready = is_loot && self.quick_looting_state > 0;

            // vis check for shooting current aim entity
            if self.settings.aim_mode & 0x2 != 0 && !self.is_grenade() {
                self.gun_safety = !visible;
            }
        }
    }

    #[tracing::instrument]
    pub fn finish_select_target(&mut self) {
        // set current aim entity
        if self.lock {
            // locked target
            self.aim_entity = self.locked_aimentity;
        } else {
            // or new target
            self.aim_entity = self.tmp_aimentity;
        }

        // disable safety if vis check or aimbot is turned off
        if self.settings.aim_mode & 0x2 == 0 && !self.is_grenade() {
            self.gun_safety = false;
        }

        trace!(
            aim_entity = self.aim_entity,
            aim_lock = self.lock,
            gun_safety = self.gun_safety
        );
    }

    pub fn lock_target(&mut self, target_ptr: u64) {
        self.lock = true;
        self.locked_aimentity = target_ptr;
    }

    pub fn cancel_locking(&mut self) {
        self.lock = false;
        self.locked_aimentity = 0;
    }

    /// Update aimbot state
    #[tracing::instrument]
    pub fn update(&mut self, local_entity: u64, game_fps: f32) {
        if local_entity == 0 {
            tracing::error!("invalid local_entity");
            return;
        }
        if game_fps < f32::EPSILON {
            tracing::error!("invalid game_fps");
            return;
        }
        self.local_entity = local_entity;
        self.game_fps = game_fps;

        if self.is_grenade() {
            // Update grenade safety state
            self.gun_safety = (!self.settings.auto_nade_aim && self.zoom_state == 0)
                || (self.settings.auto_nade_aim && self.zoom_state > 0);

            // Update aimbot fov for grenade
            self.max_fov = 999.9;
        } else if self.quick_looting_ready {
            // Update aimbot fov for quick looting
            self.max_fov = 999.9;
        } else {
            // Update aimbot fov
            self.max_fov = if self.zoom_state > 0 {
                self.settings.ads_fov
            } else {
                self.settings.non_ads_fov
            };
        }

        // Update aiming state
        self.aiming = self.settings.aim_mode > 0
            && if self.aim_key_state > 0 || self.quick_looting_ready {
                true
            } else if self.settings.gamepad && (self.attack_state > 0 || self.zoom_state > 0) {
                true
            } else {
                false
            };

        // Update triggerbot state
        self.triggerbot_ready = self.triggerbot_key_state > 0;

        // Update quick looting state
        if !(self.quick_looting_state > 0) {
            self.quick_looting_ready = false;
        }

        // Update target lock
        if !self.aiming
            || self.is_headshot()
            || self.get_zoom_state() == 0
            || (self.triggerbot_ready && !self.settings.auto_shoot)
            || (self.settings.aim_mode & 0x4 != 0)
        {
            self.cancel_locking();
        } else {
            self.lock_target(self.aim_entity);
        }

        trace!(
            gun_safety = self.gun_safety,
            max_fov = self.max_fov,
            aiming = self.aiming,
            triggerbot_ready = self.triggerbot_ready,
            aim_mode = self.settings.aim_mode,
        )
    }

    #[instrument(skip_all)]
    fn hit_scan(
        &self,
        view_origin: [f32; 3],
        view_angles: [f32; 3],
        target_origin: [f32; 3],
        target_vel: [f32; 3],
        target_hitboxes: Vec<([f32; 3], ([f32; 3], [f32; 3]))>,
    ) -> HitScanReport {
        let max_time = 2.0;
        let time_step = 0.00005;
        let radius_scale = 1.0;

        let view_direction = math::qvec(view_angles);
        let bone_origin = target_origin;

        let hitbox_radius = |(bbmin, bbmax): ([f32; 3], [f32; 3])| -> f32 {
            let size = math::sub(bbmax, bbmin);
            let volume = math::dot(size, size);
            f32::cbrt(volume / (4.0 / 3.0 * std::f32::consts::PI))
        };

        let hitpoints = {
            let mut hitpoints = Vec::with_capacity(256);
            if target_hitboxes.len() < 256 {
                for (hit_pos, hb) in &target_hitboxes {
                    let hit_radius = hitbox_radius(*hb);
                    let bone_pos = math::add(bone_origin, *hit_pos);
                    let radius = hit_radius * radius_scale;
                    hitpoints.push((bone_pos, radius));
                }
            } else {
                for i in 0..256 {
                    let fi = i as f32 / 256.0 * target_hitboxes.len() as i32 as f32;
                    let starti = fi.floor() as i32 as usize;
                    let endi = fi.ceil() as i32 as usize;
                    let t = fi.fract();

                    let Some(start) = target_hitboxes.get(starti) else {
                        break;
                    };
                    let Some(end) = target_hitboxes.get(endi) else {
                        break;
                    };

                    let start_pos = math::add(bone_origin, start.0);
                    let end_pos = math::add(bone_origin, end.0);

                    let bone_pos = math::lerp(start_pos, end_pos, t);

                    let start_radius = hitbox_radius(start.1);
                    let end_radius = hitbox_radius(end.1);
                    let radius = (start_radius + (end_radius - start_radius) * t) * radius_scale;

                    hitpoints.push((bone_pos, radius));
                }
            }
            hitpoints
        };

        let mut hit = false;
        let mut nearest_hitbox = None;
        let mut nearest_bone_pos = None;

        // raycast
        let mut min_bone_offset = f32::MAX;
        for (bone_pos, radius) in &hitpoints {
            let dist2 = math::dist2(
                math::project(view_origin, view_direction, *bone_pos),
                *bone_pos,
            );
            let offset = dist2 - radius * radius;

            if offset < min_bone_offset {
                nearest_bone_pos = Some(*bone_pos);
                min_bone_offset = offset;
            }
        }

        if self.quick_looting_ready {
            return HitScanReport {
                hit: min_bone_offset < 0.0,
                nearest_hitbox: None,
                nearest_bone_pos: None,
            };
        }

        // projectile
        if min_bone_offset < 40.0 * 40.0 {
            let target_hitboxes: Vec<_> = target_hitboxes
                .iter()
                .map(|(hit_pos, (bbmin, bbmax))| {
                    let bbmin = math::muls(*bbmin, 0.9);
                    let bbmax = math::muls(*bbmax, 0.9);
                    (*hit_pos, (bbmin, bbmax))
                })
                .collect();

            let mut nearest_hitbox_index = None;
            let mut min_bone_dist2 = f32::MAX;

            let v0 = math::muls(view_direction, self.weapon_info.bullet_speed);
            let g = self.weapon_info.bullet_gravity;

            let mut time = 0.0;
            while time < max_time {
                let projectile_pos = [
                    view_origin[0] + v0[0] * time,
                    view_origin[1] + v0[1] * time,
                    view_origin[2] + v0[2] * time - 0.5 * g * time * time,
                ];
                let equivalent_pos = math::sub(projectile_pos, math::muls(target_vel, time));
                if math::dist2(equivalent_pos, bone_origin) < (2.0 * 40.0) * (2.0 * 40.0) {
                    for (i, (hit_pos, (bbmin, bbmax))) in target_hitboxes.iter().enumerate() {
                        let bone_pos = math::add(bone_origin, *hit_pos);
                        let bone_dist2 = math::dist2(equivalent_pos, bone_pos);
                        if bone_dist2 < min_bone_dist2 {
                            nearest_hitbox_index = Some(i);
                            min_bone_dist2 = bone_dist2;
                        }
                        let pos_offset = math::sub(equivalent_pos, bone_pos);
                        if bbmin[0] < pos_offset[0]
                            && pos_offset[0] < bbmax[0]
                            && bbmin[1] < pos_offset[1]
                            && pos_offset[1] < bbmax[1]
                            && bbmin[2] < pos_offset[2]
                            && pos_offset[2] < bbmax[2]
                        {
                            hit = true;
                            break;
                        }
                    }
                    if hit {
                        break;
                    }
                }
                time += time_step;
            }
            if let Some(hb) = nearest_hitbox_index.and_then(|i| target_hitboxes.get(i)) {
                let (bone_pos, (bbmin, bbmax)) = hb;
                let bone_pos = math::add(bone_origin, *bone_pos);
                nearest_hitbox = Some((bone_pos, (*bbmin, *bbmax)));
                nearest_bone_pos = Some(bone_pos);
            }
        }
        HitScanReport {
            hit,
            nearest_hitbox,
            nearest_bone_pos,
        }
    }

    #[instrument(skip_all)]
    pub fn calc_best_aim(
        &self,
        from: &dyn AimEntity,
        target: &dyn AimEntity,
        //local_origin: [f32; 3],
        view_angles: [f32; 3],
        //target_origin: [f32; 3],
        //target_vel: [f32; 3],
    ) -> (AimAngles, HitScanReport, [f32; 3]) {
        let target_origin = target.get_position();
        let target_vel = target.get_abs_velocity();
        let local_origin = from.get_position();
        let view_origin = math::add(local_origin, from.get_view_offset());
        let camera_origin = from.get_cam_pos();
        //let view_angles = from.get_view_angles();
        let sway_angles = from.get_sway_angles();
        let distance = math::dist(camera_origin, target_origin);
        let delta_time = 1.0 / self.game_fps;
        let expect_headshot = self.is_headshot() && distance <= self.settings.headshot_dist;

        let hitscan = self.hit_scan(
            view_origin,
            view_angles,
            target_origin,
            target_vel,
            target.get_hitbox(),
        );

        let target_head_pos = target.get_bone_position_by_hitbox(0);
        let (target_bone_position_min, target_bone_position_max): ([f32; 3], [f32; 3]) =
            if expect_headshot {
                (target_head_pos, target_head_pos)
            } else if self.settings.bone_nearest {
                let lowest_aim_pos = math::muls(
                    math::add(target.get_bone_position_by_hitbox(3), target_origin),
                    0.5,
                );
                let best_hitbox = hitscan.nearest_hitbox.and_then(|hitbox| {
                    let (bone_pos, (bbmin, bbmax)) = hitbox;
                    if f32::min(bone_pos[2] + bbmin[2], bone_pos[2] + bbmax[2]) > lowest_aim_pos[2]
                    {
                        Some(hitbox)
                    } else {
                        None
                    }
                });
                let best_bone_pos = hitscan.nearest_bone_pos.and_then(|pos| {
                    if pos[2] > lowest_aim_pos[2] {
                        Some(pos)
                    } else {
                        None
                    }
                });
                match (best_hitbox, best_bone_pos, hitscan.hit) {
                    (Some(hitbox), _, false) => {
                        let (bone_pos, (bbmin, bbmax)) = hitbox;
                        let scale = if self.settings.auto_shoot && self.triggerbot_ready {
                            0.4
                        } else {
                            0.9
                        };
                        let min = math::add(bone_pos, math::muls(bbmin, scale));
                        let max = math::add(bone_pos, math::muls(bbmax, scale));
                        (min, max)
                    }
                    (None, Some(best_bone_pos), false) => (best_bone_pos, best_bone_pos),
                    _ => {
                        let min = [
                            target_head_pos[0],
                            target_head_pos[1],
                            target_head_pos[2] + 10.0,
                        ];
                        let max = if expect_headshot {
                            target.get_bone_position_by_hitbox(1)
                        } else {
                            lowest_aim_pos
                        };
                        (min, max)
                    }
                }
            } else if self.settings.bone_auto {
                (target_head_pos, target_origin)
            } else {
                let fixed_bone_pos =
                    target.get_bone_position_by_hitbox(self.settings.bone.try_into().unwrap());
                (fixed_bone_pos, fixed_bone_pos)
            };
        // tracing::trace!(
        //     ?target_bone_position_max,
        //     ?target_bone_position_min,
        //     "{}",
        //     s!("711aac39-e83c-4788")
        // );

        let aim_target: [f32; 3];

        if !self.is_grenade() {
            let fun_calc_angles = |local_camera_position: [f32; 3],
                                   target_bone_position: [f32; 3],
                                   target_vel: [f32; 3],
                                   bullet_speed: f32,
                                   bullet_grav: f32,
                                   _delta_time: f32| {
                let mut aim_target: [f32; 3] = target_bone_position;
                let mut calculated_angles = Vec4::default();

                if self.quick_looting_ready {
                    return (
                        calc_angle(&local_camera_position, &target_bone_position),
                        aim_target,
                    );
                }

                if bullet_speed > 1.0 {
                    let distance_to_target =
                        math::dist(target_bone_position, local_camera_position);
                    let time_to_target = distance_to_target / bullet_speed;
                    let target_pos_ahead = math::add(
                        target_bone_position,
                        math::muls(target_vel, time_to_target * 0.5),
                    );

                    aim_target = target_pos_ahead;

                    calculated_angles = ffi::linear_predict(
                        bullet_grav,
                        bullet_speed,
                        local_camera_position[0],
                        local_camera_position[1],
                        local_camera_position[2],
                        target_pos_ahead[0],
                        target_pos_ahead[1],
                        target_pos_ahead[2],
                        target_vel[0],
                        target_vel[1],
                        target_vel[2],
                    );
                }

                if calculated_angles.w.is_normal() {
                    trace!(?calculated_angles);
                } else {
                    let angles = calc_angle(&local_camera_position, &target_bone_position);
                    // tracing::debug!(
                    //     ?local_camera_position,
                    //     ?target_bone_position,
                    //     ?angles,
                    //     "{}",
                    //     s!("711aac39-e83c-4788")
                    // );
                    calculated_angles = (angles[0], angles[1]).into();
                }

                ([calculated_angles.x, calculated_angles.y, 0.0], aim_target)
            };

            let (calculated_angles_min, _) = fun_calc_angles(
                camera_origin,
                target_bone_position_min,
                target_vel,
                self.weapon_info.bullet_speed,
                self.weapon_info.bullet_gravity,
                delta_time,
            );
            let (calculated_angles_max, aim_pos) = fun_calc_angles(
                camera_origin,
                target_bone_position_max,
                target_vel,
                self.weapon_info.bullet_speed,
                self.weapon_info.bullet_gravity,
                delta_time,
            );
            aim_target = aim_pos;

            let mut calculated_angles_min =
                math::sub(calculated_angles_min, math::sub(sway_angles, view_angles));
            let mut calculated_angles_max =
                math::sub(calculated_angles_max, math::sub(sway_angles, view_angles));
            normalize_angles(&mut calculated_angles_min);
            normalize_angles(&mut calculated_angles_max);
            let mut delta_min = math::sub(calculated_angles_min, view_angles);
            let mut delta_max = math::sub(calculated_angles_max, view_angles);
            normalize_delta_angles(&mut delta_min);
            normalize_delta_angles(&mut delta_max);

            let mut delta = [0.0, 0.0, 0.0];
            if (delta_min[0] * delta_max[0]).is_sign_positive() {
                delta[0] = (delta_min[0] + delta_max[0]) * 0.5;
            }
            if (delta_min[1] * delta_max[1]).is_sign_positive() {
                delta[1] = (delta_min[1] + delta_max[1]) * 0.5;
            }
            //println!("{:.2},{:.2}  {:.2},{:.2}", delta_min[0], delta_min[1], delta_max[0], delta_max[1]);

            let target_fov = calc_fov(&[0.0, 0.0, 0.0], &delta);
            let max_fov = {
                let mut fov = self.max_fov;
                if distance < 160.0 {
                    fov += 30.0;
                }
                if distance < 80.0 {
                    fov += 60.0;
                }
                let zoom_fov = self.weapon_info.weapon_zoom_fov;
                if zoom_fov.is_normal() && (zoom_fov - 1.0).abs() > f32::EPSILON {
                    fov *= zoom_fov / 90.0
                }
                // When autofire is enabled, add up to an additional 30 fov to meet the requirement
                if self.settings.auto_shoot && self.triggerbot_ready {
                    fov = f32::max(f32::min(fov + 30.0, target_fov), fov);
                }
                fov
            };

            if target_fov > max_fov {
                trace!(target_fov, ?delta, "ExceededFOVThreshold");
                (AimAngles::default(), hitscan, aim_target)
            } else if delta[0].is_nan() || delta[1].is_nan() {
                tracing::error!(
                    ?delta,
                    ?delta_min,
                    ?delta_max,
                    ?view_angles,
                    ?calculated_angles_min,
                    ?calculated_angles_max,
                    ?sway_angles,
                    ?camera_origin
                );
                (AimAngles::default(), hitscan, aim_target)
            } else {
                (
                    AimAngles {
                        valid: true,
                        hitscan: hitscan.hit,
                        view_pitch: view_angles[0],
                        view_yaw: view_angles[1],
                        delta_pitch: delta[0],
                        delta_yaw: delta[1],
                        delta_pitch_min: delta_min[0],
                        delta_pitch_max: delta_max[0],
                        delta_yaw_min: delta_min[1],
                        delta_yaw_max: delta_max[1],
                        distance,
                    },
                    hitscan,
                    aim_target,
                )
            }
        } else {
            let target_origin =
                math::add(target.get_position(), math::muls(target_vel, delta_time));
            aim_target = target_origin;

            let target_angle = calc_angle(&view_origin, &target_origin);
            if target_angle[0].abs() > 80.0 {
                trace!("ExceededPitchThreshold");
                return (AimAngles::default(), hitscan, aim_target);
            }

            let skynade_angles = ffi::skynade_angle(
                self.weapon_info.weapon_id.try_into().unwrap(),
                self.weapon_info.weapon_mod_bitfield,
                self.weapon_info.bullet_gravity / 750.0,
                self.weapon_info.bullet_speed,
                view_origin[0],
                view_origin[1],
                view_origin[2],
                target_origin[0],
                target_origin[1],
                target_origin[2],
            );

            trace!(?view_angles, ?skynade_angles);
            if !skynade_angles.w.is_normal() {
                return (AimAngles::default(), hitscan, aim_target);
            }

            let target_aim_angles = [
                -skynade_angles.x.to_degrees(),
                skynade_angles.y.to_degrees(),
                0.0,
            ];
            trace!(weapon = ?self.weapon_info, ?target_aim_angles);

            let mut delta = math::sub(target_aim_angles, view_angles);
            normalize_delta_angles(&mut delta);

            let aim_angles = if delta[0].is_nan() || delta[1].is_nan() || delta[2].is_nan() {
                tracing::error!(
                    ?delta,
                    ?target_aim_angles,
                    ?view_angles,
                    ?skynade_angles,
                    ?view_origin,
                    ?target_origin
                );
                AimAngles::default()
            } else {
                AimAngles {
                    valid: true,
                    hitscan: false,
                    view_pitch: view_angles[0],
                    view_yaw: view_angles[1],
                    delta_pitch: delta[0],
                    delta_yaw: delta[1],
                    delta_pitch_min: delta[0],
                    delta_pitch_max: delta[0],
                    delta_yaw_min: delta[1],
                    delta_yaw_max: delta[1],
                    distance,
                }
            };
            (aim_angles, hitscan, aim_target)
        }
    }

    /// Calculates the delay in milliseconds before triggering the mechanism based on
    /// the Aimbot's current state and the provided `AimAngles`.
    ///
    /// This function is associated with the Aimbot trait and takes a reference to
    /// the Aimbot instance (`&self`) and a reference to `AimAngles` (`aim_angles: &AimAngles`)
    /// as parameters.
    ///
    /// # Parameters
    /// - `&self`: A reference to the Aimbot instance.
    /// - `aim_angles`: A reference to the `AimAngles` structure containing information
    ///                about the desired aiming angles.
    ///
    /// # Returns
    /// - If the Aimbot should be triggered, the function returns the delay in
    ///   milliseconds until the trigger should occur.
    /// - If no trigger is needed, the function returns 0.
    ///
    /// # Example
    /// ```rust
    /// let delay = aimbot.calculate_trigger_delay(&aim_angles);
    /// if delay > 0 {
    ///     // Perform the trigger action after the specified delay
    ///     // ...
    /// } else {
    ///     // No trigger action needed
    ///     // ...
    /// }
    /// ```
    ///
    #[tracing::instrument]
    pub fn calculate_trigger_delay(&self, aim_angles: &AimAngles) -> u64 {
        if !self.is_triggerbot_ready() || !aim_angles.valid {
            return 0;
        }

        if if self.settings.aim_mode & 0x8 != 0 {
            aim_angles.hitscan
        } else {
            lock_mod!().triggerbot_cross_hair_ready(
                aim_angles.view_pitch,
                aim_angles.view_yaw,
                aim_angles.delta_pitch,
                aim_angles.delta_yaw,
                aim_angles.delta_pitch_min,
                aim_angles.delta_pitch_max,
                aim_angles.delta_yaw_min,
                aim_angles.delta_yaw_max,
                aim_angles.distance,
                self.weapon_info.weapon_zoom_fov,
            ) > 0
        } {
            let delay = if self.love_aimentity {
                60..600
            } else {
                20..200
            };
            rand::thread_rng().gen_range(delay)
        } else {
            0
        }
    }

    pub fn smooth_aim_angles(&self, aim_angles: &AimAngles, smooth_factor: f32) -> (f32, f32) {
        assert!(aim_angles.valid);

        let smooth = if self.is_grenade() {
            self.settings.skynade_smooth
        } else if self.triggerbot_ready && self.settings.auto_shoot {
            (self.settings.smooth / 2.0).clamp(40.0, 90.0)
        } else if self.quick_looting_ready {
            self.settings.looting_smooth
        } else {
            self.settings.smooth
        } / smooth_factor;

        let smoothed = {
            let mut sm = lock_mod!();
            (
                sm.aimbot_smooth_x(
                    self.aim_entity as i64,
                    aim_angles.view_pitch,
                    aim_angles.delta_pitch,
                    smooth,
                ),
                sm.aimbot_smooth_y(
                    self.aim_entity as i64,
                    aim_angles.view_yaw,
                    aim_angles.delta_yaw,
                    smooth,
                ),
            )
        };
        if smoothed.0.is_nan() || smoothed.1.is_nan() {
            tracing::warn!(?aim_angles, ?smoothed);
        }
        smoothed
    }
}

impl TriggerBot for Aimbot {
    #[tracing::instrument]
    fn is_triggerbot_ready(&self) -> bool {
        (self.triggerbot_ready && !self.is_grenade()) || self.quick_looting_ready
    }

    #[tracing::instrument]
    fn poll_trigger_action(&mut self) -> i32 {
        let now_ms = get_unix_timestamp_in_millis();
        match self.triggerbot_state {
            TriggerState::WaitTrigger => {
                if now_ms > self.triggerbot_trigger_time {
                    self.triggerbot_state = TriggerState::Trigger;
                    trace!("trigger");
                    5
                } else {
                    0
                }
            }
            TriggerState::WaitRelease => {
                if now_ms > self.triggerbot_release_time {
                    self.triggerbot_state = TriggerState::Idle;
                    trace!("release");
                    4
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    #[tracing::instrument]
    fn poll_looting_action(&mut self) -> i32 {
        let now_ms = get_unix_timestamp_in_millis();
        match self.triggerbot_state {
            TriggerState::WaitTriggerLooting => {
                if now_ms > self.triggerbot_trigger_time {
                    self.triggerbot_state = TriggerState::TriggerLooting;
                    //println!("looting press");
                    5
                } else {
                    0
                }
            }
            TriggerState::WaitReleaseLooting => {
                if now_ms > self.triggerbot_release_time {
                    self.triggerbot_state = TriggerState::Idle;
                    //println!("looting release");
                    4
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    #[tracing::instrument]
    fn triggerbot_update(
        &mut self,
        aim_entity: Option<Arc<dyn AimEntity>>,
        aim_angles: &AimAngles,
        force_attack_state: i32,
    ) {
        let trigger_delay = self.calculate_trigger_delay(aim_angles);
        let now_ms = get_unix_timestamp_in_millis();
        let semi_auto = self.is_semi_auto();

        if trigger_delay > 0 {
            let attack_pressed = force_attack_state == 5;

            match self.triggerbot_state {
                TriggerState::Idle => {
                    // Prepare for the next trigger.
                    let viz_time = (aim_entity
                        .map(|ent| ent.get_visible_duration())
                        .unwrap_or(0.0)
                        * 1000.0) as u64;
                    let delay = if trigger_delay > viz_time {
                        trigger_delay - viz_time
                    } else {
                        0
                    };
                    if self.quick_looting_ready {
                        self.triggerbot_trigger_time = now_ms + delay;
                        self.triggerbot_state = TriggerState::WaitTriggerLooting;
                    } else if self.get_weapon_id() == 2 && attack_pressed {
                        // Release the drawn bow.
                        self.triggerbot_release_time = now_ms + delay;
                        self.triggerbot_state = TriggerState::WaitRelease;
                    } else if !attack_pressed {
                        // Do not interrupt user attacks
                        self.triggerbot_trigger_time = now_ms + delay;
                        self.triggerbot_state = TriggerState::WaitTrigger;
                    }
                }
                TriggerState::WaitTrigger => {
                    // Keep wait
                }
                TriggerState::Trigger => {
                    if semi_auto {
                        // No continuous triggering for headshot weapons
                        self.triggerbot_release_time =
                            now_ms + rand::thread_rng().gen_range(10..100);
                        self.triggerbot_state = TriggerState::WaitRelease;
                    } else {
                        // Keep triggering the trigger.
                    }
                }
                TriggerState::WaitRelease => {
                    if !semi_auto && self.get_weapon_id() != 2 {
                        // Cancel release
                        self.triggerbot_state = TriggerState::Trigger;
                    }
                }
                TriggerState::WaitTriggerLooting => (),
                TriggerState::TriggerLooting => {
                    // No long press for button_use
                    self.triggerbot_release_time = now_ms + rand::thread_rng().gen_range(10..100);
                    self.triggerbot_state = TriggerState::WaitReleaseLooting;
                }
                TriggerState::WaitReleaseLooting => (),
            }
        } else {
            match self.triggerbot_state {
                TriggerState::Idle => (),
                TriggerState::WaitTrigger => {
                    if semi_auto {
                        self.triggerbot_state = TriggerState::Idle;
                    }
                }
                TriggerState::Trigger => {
                    // It's time to release
                    self.triggerbot_release_time = now_ms + rand::thread_rng().gen_range(10..100);
                    self.triggerbot_state = TriggerState::WaitRelease;
                }
                TriggerState::WaitRelease => (),
                TriggerState::WaitTriggerLooting => {
                    self.triggerbot_state = TriggerState::Idle;
                }
                TriggerState::TriggerLooting => {
                    self.triggerbot_release_time = now_ms + rand::thread_rng().gen_range(10..100);
                    self.triggerbot_state = TriggerState::WaitReleaseLooting;
                }
                TriggerState::WaitReleaseLooting => (),
            }
        }
    }
}

/// Function to get the Unix timestamp in milliseconds
pub fn get_unix_timestamp_in_millis() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            // Calculate the total milliseconds from the duration
            duration.as_secs() * 1000 + duration.subsec_millis() as u64
        }
        Err(e) => {
            // Handle errors, such as clock rollback
            panic!("{}{}", s!("Error getting Unix Timestamp: "), e);
        }
    }
}

pub fn normalize_angles(angle: &mut [f32; 3]) {
    while angle[0] + f32::EPSILON > 90.0 {
        angle[0] -= 180.0;
    }
    while angle[0] - f32::EPSILON < -90.0 {
        angle[0] += 180.0;
    }
    while angle[1] > 180.0 - f32::EPSILON {
        angle[1] -= 360.0;
    }
    while angle[1] - f32::EPSILON < -180.0 {
        angle[1] += 360.0;
    }
}

pub fn normalize_delta_angles(angle: &mut [f32; 3]) {
    while angle[0] + f32::EPSILON > 180.0 {
        angle[0] -= 180.0;
    }
    while angle[0] - f32::EPSILON < -180.0 {
        angle[0] += 180.0;
    }
    while angle[1] > 180.0 - f32::EPSILON {
        angle[1] -= 360.0;
    }
    while angle[1] - f32::EPSILON < -180.0 {
        angle[1] += 360.0;
    }
}

pub fn calc_angle(src: &[f32; 3], dst: &[f32; 3]) -> [f32; 3] {
    let delta = math::sub(*src, *dst);
    let hyp = (delta[0] * delta[0] + delta[1] * delta[1]).sqrt();
    let mut angle = [
        (delta[2] / hyp).atan().to_degrees(),
        (delta[1] / delta[0]).atan().to_degrees(),
        0.0,
    ];
    if delta[0] >= 0.0 {
        angle[1] += 180.0;
    }
    angle
}

pub fn calc_fov(view_angle: &[f32; 3], aim_angle: &[f32; 3]) -> f32 {
    let mut delta = math::sub(*aim_angle, *view_angle);
    normalize_angles(&mut delta);
    (delta[0].powf(2.0) + delta[1].powf(2.0)).sqrt()
}
