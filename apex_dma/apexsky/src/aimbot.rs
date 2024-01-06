use serde::{Deserialize, Serialize};

#[allow(dead_code)]
enum WeaponId {
    R301 = 0,
    Sentinel = 1,
    Bow = 2,
    SheilaStationary = 10,
    Sheila = 56,
    Rampage = 20,
    Melee = 113,
    SnipersMark = 76,
    Alternator = 79,
    Re45,
    ChargeRifle = 82,
    Devotion,
    Longbow = 84,
    Havoc,
    Eva8,
    Flatline,
    G7Scout = 88,
    Hemlock,
    Kraber = 91,
    Lstar,
    Mastiff = 94,
    Mozambique,
    Prowler = 101,
    Peacekeeper,
    R99 = 103,
    P2020,
    Spitfire = 105,
    TripleTake = 106,
    Wingman = 108,
    Volt,
    _3030Repeater = 110,
    CarSmg = 111,
    Nemesis,
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
const IDWEAPON_KRABER: i32 = WeaponId::Kraber as i32;
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
            gamepad: false,
            aim_mode: 2, // 0 no aim, 1 aim with no vis check, 2 aim with vis check
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
            headshot_dist: 150.0 * 40.0,
            skynade_dist: 150.0 * 40.0,
            smooth: 200.0,
            skynade_smooth: 200.0 * 0.6667,
            recoil_smooth_x: 60.0,
            recoil_smooth_y: 60.0,
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
    weapon_id: i32,
    bullet_speed: f32,
    bullet_gravity: f32,
    weapon_zoom_fov: f32,
    weapon_mod_bitfield: i32,
    weapon_grenade: bool,
    weapon_headshot: bool,
    max_fov: f32,
    target_score_max: f32,
    aimentity: u64,
    tmp_aimentity: u64,
    locked_aimentity: u64,
    game_fps: f32,
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
            max_fov: 10.0,
            target_score_max: 0.0,
            aimentity: 0,
            tmp_aimentity: 0,
            locked_aimentity: 0,
            game_fps: 75.0,
        }
    }
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

    pub fn is_locked(&self) -> bool {
        self.lock
    }

    pub fn is_triggerbot_ready(&self) -> bool {
        self.triggerbot_ready
    }

    pub fn get_max_fov(&self) -> f32 {
        self.max_fov
    }

    pub fn get_held_id(&self) -> i32 {
        self.held_id
    }

    pub fn update_held_id(&mut self, held_id: i32) {
        self.held_id = held_id;
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
        self.aimentity
    }

    pub fn target_distance_check(&self, distance: f32) -> bool {
        if self.held_id == -251 {
            distance <= self.settings.skynade_dist
        } else {
            distance <= self.settings.aim_dist
        }
    }

    pub fn start_select_target(&mut self) {
        self.target_score_max = (50.0 * 50.0) * 100.0 + (self.settings.aim_dist * 0.025) * 10.0;
        self.tmp_aimentity = 0;
    }

    pub fn add_select_target(&mut self, fov: f32, distance: f32, visible: bool, target_ptr: u64) {
        const VIS_WEIGHTS: f32 = 12.5;
        let score = (fov * fov) * 100.0
            + (distance * 0.025) * 10.0
            + (if visible { 0.0 } else { VIS_WEIGHTS });
        /*
         fov:dist:score
          1  10m  100
          2  40m  400
          3  90m  900
          4  160m 1600
        */
        if score < self.target_score_max {
            self.target_score_max = score;
            self.tmp_aimentity = target_ptr;
        }

        // vis check for shooting current aim entity
        if self.settings.aim_mode == 2 && self.held_id != -251 && self.aimentity == target_ptr {
            if !visible {
                // turn on safety
                self.gun_safety = true;
            } else {
                self.gun_safety = false;
            }
        }
    }

    pub fn finish_select_target(&mut self) {
        // set current aim entity
        if self.lock {
            // locked target
            self.aimentity = self.locked_aimentity;
        } else {
            // or new target
            self.aimentity = self.tmp_aimentity;
        }

        // disable aimbot safety if vis check is turned off
        if self.settings.aim_mode == 1 && self.held_id != -251 {
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
        if self.held_id == -251 {
            // Set weapon type
            self.weapon_grenade = true;
            self.weapon_headshot = false;

            // Update grenade safety state
            if (!self.settings.auto_nade_aim && self.zoom_state == 0)
                || (self.settings.auto_nade_aim && self.zoom_state > 0)
            {
                self.gun_safety = true;
            } else {
                self.gun_safety = false;
            }

            // Update aimbot fov for grenade
            self.max_fov = 999.9;
        } else {
            // Set weapon type
            self.weapon_grenade = false;
            self.weapon_headshot = {
                match self.weapon_id {
                    IDWEAPON_3030_REPEATER => true,
                    IDWEAPON_BOW => true,
                    IDWEAPON_CHARGE_RIFLE => true,
                    IDWEAPON_G7_SCOUT => true,
                    IDWEAPON_KRABER => true,
                    IDWEAPON_LONGBOW => true,
                    IDWEAPON_SENTINEL => true,
                    IDWEAPON_TRIPLE_TAKE => true,
                    IDWEAPON_WINGMAN => true,
                    _ => false,
                }
            };

            // Update aimbot fov
            if self.zoom_state > 0 {
                self.max_fov = self.settings.ads_fov;
            } else {
                self.max_fov = self.settings.non_ads_fov;
            }
        }

        // Update aiming state
        if self.aim_key_state > 0 {
            self.aiming = true;
        } else if self.settings.gamepad && (self.attack_state > 0 || self.zoom_state > 0) {
            self.aiming = true;
        } else {
            self.aiming = false;
        }

        // Update triggerbot state
        if self.settings.auto_shoot && self.triggerbot_key_state > 0 {
            self.triggerbot_ready = true;
        } else {
            self.triggerbot_ready = false;
        }
    }
}

