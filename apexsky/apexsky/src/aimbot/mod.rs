pub mod ext;
pub mod ffi;
mod skynade;

use rand::Rng;
use serde::{Deserialize, Serialize};
use skyapex_sdk::module::AimbotUtils;
use std::{
    fmt::Debug,
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
    pub recoil_smooth_x: f32,
    pub recoil_smooth_y: f32,
}

impl Default for AimbotSettings {
    fn default() -> Self {
        Self {
            gamepad: false, // auto
            aim_mode: 2,    // 0 no aim, 1 aim with no vis check, 2 aim with vis check
            auto_shoot: true,
            ads_fov: 12.0,
            non_ads_fov: 50.0,
            auto_nade_aim: true,
            no_recoil: false,
            bone: 2,
            bone_nearest: false,
            bone_auto: true,
            max_dist: 3800.0 * 40.0,
            aim_dist: 500.0 * 40.0,
            headshot_dist: 15.0 * 40.0,
            skynade_dist: 150.0 * 40.0,
            smooth: 200.0,
            skynade_smooth: 250.0 * 0.6667,
            recoil_smooth_x: 30.0,
            recoil_smooth_y: 30.0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct AimAngles {
    pub valid: bool,
    pub view_pitch: f32,
    pub view_yew: f32,
    pub delta_pitch: f32,
    pub delta_yew: f32,
    pub delta_pitch_min: f32,
    pub delta_pitch_max: f32,
    pub delta_yew_min: f32,
    pub delta_yew_max: f32,
    pub distance: f32,
}

impl Default for AimAngles {
    fn default() -> Self {
        Self {
            valid: false,
            view_pitch: Default::default(),
            view_yew: Default::default(),
            delta_pitch: Default::default(),
            delta_yew: Default::default(),
            delta_pitch_min: Default::default(),
            delta_pitch_max: Default::default(),
            delta_yew_min: Default::default(),
            delta_yew_max: Default::default(),
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
        }
    }
}

pub trait TriggerBot {
    fn is_triggerbot_ready(&self) -> bool;
    fn poll_trigger_action(&mut self) -> i32;
    fn triggerbot_update(&mut self, aim_angles: &AimAngles, force_attack_state: i32);
}

pub trait AimEntity: Debug + Send + Sync {
    fn get_entity_ptr(&self) -> u64;
    fn get_view_angles(&self) -> [f32; 3];
    fn get_cam_pos(&self) -> [f32; 3];
    fn get_sway_angles(&self) -> [f32; 3];
    fn get_abs_velocity(&self) -> [f32; 3];
    fn get_bone_position_by_hitbox(&self, id: u32) -> [f32; 3];
    fn get_position(&self) -> [f32; 3];
    fn get_recoil_angles(&self) -> [f32; 3];
    fn get_view_offset(&self) -> [f32; 3];
    fn get_team_num(&self) -> i32;
    fn get_health(&self) -> i32;
    fn get_shield_health(&self) -> i32;
    fn is_alive(&self) -> bool;
    fn is_knocked(&self) -> bool;
    fn is_player(&self) -> bool;
    fn is_visible(&self) -> bool;
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
    pub fn update_attack_state(&mut self, attack_state: i32) {
        self.attack_state = attack_state;
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
    fn calc_target_score(&self, fov: f32, distance: f32, visible: bool) -> f32 {
        // Reduce weight for invisible targets
        const VIS_WEIGHTS: f32 = 12.5;
        // Increase weight for targets that are too close
        const CLOSE_WEIGHTS: f32 = 30.0 * 30.0 * 100.0; // equals to 30 fov

        let score = (fov * fov) * 100.0
            + (distance * 0.025) * 10.0
            + (if visible { 0.0 } else { VIS_WEIGHTS })
            + (if distance < 3.0 * 40.0 {
                0.0
            } else {
                CLOSE_WEIGHTS
            });
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
        self.target_score_max = self.calc_target_score(50.0, self.settings.aim_dist, false);
        self.tmp_aimentity = 0;
    }

    #[tracing::instrument]
    pub fn add_select_target(
        &mut self,
        fov: f32,
        distance: f32,
        visible: bool,
        love: bool,
        target_ptr: u64,
    ) {
        if !self.target_distance_check(distance) {
            return;
        }

        let score = self.calc_target_score(fov, distance, visible);

        if score < self.target_score_max {
            self.target_score_max = score;
            self.tmp_aimentity = target_ptr;
            trace!("{}", s!("target selected"));
        }

        if self.aim_entity == target_ptr {
            self.love_aimentity = love;

            // vis check for shooting current aim entity
            if self.settings.aim_mode == 2 && !self.is_grenade() {
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
        if self.settings.aim_mode < 2 && !self.is_grenade() {
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
        } else {
            // Update aimbot fov
            self.max_fov = if self.zoom_state > 0 {
                self.settings.ads_fov
            } else {
                self.settings.non_ads_fov
            }
        }

        // Update aiming state
        self.aiming = self.settings.aim_mode > 0
            && if self.aim_key_state > 0 {
                true
            } else if self.settings.gamepad && (self.attack_state > 0 || self.zoom_state > 0) {
                true
            } else {
                false
            };

        // Update triggerbot state
        self.triggerbot_ready = self.settings.auto_shoot && self.triggerbot_key_state > 0;

        // Update target lock
        if !self.aiming || self.triggerbot_ready {
            self.cancel_locking();
        }
        if self.aiming && !self.is_headshot() && !self.triggerbot_ready {
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
    pub fn calc_best_aim(
        &self,
        from: &dyn AimEntity,
        target: &dyn AimEntity,
    ) -> (AimAngles, [f32; 3]) {
        let view_angles = from.get_view_angles();
        let local_camera = from.get_cam_pos();
        let sway_angles = from.get_sway_angles();
        let target_vel = target.get_abs_velocity();
        let distance = math::dist(local_camera, target.get_position());

        let mut target_bone_position_min: [f32; 3];
        let mut target_bone_position_max: [f32; 3];

        let delta_time = 1.0 / self.game_fps;

        if self.is_headshot() && distance <= self.settings.headshot_dist {
            target_bone_position_max = target.get_bone_position_by_hitbox(0);
            target_bone_position_min = target_bone_position_max;
        } else if self.settings.bone_nearest {
            // find nearest bone
            target_bone_position_max = target.get_position();
            target_bone_position_min = target.get_bone_position_by_hitbox(0);
            let mut nearest_bone_dist = self.settings.max_dist;
            for i in 0..7 {
                let current_bone_position = target.get_bone_position_by_hitbox(i);
                let dist_from_crosshair = math::dist(current_bone_position, local_camera);
                if dist_from_crosshair < nearest_bone_dist {
                    target_bone_position_max = current_bone_position;
                    target_bone_position_min = target_bone_position_max;
                    nearest_bone_dist = dist_from_crosshair;
                }
                tracing::trace!(i, ?current_bone_position);
            }
        } else if self.settings.bone_auto {
            target_bone_position_max = target.get_position();
            target_bone_position_min = target.get_bone_position_by_hitbox(0);
            // tracing::trace!(
            //     ?target_bone_position_max,
            //     ?target_bone_position_min,
            //     "{}",
            //     s!("711aac39-e83c-4788")
            // );
        } else {
            target_bone_position_max =
                target.get_bone_position_by_hitbox(self.settings.bone.try_into().unwrap());
            target_bone_position_min = target_bone_position_max;
        }

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

                if bullet_speed > 1.0 {
                    let distance_to_target =
                        math::dist(target_bone_position, local_camera_position);
                    let time_to_target = distance_to_target / bullet_speed;
                    let target_pos_ahead =
                        math::add(target_bone_position, math::muls(target_vel, time_to_target));

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
                local_camera,
                target_bone_position_min,
                target_vel,
                self.weapon_info.bullet_speed,
                self.weapon_info.bullet_gravity,
                delta_time,
            );
            let (calculated_angles_max, aim_pos) = fun_calc_angles(
                local_camera,
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

            let max_fov = {
                let zoom_fov = self.weapon_info.weapon_zoom_fov;
                if zoom_fov.is_normal() && (zoom_fov - 1.0).abs() > f32::EPSILON {
                    self.max_fov * zoom_fov / 90.0
                } else {
                    self.max_fov
                }
            } + if distance < 160.0 { 30.0 } else { 0.0 };

            let target_fov = calc_fov(&[0.0, 0.0, 0.0], &delta);
            if target_fov > max_fov {
                trace!(target_fov, ?delta, "ExceededFOVThreshold");
                (AimAngles::default(), aim_target)
            } else {
                (
                    AimAngles {
                        valid: true,
                        view_pitch: view_angles[0],
                        view_yew: view_angles[1],
                        delta_pitch: delta[0],
                        delta_yew: delta[1],
                        delta_pitch_min: delta_min[0],
                        delta_pitch_max: delta_max[0],
                        delta_yew_min: delta_min[1],
                        delta_yew_max: delta_max[1],
                        distance,
                    },
                    aim_target,
                )
            }
        } else {
            let local_origin = from.get_position();
            let view_offset = from.get_view_offset();
            trace!(?view_offset);
            let view_origin = math::add(local_origin, view_offset);
            let target_origin =
                math::add(target.get_position(), math::muls(target_vel, delta_time));
            aim_target = target_origin;

            let target_angle = calc_angle(&view_origin, &target_origin);
            if target_angle[0].abs() > 80.0 {
                trace!("ExceededPitchThreshold");
                return (AimAngles::default(), aim_target);
            }

            let skynade_angles = ffi::skynade_angle(
                self.weapon_info.weapon_id.try_into().unwrap(),
                self.weapon_info.weapon_mod_bitfield.try_into().unwrap(),
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
                return (AimAngles::default(), aim_target);
            }

            let target_aim_angles = [
                -skynade_angles.x.to_degrees(),
                skynade_angles.y.to_degrees(),
                0.0,
            ];
            trace!(weapon = ?self.weapon_info, ?target_aim_angles);

            let mut delta = math::sub(target_aim_angles, view_angles);
            normalize_delta_angles(&mut delta);
            (
                AimAngles {
                    valid: true,
                    view_pitch: view_angles[0],
                    view_yew: view_angles[1],
                    delta_pitch: delta[0],
                    delta_yew: delta[1],
                    delta_pitch_min: delta[0],
                    delta_pitch_max: delta[0],
                    delta_yew_min: delta[1],
                    delta_yew_max: delta[1],
                    distance,
                },
                aim_target,
            )
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

        if lock_mod!().triggerbot_cross_hair_ready(
            aim_angles.view_pitch,
            aim_angles.view_yew,
            aim_angles.delta_pitch,
            aim_angles.delta_yew,
            aim_angles.delta_pitch_min,
            aim_angles.delta_pitch_max,
            aim_angles.delta_yew_min,
            aim_angles.delta_yew_max,
            aim_angles.distance,
            self.weapon_info.weapon_zoom_fov,
        ) > 0
        {
            let delay = if self.love_aimentity { 60..160 } else { 20..40 };
            rand::thread_rng().gen_range(delay)
        } else {
            0
        }
    }

    pub fn smooth_aim_angles(&self, aim_angles: &AimAngles, smooth_factor: f32) -> (f32, f32) {
        assert!(aim_angles.valid);

        let smooth = if self.is_grenade() {
            self.settings.skynade_smooth
        } else {
            self.settings.smooth
        } / smooth_factor;

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
                aim_angles.view_yew,
                aim_angles.delta_yew,
                smooth,
            ),
        )
    }
}

impl TriggerBot for Aimbot {
    #[tracing::instrument]
    fn is_triggerbot_ready(&self) -> bool {
        self.triggerbot_ready && !self.is_grenade()
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
    fn triggerbot_update(&mut self, aim_angles: &AimAngles, force_attack_state: i32) {
        let trigger_delay = self.calculate_trigger_delay(aim_angles);
        let now_ms = get_unix_timestamp_in_millis();
        let semi_auto = self.is_semi_auto();

        if trigger_delay > 0 {
            let attack_pressed = force_attack_state == 5;

            match self.triggerbot_state {
                TriggerState::Idle => {
                    // Prepare for the next trigger.
                    if self.get_weapon_id() == 2 && attack_pressed {
                        // Release the drawn bow.
                        self.triggerbot_release_time =
                            now_ms + rand::thread_rng().gen_range(10..60);
                        self.triggerbot_state = TriggerState::WaitRelease;
                    } else if !attack_pressed {
                        // Do not interrupt user attacks
                        self.triggerbot_trigger_time = now_ms + trigger_delay;
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
                    self.triggerbot_release_time = now_ms + rand::thread_rng().gen_range(0..100);
                    self.triggerbot_state = TriggerState::WaitRelease;
                }
                TriggerState::WaitRelease => (),
            }
        }
    }
}

/// Function to get the Unix timestamp in milliseconds
pub fn get_unix_timestamp_in_millis() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            // Calculate the total milliseconds from the duration
            let millis = duration.as_secs() * 1000 + duration.subsec_millis() as u64;
            millis
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
