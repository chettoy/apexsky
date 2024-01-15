pub mod ffi;

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{lock_mod, skyapex::aimbot_utils::AimbotUtils};

#[repr(C)]
#[allow(dead_code)]
enum WeaponId {
    R301 = 0,
    Sentinel = 1,
    Bow = 2,
    R2R5 = 3,
    SheilaStationary = 10,
    Rampage = 21,
    Sheila = 56,
    Melee,
    SnipersMark = 76,
    Alternator = 80,
    Re45 = 81,
    ChargeRifle = 83,
    Devotion = 84,
    Longbow = 85,
    Havoc = 86,
    Eva8 = 87,
    Flatline = 88,
    G7Scout = 89,
    Hemlock = 90,
    Kraber = 92,
    Lstar = 93,
    Mastiff = 95,
    Mozambique = 96,
    Prowler = 101,
    Peacekeeper = 103,
    R99 = 104,
    P2020 = 105,
    Spitfire = 106,
    TripleTake = 107,
    Wingman = 109,
    Volt = 110,
    _3030Repeater = 111,
    CarSmg = 112,
    Nemesis = 113,
    Hands = 114,
    ThrowingKnife = 158,
    GrenadeThermite = 159,
    GrenadeFrag = 160,
    GrenadeArcStar = 161,
    Max,
}