#[no_mangle]
pub fn aimbot_new() -> Aimbot {
    Aimbot::new()
}

#[no_mangle]
pub fn aimbot_get_settings(aimbot: &Aimbot) -> AimbotSettings {
    aimbot.get_settings()
}

#[no_mangle]
pub fn aimbot_settings(aimbot: &mut Aimbot, settings: &AimbotSettings) {
    aimbot.settings(settings.clone())
}

#[no_mangle]
pub fn aimbot_is_aiming(aimbot: &Aimbot) -> bool {
    aimbot.is_aiming()
}

#[no_mangle]
pub fn aimbot_is_grenade(aimbot: &Aimbot) -> bool {
    aimbot.is_grenade()
}

#[no_mangle]
pub fn aimbot_is_headshot(aimbot: &Aimbot) -> bool {
    aimbot.is_headshot()
}

#[no_mangle]
pub fn aimbot_is_locked(aimbot: &Aimbot) -> bool {
    aimbot.is_locked()
}

#[no_mangle]
pub fn aimbot_is_triggerbot_ready(aimbot: &Aimbot) -> bool {
    aimbot.is_triggerbot_ready()
}

#[no_mangle]
pub fn aimbot_get_max_fov(aimbot: &Aimbot) -> f32 {
    aimbot.get_max_fov()
}

#[no_mangle]
pub fn aimbot_get_held_id(aimbot: &Aimbot) -> i32 {
    aimbot.get_held_id()
}

#[no_mangle]
pub fn aimbot_update_held_id(aimbot: &mut Aimbot, held_id: i32) {
    aimbot.update_held_id(held_id)
}

#[no_mangle]
pub fn aimbot_get_weapon_id(aimbot: &Aimbot) -> i32 {
    aimbot.get_weapon_id()
}

#[no_mangle]
pub fn aimbot_update_weapon_info(
    aimbot: &mut Aimbot,
    weapon_id: i32,
    bullet_speed: f32,
    bullet_gravity: f32,
    weapon_zoom_fov: f32,
    weapon_mod_bitfield: i32,
) {
    aimbot.update_weapon_info(
        weapon_id,
        bullet_speed,
        bullet_gravity,
        weapon_zoom_fov,
        weapon_mod_bitfield,
    )
}

#[no_mangle]
pub fn aimbot_get_gun_safety(aimbot: &Aimbot) -> bool {
    aimbot.get_gun_safety()
}

#[no_mangle]
pub fn aimbot_set_gun_safety(aimbot: &mut Aimbot, gun_safety: bool) {
    aimbot.set_gun_safety(gun_safety)
}

#[no_mangle]
pub fn aimbot_get_aim_key_state(aimbot: &Aimbot) -> i32 {
    aimbot.get_aim_key_state()
}

#[no_mangle]
pub fn aimbot_update_aim_key_state(aimbot: &mut Aimbot, aim_key_state: i32) {
    aimbot.update_aim_key_state(aim_key_state)
}

#[no_mangle]
pub fn aimbot_update_triggerbot_key_state(aimbot: &mut Aimbot, triggerbot_key_state: i32) {
    aimbot.update_triggerbot_key_state(triggerbot_key_state)
}

#[no_mangle]
pub fn aimbot_update_attack_state(aimbot: &mut Aimbot, attack_state: i32) {
    aimbot.update_attack_state(attack_state)
}

#[no_mangle]
pub fn aimbot_update_zoom_state(aimbot: &mut Aimbot, zoom_state: i32) {
    aimbot.update_zoom_state(zoom_state)
}

#[no_mangle]
pub fn aimbot_get_aim_entity(aimbot: &Aimbot) -> u64 {
    aimbot.get_aim_entity()
}

#[no_mangle]
pub fn aimbot_target_distance_check(aimbot: &Aimbot, distance: f32) -> bool {
    aimbot.target_distance_check(distance)
}

#[no_mangle]
pub fn aimbot_start_select_target(aimbot: &mut Aimbot) {
    aimbot.start_select_target()
}

#[no_mangle]
pub fn aimbot_add_select_target(
    aimbot: &mut Aimbot,
    fov: f32,
    distance: f32,
    visible: bool,
    target_ptr: u64,
) {
    aimbot.add_select_target(fov, distance, visible, target_ptr)
}

#[no_mangle]
pub fn aimbot_finish_select_target(aimbot: &mut Aimbot) {
    aimbot.finish_select_target()
}

#[no_mangle]
pub fn aimbot_lock_target(aimbot: &mut Aimbot, target_ptr: u64) {
    aimbot.lock_target(target_ptr)
}

#[no_mangle]
pub fn aimbot_cancel_locking(aimbot: &mut Aimbot) {
    aimbot.cancel_locking()
}

#[no_mangle]
pub fn aimbot_update(aimbot: &mut Aimbot, game_fps: f32) {
    aimbot.game_fps = game_fps;
    aimbot.update();
}