const IDWEAPON_SENTINEL: i32 = WeaponId::Sentinel as i32;
const IDWEAPON_BOW: i32 = WeaponId::Bow as i32;
const IDWEAPON_CHARGE_RIFLE: i32 = WeaponId::ChargeRifle as i32;
const IDWEAPON_LONGBOW: i32 = WeaponId::Longbow as i32;
const IDWEAPON_G7_SCOUT: i32 = WeaponId::G7Scout as i32;
const IDWEAPON_HEMLOCK: i32 = WeaponId::Hemlock as i32;
const IDWEAPON_KRABER: i32 = WeaponId::Kraber as i32;
const IDWEAPON_P2020: i32 = WeaponId::P2020 as i32;
const IDWEAPON_TRIPLE_TAKE: i32 = WeaponId::TripleTake as i32;
const IDWEAPON_WINGMAN: i32 = WeaponId::Wingman as i32;
const IDWEAPON_3030_REPEATER: i32 = WeaponId::_3030Repeater as i32;

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
            no_recoil: true,
            bone: 2,
            bone_nearest: false,
            bone_auto: true,
            max_dist: 3800.0 * 40.0,
            aim_dist: 500.0 * 40.0,
            headshot_dist: 15.0 * 40.0,
            skynade_dist: 150.0 * 40.0,
            smooth: 200.0,
            skynade_smooth: 200.0 * 0.6667,
            recoil_smooth_x: 51.4,
            recoil_smooth_y: 51.4,
        }
    }
}

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct AimAngles {
    valid: bool,
    view_pitch: f32,
    view_yew: f32,
    delta_pitch: f32,
    delta_yew: f32,
    delta_pitch_min: f32,
    delta_pitch_max: f32,
    delta_yew_min: f32,
    delta_yew_max: f32,
    distance: f32,
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
    weapon_id: i32,
    bullet_speed: f32,
    bullet_gravity: f32,
    weapon_zoom_fov: f32,
    weapon_mod_bitfield: i32,
    weapon_grenade: bool,
    weapon_headshot: bool,
    weapon_semi_auto: bool,
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
            weapon_id: -999,
            bullet_speed: 0.0,
            bullet_gravity: 0.0,
            weapon_zoom_fov: 0.0,
            weapon_mod_bitfield: 0,
            weapon_grenade: false,
            weapon_headshot: false,
            weapon_semi_auto: false,
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

impl Aimbot {
    pub fn new() -> Self {
        Aimbot::default()
    }

    pub fn get_settings(&self) -> AimbotSettings {
        self.settings.clone()
    }

    pub fn settings(&mut self, settings: AimbotSettings) {
        self.settings = settings;
    }

    pub fn is_aiming(&self) -> bool {
        self.aiming
    }

    pub fn is_grenade(&self) -> bool {
        self.weapon_grenade
    }

    pub fn is_headshot(&self) -> bool {
        self.weapon_headshot
    }

    pub fn is_semi_auto(&self) -> bool {
        self.weapon_semi_auto
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

    pub fn update_held_id(&mut self, held_id: i32) {
        self.held_id = held_id;
        self.weapon_grenade = self.held_id == -251;
    }

    pub fn get_weapon_id(&self) -> i32 {
        self.weapon_id
    }

    pub fn update_weapon_info(
        &mut self,
        weapon_id: i32,
        bullet_speed: f32,
        bullet_gravity: f32,
        weapon_zoom_fov: f32,
        weapon_mod_bitfield: i32,
    ) {
        self.weapon_id = weapon_id;
        self.bullet_speed = bullet_speed;
        self.bullet_gravity = bullet_gravity;
        self.weapon_zoom_fov = weapon_zoom_fov;
        self.weapon_mod_bitfield = weapon_mod_bitfield;

        if self.weapon_grenade {
            self.weapon_headshot = false;
            self.weapon_semi_auto = false;
        } else {
            self.weapon_headshot = {
                match self.weapon_id {
                    IDWEAPON_3030_REPEATER => true,
                    IDWEAPON_BOW => true,
                    IDWEAPON_CHARGE_RIFLE => true,
                    IDWEAPON_G7_SCOUT => true,
                    IDWEAPON_KRABER => true,
                    IDWEAPON_LONGBOW => true,
                    IDWEAPON_SENTINEL => true,
                    IDWEAPON_P2020 => false,
                    IDWEAPON_TRIPLE_TAKE => true,
                    IDWEAPON_WINGMAN => true,
                    _ => false,
                }
            };
            self.weapon_semi_auto = {
                match self.weapon_id {
                    IDWEAPON_3030_REPEATER => true,
                    IDWEAPON_BOW => true,
                    IDWEAPON_CHARGE_RIFLE => false,
                    IDWEAPON_G7_SCOUT => true,
                    IDWEAPON_HEMLOCK => true,
                    IDWEAPON_KRABER => false,
                    IDWEAPON_LONGBOW => true,
                    IDWEAPON_SENTINEL => false,
                    IDWEAPON_P2020 => true,
                    IDWEAPON_TRIPLE_TAKE => true,
                    IDWEAPON_WINGMAN => true,
                    _ => false,
                }
            };
        }
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

    pub fn update_aim_key_state(&mut self, aim_key_state: i32) {
        self.aim_key_state = aim_key_state;
    }

    pub fn update_triggerbot_key_state(&mut self, triggerbot_key_state: i32) {
        self.triggerbot_key_state = triggerbot_key_state;
    }

    pub fn update_attack_state(&mut self, attack_state: i32) {
        self.attack_state = attack_state;
    }

    pub fn update_zoom_state(&mut self, zoom_state: i32) {
        self.zoom_state = zoom_state;
    }

    pub fn get_aim_entity(&self) -> u64 {
        self.aim_entity
    }

    pub fn target_distance_check(&self, distance: f32) -> bool {
        if self.held_id == -251 {
            distance <= self.settings.skynade_dist
        } else {
            distance <= self.settings.aim_dist
        }
    }

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

    pub fn start_select_target(&mut self) {
        self.target_score_max = self.calc_target_score(50.0, self.settings.aim_dist, false);
        self.tmp_aimentity = 0;
    }

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
        }

        if self.aim_entity == target_ptr {
            self.love_aimentity = love;

            // vis check for shooting current aim entity
            if self.settings.aim_mode == 2 && !self.is_grenade() {
                self.gun_safety = !visible;
            }
        }
    }

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
    pub fn update(&mut self) {
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
            self.weapon_zoom_fov,
        ) > 0
        {
            rand::thread_rng().gen_range(40..100)
        } else {
            0
        }
    }

    pub fn smooth_aim_angles(&self, aim_angles: &AimAngles, smooth_factor: f32) -> (f32, f32) {
        assert!(aim_angles.valid);

        let smooth = if self.weapon_grenade {
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
    fn is_triggerbot_ready(&self) -> bool {
        self.triggerbot_ready && !self.love_aimentity && !self.is_grenade()
    }

    fn poll_trigger_action(&mut self) -> i32 {
        let now_ms = get_unix_timestamp_in_millis();
        match self.triggerbot_state {
            TriggerState::WaitTrigger => {
                if now_ms > self.triggerbot_trigger_time {
                    self.triggerbot_state = TriggerState::Trigger;
                    // println!("trigger");
                    5
                } else {
                    0
                }
            }
            TriggerState::WaitRelease => {
                if now_ms > self.triggerbot_release_time {
                    self.triggerbot_state = TriggerState::Idle;
                    // println!("release");
                    4
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn triggerbot_update(&mut self, aim_angles: &AimAngles, force_attack_state: i32) {
        // println!("force_attack={}", force_attack_state);

        let trigger_delay = self.calculate_trigger_delay(aim_angles);
        let now_ms = get_unix_timestamp_in_millis();

        if trigger_delay > 0 {
            let semi_auto = self.is_semi_auto();
            let attack_pressed = force_attack_state == 5;

            match self.triggerbot_state {
                TriggerState::Idle => {
                    // Prepare for the next trigger.
                    if self.weapon_id == 2 && attack_pressed {
                        // Release the drawn bow.
                        self.triggerbot_release_time =
                            now_ms + rand::thread_rng().gen_range(60..150);
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
                            now_ms + rand::thread_rng().gen_range(60..150);
                        self.triggerbot_state = TriggerState::WaitRelease;
                    } else {
                        // Keep triggering the trigger.
                    }
                }
                TriggerState::WaitRelease => {
                    if !semi_auto && self.weapon_id != 2 {
                        // Cancel release
                        self.triggerbot_state = TriggerState::Trigger;
                    }
                }
            }
        } else {
            match self.triggerbot_state {
                TriggerState::Idle => (),
                TriggerState::WaitTrigger => {
                    self.triggerbot_state = TriggerState::Idle;
                }
                TriggerState::Trigger => {
                    // It's time to release
                    self.triggerbot_release_time = now_ms + rand::thread_rng().gen_range(0..150);
                    self.triggerbot_state = TriggerState::WaitRelease;
                }
                TriggerState::WaitRelease => (),
            }
        }
    }
}

/// Function to get the Unix timestamp in milliseconds
fn get_unix_timestamp_in_millis() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            // Calculate the total milliseconds from the duration
            let millis = duration.as_secs() * 1000 + duration.subsec_millis() as u64;
            millis
        }
        Err(e) => {
            // Handle errors, such as clock rollback
            panic!("Error getting Unix Timestamp: {}", e);
        }
    }
}
