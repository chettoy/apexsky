use super::Pod;
use crate::apexdream::base::hash;
use apexsky::noobfstr as s;
use bitset_core::*;
use named_constants::named_constants;
use once_cell::sync::Lazy;
use std::fmt;

//----------------------------------------------------------------

/// Item rarities.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Rarity {
    Common = 0,
    Rare = 1,
    Epic = 2,
    Legendary = 3,
    Heirloom = 4,
}

impl Rarity {
    pub fn to_str<R, F: FnMut(&str) -> R>(self, mut f: F) -> R {
        static S_COMMON: Lazy<String> = Lazy::new(|| s!("Common").to_string());
        static S_RARE: Lazy<String> = Lazy::new(|| s!("Rare").to_string());
        static S_EPIC: Lazy<String> = Lazy::new(|| s!("Epic").to_string());
        static S_LEGENDARY: Lazy<String> = Lazy::new(|| s!("Legendary").to_string());
        static S_HEIRLOOM: Lazy<String> = Lazy::new(|| s!("Heirloom").to_string());
        match self {
            Rarity::Common => f(&*S_COMMON),
            Rarity::Rare => f(&*S_RARE),
            Rarity::Epic => f(&*S_EPIC),
            Rarity::Legendary => f(&*S_LEGENDARY),
            Rarity::Heirloom => f(&*S_HEIRLOOM),
        }
    }
}

impl fmt::Display for Rarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_str(|string| f.write_str(string))
    }
}

//----------------------------------------------------------------

#[named_constants]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash, Reflection)]
#[repr(u32)]
pub enum WeaponName {
    R301 = hash("mp_weapon_rspn101"),
    SENTINEL = hash("mp_weapon_sentinel"),
    BOCEK = hash("mp_weapon_bow"),
    TITAN_SWORD = hash("mp_weapon_titan_sword"),
    MELEE_SURVIVAL = hash("mp_weapon_melee_survival"),
    ALTERNATOR = hash("mp_weapon_alternator_smg"),
    RE45 = hash("mp_weapon_autopistol"),
    CHARGE_RIFLE = hash("mp_weapon_defender"),
    DEVOTION = hash("mp_weapon_esaw"),
    LONGBOW = hash("mp_weapon_dmr"),
    HAVOC = hash("mp_weapon_energy_ar"),
    EVA8_AUTO = hash("mp_weapon_shotgun"),
    FLATLINE = hash("mp_weapon_vinson"),
    G7_SCOUT = hash("mp_weapon_g2"),
    HEMLOK = hash("mp_weapon_hemlok"),
    KRABER = hash("mp_weapon_sniper"),
    LSTAR = hash("mp_weapon_lstar"),
    MASTIFF = hash("mp_weapon_mastiff"),
    MOZAMBIQUE = hash("mp_weapon_shotgun_pistol"),
    PROWLER = hash("mp_weapon_pdw"),
    PEACEKEEPER = hash("mp_weapon_energy_shotgun"),
    R99 = hash("mp_weapon_r97"),
    P2020 = hash("mp_weapon_semipistol"),
    SPITFIRE = hash("mp_weapon_lmg"),
    TRIPLE_TAKE = hash("mp_weapon_doubletake"),
    WINGMAN = hash("mp_weapon_wingman"),
    VOLT = hash("mp_weapon_volt_smg"),
    REPEATER = hash("mp_weapon_3030"),
    RAMPAGE = hash("mp_weapon_dragon_lmg"),
    CAR = hash("mp_weapon_car"),
    NEMESIS = hash("mp_weapon_nemesis"),
    THROWING_KNIFE = hash("mp_weapon_throwingknife"),

    EMPLACED_MINIGUN = hash("mp_weapon_mounted_turret_weapon"),
    CLUSTER_BOMB_LAUNCHER = hash("mp_weapon_cluster_bomb_launcher"),
    VANTAGE_SNIPER = hash("mp_ability_sniper_ult"),

    CONSUMABLE = hash("mp_ability_consumable"),

    THERMITE_GRENADE = hash("mp_weapon_thermite_grenade"),
    FRAG_GRENADE = hash("mp_weapon_frag_grenade"),
    ARC_STAR = hash("mp_weapon_grenade_emp"),
}

//----------------------------------------------------------------

#[named_constants]
#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
#[repr(u32)]
pub enum ModelName {
    EMPTY_MODEL = hash("mdl/dev/empty_model.rmdl"),
    DUMMIE_GENERIC = hash("mdl/humans/class/medium/pilot_medium_generic.rmdl"),
    DUMMIE_COMBAT = hash("mdl/humans/class/medium/combat_dummie_medium.rmdl"),
    DUMMIE_TRAINING = hash("mdl/humans/class/medium/dummie_medium_training.rmdl"),
    DUMMY = hash("mdl/humans/class/medium/dummy_v20_base_w.rmdl"),

    PROWLER = hash("mdl/creatures/prowler/prowler_apex.rmdl"),
    LOOT_TICK = hash("mdl/robots/drone_frag/drone_frag_loot.rmdl"), // normal: skin=0, gold: skin=1
    LOOT_BIN = hash("mdl/props/loot_bin/loot_bin_01_loba.rmdl"),    // normal: skin=0, extra: skin=4
    LOOT_DRONE = hash("mdl/props/loot_drone/loot_drone.rmdl"),
    LOOT_SPHERE = hash("mdl/props/loot_sphere/loot_sphere.rmdl"),
    MARVIN_BASE = hash("mdl/robots/marvin/marvin_base_v2.rmdl"), // normal: skin=0, gold arm: skin=1
    GAS_TANK = hash("mdl/props/caustic_gas_tank/caustic_gas_tank.rmdl"),
    BUBBLESHIELD = hash("mdl/props/gibraltar_bubbleshield/gibraltar_bubbleshield.rmdl"),
    JUMP_PAD = hash("mdl/props/octane_jump_pad/octane_jump_pad.rmdl"),
    TOTEM = hash("mdl/props/revenant_totem/revenant_totem.rmdl"),
    ELECTRIC_FENCE = hash("mdl/props/wattson_electric_fence/wattson_electric_fence.rmdl"),
    TROPHY_SYSTEM = hash("mdl/props/wattson_trophy_system/wattson_trophy_system.rmdl"),
    ZIPLINE = hash("mdl/props/pathfinder_zipline/pathfinder_zipline.rmdl"),
    ROCKETS_PROJECTILE =
        hash("mdl/weapons_r5/misc_bangalore_rockets/bangalore_rockets_projectile.rmdl"),
    HOLO_STAND = hash("mdl/fx/loba_staff_holo_stand.rmdl"),
    COVER_WALL = hash("mdl/props/rampart_cover_wall/rampart_cover_wall.rmdl"),
    PARIAH_DRONE = hash("mdl/props/pariah_drone_cluster/pariah_drone_cluster.rmdl"),
    SHOOTING_RANGE_TARGET = hash("mdl/barriers/shooting_range_target_02.rmdl"),
    SHOOTING_RANGE_CUTOUT = hash("mdl/barriers/shooting_range_target_01_animated.rmdl"),

    LIGHT_ROUNDS = hash("mdl/weapons_r5/loot/_master/w_loot_wep_ammo_sc.rmdl"),
    ENERGY_AMMO = hash("mdl/weapons_r5/loot/_master/w_loot_wep_ammo_nrg.rmdl"),
    SHOTGUN_AMMO = hash("mdl/weapons_r5/loot/_master/w_loot_wep_ammo_shg.rmdl"),
    HEAVY_ROUNDS = hash("mdl/weapons_r5/loot/_master/w_loot_wep_ammo_hc.rmdl"),
    SNIPER_AMMO = hash("mdl/weapons_r5/loot/_master/w_loot_wep_ammo_sniper.rmdl"),
    ARROWS = hash("mdl/weapons_r5/loot/_master/w_loot_wep_arrows_mn.rmdl"),
    ARROWS_SINGLE = hash("mdl/weapons_r5/loot/_master/w_loot_wep_arrow_single.rmdl"),

    ULTIMATE_ACCELERANT = hash("mdl/weapons_r5/loot/w_loot_wep_iso_ultimate_accelerant.rmdl"),
    PHOENIX_KIT = hash("mdl/weapons_r5/loot/w_loot_wep_iso_phoenix_kit_v1.rmdl"),
    MED_KIT = hash("mdl/weapons_r5/loot/w_loot_wep_iso_health_main_large.rmdl"),
    SYRINGE = hash("mdl/weapons_r5/loot/w_loot_wep_iso_health_main_small.rmdl"),
    SHIELD_BATTERY = hash("mdl/weapons_r5/loot/w_loot_wep_iso_shield_battery_large.rmdl"),
    SHIELD_CELL = hash("mdl/weapons_r5/loot/w_loot_wep_iso_shield_battery_small.rmdl"),

    THERMITE_GRENADE = hash("mdl/weapons/grenades/w_thermite_grenade.rmdl"),
    FRAG_GRENADE = hash("mdl/weapons/grenades/w_loot_m20_f_grenade_projectile.rmdl"),
    ARC_STAR = hash("mdl/weapons_r5/loot/w_loot_wep_iso_shuriken.rmdl"),

    UPGRADE_HEAD = hash("mdl/weapons_r5/loot/_master/w_loot_cha_shield_upgrade_head.rmdl"),
    UPGRADE_BODY = hash("mdl/weapons_r5/loot/_master/w_loot_cha_shield_upgrade_body.rmdl"),
    KNOCKDOWN_SHIELD = hash("mdl/weapons_r5/loot/w_loot_wep_iso_shield_down_v1.rmdl"),
    BACKPACK_LIGHT = hash("mdl/humans_r5/loot/w_loot_char_backpack_light.rmdl"),
    BACKPACK_MEDIUM = hash("mdl/humans_r5/loot/w_loot_char_backpack_medium.rmdl"),
    BACKPACK_HEAVY = hash("mdl/humans_r5/loot/w_loot_char_backpack_heavy.rmdl"),

    HCOG_CLASSIC = hash("mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_cq_hcog_r1.rmdl"),
    HCOG_BRUISER = hash("mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_cq_hcog_r2.rmdl"),
    HCOG_RANGER = hash("mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_rng_hcog_acgs.rmdl"),
    DIGITAL_THREAT = hash("mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_cq_threat.rmdl"),
    HOLO = hash("mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_cq_holo_var.rmdl"),
    VARIABLE_HOLO = hash("mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_cq_holo_var_2x.rmdl"),
    VARIABLE_AOG = hash("mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_rng_aog_var_r1.rmdl"),
    SNIPER = hash("mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_sni_dcom.rmdl"),
    VARIABLE_SNIPER = hash("mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_sni_var_talon.rmdl"),
    DIGITAL_SNIPER_THREAT =
        hash("mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_sni_threat_wyeon.rmdl"),

    BARREL_STABILIZER = hash("mdl/weapons_r5/loot/_master/w_loot_wep_mods_suppr_v2b.rmdl"),
    LASER_SIGHT = hash("mdl/weapons_r5/loot/_master/w_loot_wep_mods_lasersight_v1.rmdl"),
    LIGHT_MAGAZINE = hash("mdl/weapons_r5/loot/_master/w_loot_wep_mods_mag_v1b.rmdl"),
    HEAVY_MAGAZINE = hash("mdl/weapons_r5/loot/_master/w_loot_wep_mods_mag_v2b.rmdl"),
    ENERGY_MAGAZINE = hash("mdl/weapons_r5/loot/_master/w_loot_wep_mods_mag_energy_v1.rmdl"),
    SNIPER_MAGAZINE = hash("mdl/weapons_r5/loot/_master/w_loot_wep_mods_mag_sniper_v1.rmdl"),
    SHOTGUN_BOLT = hash("mdl/weapons_r5/loot/_master/w_loot_wep_mods_mag_v3b.rmdl"),
    STANDARD_STOCK = hash("mdl/weapons_r5/loot/w_loot_wep_iso_stock_folded_regular.rmdl"),
    SNIPER_STOCK = hash("mdl/weapons_r5/loot/w_loot_wep_iso_stock_folded_sniper.rmdl"),

    MODS_CHIP = hash("mdl/weapons_r5/loot/_master/w_loot_wep_mods_chip.rmdl"),

    TREASURE_BOX = hash("mdl/weapons_r5/misc_pve/s5_treasure_box/w_s5_treasure_box.rmdl"),
    KEYCARD_V1 = hash("mdl/weapons_r5/loot/_master/w_loot_msc_keycard_v1.rmdl"),
    VOID_RING = hash("mdl/props/void_ring/loot_void_ring.rmdl"),
    BEACON_CAPSULE = hash("mdl/weapons/beacon_capsule_01/beacon_capsule_01.rmdl"),
    GOLDEN_TICKET = hash("mdl/props/golden_ticket.rmdl"),
    BANNER_CRAFTING =
        hash("mdl/props/ultimate_accelerant/ultimate_accelerant_banner_crafting.rmdl"),
}

//----------------------------------------------------------------

#[named_constants]
#[derive(Pod, Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(i32)]
pub enum WeapState {
    READY = 0,
    END_SWAP = 1,
    START_SWAP = 2,
    WINDUP_ANIM = 3,
    HAVOC_WINDUP = 5,
    CHARGE_RIFLE_TRACKING = 7,
    RUNNING = 8,
    FIRING = 9,
    RELOADING = 10,
    IN_RELOAD_ANIMATION_FOR_LONG_WINDUP = 13,
}

//----------------------------------------------------------------

pub const AMMO_COLOR: [f32; 3] = [0.1, 0.1, 0.1];
pub const WEAPON_COLOR: [f32; 3] = [0.55, 0.55, 0.5];
pub const COMMON_COLORS: [[f32; 3]; 4] = [
    [0.75294125, 0.75294125, 0.75294125],
    [0.28235295, 0.28235295, 0.28235295],
    [0.28235295, 0.28235295, 0.28235295],
    [0.5019608, 0.5019608, 0.5019608],
];
pub const RARE_COLORS: [[f32; 3]; 4] = [
    [0.11764707, 0.5647059, 1.0],
    [0.0, 0.24313727, 0.46274513],
    [0.0, 0.29411766, 0.73333335],
    [0.15686275, 0.7686275, 1.0],
];
pub const EPIC_COLORS: [[f32; 3]; 4] = [
    [0.4901961, 0.0, 1.0],
    [0.6156863, 0.5176471, 0.9490197],
    [0.8862746, 0.6862745, 1.0],
    [0.6627451, 0.1764706, 0.9725491],
];
pub const LEGENDARY_COLORS: [[f32; 3]; 4] = [
    [1.0, 0.80392164, 0.23529413],
    [0.9803922, 0.95294124, 0.29803923],
    [1.0, 0.68235296, 0.2509804],
    [0.9803922, 0.8313726, 0.3137255],
];
pub const HEIRLOOM_COLORS: [[f32; 3]; 4] = [
    [1.0, 0.30588236, 0.1137255],
    [1.0, 0.427451, 0.6431373],
    [1.0, 0.19607845, 0.15686275],
    [0.8117648, 0.27450982, 0.2901961],
];

//----------------------------------------------------------------

#[named_constants]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd, Reflection)]
#[repr(u8)]
pub enum ItemId {
    None,

    // Weapon
    R301,
    Sentinel,
    Bocek,
    Alternator,
    RE45,
    ChargeRifle,
    Devotion,
    Longbow,
    Havoc,
    EVA8Auto,
    Flatline,
    Hemlok,
    Kraber,
    G7Scout,
    LStar,
    Mastiff,
    Mozambique,
    Prowler,
    PK,
    R99,
    P2020,
    Spitfire,
    TripleTake,
    Wingman,
    Volt,
    Repeater,
    Rampage,
    CAR,

    // Ammo
    LightRounds,
    EnergyAmmo,
    ShotgunShells,
    HeavyRounds,
    SniperAmmo,
    Arrows,

    // Meds
    UltAccel,
    PhoenixKit,
    MedKit,
    Syringe,
    Battery,
    ShieldCell,

    // Equipment
    HelmetLv1,
    HelmetLv2,
    HelmetLv3,
    HelmetLv4,
    BodyArmorLv1,
    BodyArmorLv2,
    BodyArmorLv3,
    BodyArmorLv4,
    EvoShieldLv0,
    EvoShieldLv1,
    EvoShieldLv2,
    EvoShieldLv3,
    EvoShieldLv4,
    KnockdownShieldLv1,
    KnockdownShieldLv2,
    KnockdownShieldLv3,
    KnockdownShieldLv4,
    BackpackLv1,
    BackpackLv2,
    BackpackLv3,
    BackpackLv4,

    // Grenades
    Thermite,
    FragGrenade,
    ArcStar,

    // Sights
    HcogClassic,
    HcogBruiser,
    HcogRanger,
    Holo,
    VariableHolo,
    VariableAOG,
    DigiThreat,
    Sniper,
    VariableSniper,
    DigiSniperThreat,

    // Attachments
    BarrelStabilizerLv1,
    BarrelStabilizerLv2,
    BarrelStabilizerLv3,
    BarrelStabilizerLv4,
    LaserSightLv1,
    LaserSightLv2,
    LaserSightLv3,
    LaserSightLv4,
    LightMagazineLv1,
    LightMagazineLv2,
    LightMagazineLv3,
    LightMagazineLv4,
    HeavyMagazineLv1,
    HeavyMagazineLv2,
    HeavyMagazineLv3,
    HeavyMagazineLv4,
    EnergyMagazineLv1,
    EnergyMagazineLv2,
    EnergyMagazineLv3,
    EnergyMagazineLv4,
    SniperMagazineLv1,
    SniperMagazineLv2,
    SniperMagazineLv3,
    SniperMagazineLv4,
    ShotgunBoltLv1,
    ShotgunBoltLv2,
    ShotgunBoltLv3,
    ShotgunBoltLv4,
    StandardStockLv1,
    StandardStockLv2,
    StandardStockLv3,
    SniperStockLv1,
    SniperStockLv2,
    SniperStockLv3,

    // Hop-ups
    EpicHopUp0,
    EpicHopUp3,
    LegendaryHopUp0,
    LegendaryHopUp4,

    // Misc
    Keycard,
    TreasurePack,
    HeatShield,
    MobileRespawn,
    MrvnArm,
    GoldenTicket,
    BannerCrafting,
}

impl fmt::Display for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            ItemId::None => String::from(s!("None")),
            ItemId::R301 => String::from(s!("R301")),
            ItemId::Sentinel => String::from(s!("Sentinel")),
            ItemId::Bocek => String::from(s!("Bocek")),
            ItemId::Alternator => String::from(s!("Alternator")),
            ItemId::RE45 => String::from(s!("RE45")),
            ItemId::ChargeRifle => String::from(s!("ChargeRifle")),
            ItemId::Devotion => String::from(s!("Devotion")),
            ItemId::Longbow => String::from(s!("Longbow")),
            ItemId::Havoc => String::from(s!("Havoc")),
            ItemId::EVA8Auto => String::from(s!("EVA8Auto")),
            ItemId::Flatline => String::from(s!("Flatline")),
            ItemId::Hemlok => String::from(s!("Hemlok")),
            ItemId::Kraber => String::from(s!("Kraber")),
            ItemId::G7Scout => String::from(s!("G7Scout")),
            ItemId::LStar => String::from(s!("LStar")),
            ItemId::Mastiff => String::from(s!("Mastiff")),
            ItemId::Mozambique => String::from(s!("Mozambique")),
            ItemId::Prowler => String::from(s!("Prowler")),
            ItemId::PK => String::from(s!("PK")),
            ItemId::R99 => String::from(s!("R99")),
            ItemId::P2020 => String::from(s!("P2020")),
            ItemId::Spitfire => String::from(s!("Spitfire")),
            ItemId::TripleTake => String::from(s!("TripleTake")),
            ItemId::Wingman => String::from(s!("Wingman")),
            ItemId::Volt => String::from(s!("Volt")),
            ItemId::Repeater => String::from(s!("Repeater")),
            ItemId::Rampage => String::from(s!("Rampage")),
            ItemId::CAR => String::from(s!("CAR")),
            ItemId::LightRounds => String::from(s!("LightRounds")),
            ItemId::EnergyAmmo => String::from(s!("EnergyAmmo")),
            ItemId::ShotgunShells => String::from(s!("ShotgunShells")),
            ItemId::HeavyRounds => String::from(s!("HeavyRounds")),
            ItemId::SniperAmmo => String::from(s!("SniperAmmo")),
            ItemId::Arrows => String::from(s!("Arrows")),
            ItemId::UltAccel => String::from(s!("UltAccel")),
            ItemId::PhoenixKit => String::from(s!("PhoenixKit")),
            ItemId::MedKit => String::from(s!("MedKit")),
            ItemId::Syringe => String::from(s!("Syringe")),
            ItemId::Battery => String::from(s!("Battery")),
            ItemId::ShieldCell => String::from(s!("ShieldCell")),
            ItemId::HelmetLv1 => String::from(s!("HelmetLv1")),
            ItemId::HelmetLv2 => String::from(s!("HelmetLv2")),
            ItemId::HelmetLv3 => String::from(s!("HelmetLv3")),
            ItemId::HelmetLv4 => String::from(s!("HelmetLv4")),
            ItemId::BodyArmorLv1 => String::from(s!("BodyArmorLv1")),
            ItemId::BodyArmorLv2 => String::from(s!("BodyArmorLv2")),
            ItemId::BodyArmorLv3 => String::from(s!("BodyArmorLv3")),
            ItemId::BodyArmorLv4 => String::from(s!("BodyArmorLv4")),
            ItemId::EvoShieldLv0 => String::from(s!("EvoShieldLv0")),
            ItemId::EvoShieldLv1 => String::from(s!("EvoShieldLv1")),
            ItemId::EvoShieldLv2 => String::from(s!("EvoShieldLv2")),
            ItemId::EvoShieldLv3 => String::from(s!("EvoShieldLv3")),
            ItemId::EvoShieldLv4 => String::from(s!("EvoShieldLv4")),
            ItemId::KnockdownShieldLv1 => String::from(s!("KnockdownShieldLv1")),
            ItemId::KnockdownShieldLv2 => String::from(s!("KnockdownShieldLv2")),
            ItemId::KnockdownShieldLv3 => String::from(s!("KnockdownShieldLv3")),
            ItemId::KnockdownShieldLv4 => String::from(s!("KnockdownShieldLv4")),
            ItemId::BackpackLv1 => String::from(s!("BackpackLv1")),
            ItemId::BackpackLv2 => String::from(s!("BackpackLv2")),
            ItemId::BackpackLv3 => String::from(s!("BackpackLv3")),
            ItemId::BackpackLv4 => String::from(s!("BackpackLv4")),
            ItemId::Thermite => String::from(s!("Thermite")),
            ItemId::FragGrenade => String::from(s!("FragGrenade")),
            ItemId::ArcStar => String::from(s!("ArcStar")),
            ItemId::HcogClassic => String::from(s!("HcogClassic")),
            ItemId::HcogBruiser => String::from(s!("HcogBruiser")),
            ItemId::HcogRanger => String::from(s!("HcogRanger")),
            ItemId::Holo => String::from(s!("Holo")),
            ItemId::VariableHolo => String::from(s!("VariableHolo")),
            ItemId::VariableAOG => String::from(s!("VariableAOG")),
            ItemId::DigiThreat => String::from(s!("DigiThreat")),
            ItemId::Sniper => String::from(s!("Sniper")),
            ItemId::VariableSniper => String::from(s!("VariableSniper")),
            ItemId::DigiSniperThreat => String::from(s!("DigiSniperThreat")),
            ItemId::BarrelStabilizerLv1 => String::from(s!("BarrelStabilizerLv1")),
            ItemId::BarrelStabilizerLv2 => String::from(s!("BarrelStabilizerLv2")),
            ItemId::BarrelStabilizerLv3 => String::from(s!("BarrelStabilizerLv3")),
            ItemId::BarrelStabilizerLv4 => String::from(s!("BarrelStabilizerLv4")),
            ItemId::LaserSightLv1 => String::from(s!("LaserSightLv1")),
            ItemId::LaserSightLv2 => String::from(s!("LaserSightLv2")),
            ItemId::LaserSightLv3 => String::from(s!("LaserSightLv3")),
            ItemId::LaserSightLv4 => String::from(s!("LaserSightLv4")),
            ItemId::LightMagazineLv1 => String::from(s!("LightMagazineLv1")),
            ItemId::LightMagazineLv2 => String::from(s!("LightMagazineLv2")),
            ItemId::LightMagazineLv3 => String::from(s!("LightMagazineLv3")),
            ItemId::LightMagazineLv4 => String::from(s!("LightMagazineLv4")),
            ItemId::HeavyMagazineLv1 => String::from(s!("HeavyMagazineLv1")),
            ItemId::HeavyMagazineLv2 => String::from(s!("HeavyMagazineLv2")),
            ItemId::HeavyMagazineLv3 => String::from(s!("HeavyMagazineLv3")),
            ItemId::HeavyMagazineLv4 => String::from(s!("HeavyMagazineLv4")),
            ItemId::EnergyMagazineLv1 => String::from(s!("EnergyMagazineLv1")),
            ItemId::EnergyMagazineLv2 => String::from(s!("EnergyMagazineLv2")),
            ItemId::EnergyMagazineLv3 => String::from(s!("EnergyMagazineLv3")),
            ItemId::EnergyMagazineLv4 => String::from(s!("EnergyMagazineLv4")),
            ItemId::SniperMagazineLv1 => String::from(s!("SniperMagazineLv1")),
            ItemId::SniperMagazineLv2 => String::from(s!("SniperMagazineLv2")),
            ItemId::SniperMagazineLv3 => String::from(s!("SniperMagazineLv3")),
            ItemId::SniperMagazineLv4 => String::from(s!("SniperMagazineLv4")),
            ItemId::ShotgunBoltLv1 => String::from(s!("ShotgunBoltLv1")),
            ItemId::ShotgunBoltLv2 => String::from(s!("ShotgunBoltLv2")),
            ItemId::ShotgunBoltLv3 => String::from(s!("ShotgunBoltLv3")),
            ItemId::ShotgunBoltLv4 => String::from(s!("ShotgunBoltLv4")),
            ItemId::StandardStockLv1 => String::from(s!("StandardStockLv1")),
            ItemId::StandardStockLv2 => String::from(s!("StandardStockLv2")),
            ItemId::StandardStockLv3 => String::from(s!("StandardStockLv3")),
            ItemId::SniperStockLv1 => String::from(s!("SniperStockLv1")),
            ItemId::SniperStockLv2 => String::from(s!("SniperStockLv2")),
            ItemId::SniperStockLv3 => String::from(s!("SniperStockLv3")),
            ItemId::EpicHopUp0 => String::from(s!("EpicHopUp0")),
            ItemId::EpicHopUp3 => String::from(s!("EpicHopUp3")),
            ItemId::LegendaryHopUp0 => String::from(s!("LegendaryHopUp0")),
            ItemId::LegendaryHopUp4 => String::from(s!("LegendaryHopUp4")),
            ItemId::Keycard => String::from(s!("Keycard")),
            ItemId::TreasurePack => String::from(s!("TreasurePack")),
            ItemId::HeatShield => String::from(s!("HeatShield")),
            ItemId::MobileRespawn => String::from(s!("MobileRespawn")),
            ItemId::MrvnArm => String::from(s!("MrvnArm")),
            ItemId(id) => id.to_string(),
        };
        f.write_str(&name)
    }
}

//----------------------------------------------------------------

pub type ItemSet = [u8; 16];

pub fn item_set_to_string(set: &ItemSet) -> String {
    let mut s = String::new();
    for i in 0..set.bit_len() {
        s.push_str(if set.bit_test(i) { "1" } else { "0" });
    }
    return s;
}

pub fn downgrade_mask(item: ItemId) -> ItemSet {
    const INIT: [u8; 16] = [0u8; 16];
    use ItemId as KI;
    match item {
        KI::BarrelStabilizerLv1 => bitset!(INIT; KI::BarrelStabilizerLv1.0),
        KI::BarrelStabilizerLv2 => {
            bitset!(INIT; KI::BarrelStabilizerLv1.0, KI::BarrelStabilizerLv2.0)
        }
        KI::BarrelStabilizerLv3 => {
            bitset!(INIT; KI::BarrelStabilizerLv1.0, KI::BarrelStabilizerLv2.0, KI::BarrelStabilizerLv3.0)
        }
        KI::BarrelStabilizerLv4 => {
            bitset!(INIT; KI::BarrelStabilizerLv1.0, KI::BarrelStabilizerLv2.0, KI::BarrelStabilizerLv3.0)
        }
        KI::LaserSightLv1 => bitset!(INIT; KI::LaserSightLv1.0),
        KI::LaserSightLv2 => bitset!(INIT; KI::LaserSightLv1.0, KI::LaserSightLv2.0),
        KI::LaserSightLv3 => {
            bitset!(INIT; KI::LaserSightLv1.0, KI::LaserSightLv2.0, KI::LaserSightLv3.0)
        }
        KI::LaserSightLv4 => {
            bitset!(INIT; KI::LaserSightLv1.0, KI::LaserSightLv2.0, KI::LaserSightLv3.0)
        }
        KI::LightMagazineLv1 => bitset!(INIT; KI::LightMagazineLv1.0),
        KI::LightMagazineLv2 => bitset!(INIT; KI::LightMagazineLv1.0, KI::LightMagazineLv2.0),
        KI::LightMagazineLv3 => {
            bitset!(INIT; KI::LightMagazineLv1.0, KI::LightMagazineLv2.0, KI::LightMagazineLv3.0)
        }
        KI::LightMagazineLv4 => {
            bitset!(INIT; KI::LightMagazineLv1.0, KI::LightMagazineLv2.0, KI::LightMagazineLv3.0)
        }
        KI::HeavyMagazineLv1 => bitset!(INIT; KI::HeavyMagazineLv1.0),
        KI::HeavyMagazineLv2 => bitset!(INIT; KI::HeavyMagazineLv1.0, KI::HeavyMagazineLv2.0),
        KI::HeavyMagazineLv3 => {
            bitset!(INIT; KI::HeavyMagazineLv1.0, KI::HeavyMagazineLv2.0, KI::HeavyMagazineLv3.0)
        }
        KI::HeavyMagazineLv4 => {
            bitset!(INIT; KI::HeavyMagazineLv1.0, KI::HeavyMagazineLv2.0, KI::HeavyMagazineLv3.0)
        }
        KI::EnergyMagazineLv1 => bitset!(INIT; KI::EnergyMagazineLv1.0),
        KI::EnergyMagazineLv2 => bitset!(INIT; KI::EnergyMagazineLv1.0, KI::EnergyMagazineLv2.0),
        KI::EnergyMagazineLv3 => {
            bitset!(INIT; KI::EnergyMagazineLv1.0, KI::EnergyMagazineLv2.0, KI::EnergyMagazineLv3.0)
        }
        KI::EnergyMagazineLv4 => {
            bitset!(INIT; KI::EnergyMagazineLv1.0, KI::EnergyMagazineLv2.0, KI::EnergyMagazineLv3.0)
        }
        KI::SniperMagazineLv1 => bitset!(INIT; KI::SniperMagazineLv1.0),
        KI::SniperMagazineLv2 => bitset!(INIT; KI::SniperMagazineLv1.0, KI::SniperMagazineLv2.0),
        KI::SniperMagazineLv3 => {
            bitset!(INIT; KI::SniperMagazineLv1.0, KI::SniperMagazineLv2.0, KI::SniperMagazineLv3.0)
        }
        KI::SniperMagazineLv4 => {
            bitset!(INIT; KI::SniperMagazineLv1.0, KI::SniperMagazineLv2.0, KI::SniperMagazineLv3.0)
        }
        KI::ShotgunBoltLv1 => bitset!(INIT; KI::ShotgunBoltLv1.0),
        KI::ShotgunBoltLv2 => bitset!(INIT; KI::ShotgunBoltLv1.0, KI::ShotgunBoltLv2.0),
        KI::ShotgunBoltLv3 => {
            bitset!(INIT; KI::ShotgunBoltLv1.0, KI::ShotgunBoltLv2.0, KI::ShotgunBoltLv3.0)
        }
        KI::ShotgunBoltLv4 => {
            bitset!(INIT; KI::ShotgunBoltLv1.0, KI::ShotgunBoltLv2.0, KI::ShotgunBoltLv3.0)
        }
        KI::StandardStockLv1 => bitset!(INIT; KI::StandardStockLv1.0),
        KI::StandardStockLv2 => bitset!(INIT; KI::StandardStockLv1.0, KI::StandardStockLv2.0),
        KI::StandardStockLv3 => {
            bitset!(INIT; KI::StandardStockLv1.0, KI::StandardStockLv2.0, KI::StandardStockLv3.0)
        }
        KI::SniperStockLv1 => bitset!(INIT; KI::SniperStockLv1.0),
        KI::SniperStockLv2 => bitset!(INIT; KI::SniperStockLv1.0, KI::SniperStockLv2.0),
        KI::SniperStockLv3 => {
            bitset!(INIT; KI::SniperStockLv1.0, KI::SniperStockLv2.0, KI::StandardStockLv3.0)
        }

        KI::HcogClassic => bitset!(INIT; KI::Holo.0, KI::HcogClassic.0),
        KI::HcogBruiser => bitset!(INIT; KI::Holo.0, KI::HcogClassic.0, KI::HcogBruiser.0),
        KI::HcogRanger => bitset!(INIT; KI::Holo.0, KI::HcogRanger.0),
        KI::Holo => bitset!(INIT; KI::Holo.0),
        KI::VariableHolo => bitset!(INIT; KI::Holo.0, KI::VariableHolo.0),
        KI::VariableAOG => bitset!(INIT; KI::Holo.0, KI::VariableAOG.0),
        KI::DigiThreat => bitset!(INIT; KI::Holo.0, KI::DigiThreat.0),
        KI::Sniper => bitset!(INIT; KI::Sniper.0),
        KI::VariableSniper => bitset!(INIT; KI::Sniper.0, KI::VariableSniper.0),
        KI::DigiSniperThreat => bitset!(INIT; KI::Sniper.0, KI::DigiSniperThreat.0),

        _ => ItemSet::default(),
    }
}

pub fn ammo_type(weapon_name: WeaponName) -> ItemId {
    match weapon_name {
        WeaponName::R301 => ItemId::LightRounds,
        WeaponName::SENTINEL => ItemId::SniperAmmo,
        WeaponName::BOCEK => ItemId::Arrows,
        WeaponName::ALTERNATOR => ItemId::LightRounds,
        WeaponName::RE45 => ItemId::LightRounds,
        WeaponName::CHARGE_RIFLE => ItemId::SniperAmmo,
        WeaponName::DEVOTION => ItemId::EnergyAmmo,
        WeaponName::LONGBOW => ItemId::SniperAmmo,
        WeaponName::HAVOC => ItemId::EnergyAmmo,
        WeaponName::EVA8_AUTO => ItemId::ShotgunShells,
        WeaponName::FLATLINE => ItemId::HeavyRounds,
        WeaponName::G7_SCOUT => ItemId::LightRounds,
        WeaponName::HEMLOK => ItemId::HeavyRounds,
        WeaponName::KRABER => ItemId::None,
        WeaponName::LSTAR => ItemId::EnergyAmmo,
        WeaponName::MASTIFF => ItemId::None,
        WeaponName::MOZAMBIQUE => ItemId::ShotgunShells,
        WeaponName::PROWLER => ItemId::HeavyRounds,
        WeaponName::PEACEKEEPER => ItemId::ShotgunShells,
        WeaponName::R99 => ItemId::LightRounds,
        WeaponName::P2020 => ItemId::LightRounds,
        WeaponName::SPITFIRE => ItemId::LightRounds,
        WeaponName::TRIPLE_TAKE => ItemId::EnergyAmmo,
        WeaponName::WINGMAN => ItemId::SniperAmmo,
        WeaponName::VOLT => ItemId::EnergyAmmo,
        WeaponName::REPEATER => ItemId::HeavyRounds,
        WeaponName::RAMPAGE => ItemId::None,
        WeaponName::CAR => ItemId::HeavyRounds,
        WeaponName::NEMESIS => ItemId::EnergyAmmo,
        _ => ItemId::None,
    }
}

#[named_constants]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum ModName {
    ability_in_effect_regen_paused = hash("ability_in_effect_regen_paused"),
    ability_used_mod = hash("ability_used_mod"),
    alt_ammo = hash("alt_ammo"),
    altfire = hash("altfire"),
    altfire_double_tap = hash("altfire_double_tap"),
    altfire_highcal = hash("altfire_highcal"),
    ammo_type_swap = hash("ammo_type_swap"),
    amped_damage = hash("amped_damage"),
    amped_damage_alt = hash("amped_damage_alt"),
    amped_tacticals = hash("amped_tacticals"),
    arenas_regen_off = hash("arenas_regen_off"),
    arenas_regen_off_cooldown = hash("arenas_regen_off_cooldown"),
    arenas_tac_max = hash("arenas_tac_max"),
    arenas_tac_max_cooldown = hash("arenas_tac_max_cooldown"),
    arrows_shatter_dmg_lv0 = hash("arrows_shatter_dmg_lv0"),
    arrows_shatter_dmg_lv3 = hash("arrows_shatter_dmg_lv3"),
    arrows_shatter_dmg_lv5 = hash("arrows_shatter_dmg_lv5"),
    axiom_leap_ult_held = hash("axiom_leap_ult_held"),
    barrel_stabilizer_l1 = hash("barrel_stabilizer_l1"),
    barrel_stabilizer_l2 = hash("barrel_stabilizer_l2"),
    barrel_stabilizer_l3 = hash("barrel_stabilizer_l3"),
    barrel_stabilizer_l4_flash_hider = hash("barrel_stabilizer_l4_flash_hider"),
    beacon_1 = hash("beacon_1"),
    beacon_2 = hash("beacon_2"),
    beacon_3 = hash("beacon_3"),
    beacon_4 = hash("beacon_4"),
    beacon_5 = hash("beacon_5"),
    beacon_6 = hash("beacon_6"),
    blue_paintball = hash("blue_paintball"),
    breacher = hash("breacher"),
    bullets_mag_l1 = hash("bullets_mag_l1"),
    bullets_mag_l2 = hash("bullets_mag_l2"),
    bullets_mag_l3 = hash("bullets_mag_l3"),
    bullets_mag_l4 = hash("bullets_mag_l4"),
    burn_card_weapon_mod = hash("burn_card_weapon_mod"),
    burn_mod_grenade_electric_smoke = hash("burn_mod_grenade_electric_smoke"),
    charge_lv1 = hash("charge_lv1"),
    charge_lv2 = hash("charge_lv2"),
    charge_lv3 = hash("charge_lv3"),
    charge_lv4 = hash("charge_lv4"),
    charge_lv5 = hash("charge_lv5"),
    choke = hash("choke"),
    crate_ = hash("crate"),
    crypto_drone_access = hash("crypto_drone_access"),
    crypto_has_camera = hash("crypto_has_camera"),
    data_knife = hash("data_knife"),
    dev_mod_low_recharge = hash("dev_mod_low_recharge"),
    disable_lunge = hash("disable_lunge"),
    double_link_mod = hash("double_link_mod"),
    drill_error = hash("drill_error"),
    elevator_shooter = hash("elevator_shooter"),
    energized = hash("energized"),
    energy_mag_l1 = hash("energy_mag_l1"),
    energy_mag_l2 = hash("energy_mag_l2"),
    energy_mag_l3 = hash("energy_mag_l3"),
    energy_mag_l4 = hash("energy_mag_l4"),
    extra_tactical_charges = hash("extra_tactical_charges"),
    fast_reload_mod = hash("fast_reload_mod"),
    fuse_long_throw_passive = hash("fuse_long_throw_passive"),
    g7_reactive_ammo_counter_mod = hash("g7_reactive_ammo_counter_mod"),
    gold_paintball = hash("gold_paintball"),
    grapple_regen_stop = hash("grapple_regen_stop"),
    has_been_energized = hash("has_been_energized"),
    highcal_mag_l1 = hash("highcal_mag_l1"),
    highcal_mag_l2 = hash("highcal_mag_l2"),
    highcal_mag_l3 = hash("highcal_mag_l3"),
    highcal_mag_l4 = hash("highcal_mag_l4"),
    hopup_april_fools = hash("hopup_april_fools"),
    hopup_april_fools_old = hash("hopup_april_fools_old"),
    hopup_double_tap = hash("hopup_double_tap"),
    hopup_dual_loader = hash("hopup_dual_loader"),
    hopup_energy_choke = hash("hopup_energy_choke"),
    hopup_headshot_dmg = hash("hopup_headshot_dmg"),
    hopup_highcal_rounds = hash("hopup_highcal_rounds"),
    hopup_kinetic_choke = hash("hopup_kinetic_choke"),
    hopup_kinetic_loader = hash("hopup_kinetic_loader"),
    hopup_marksmans_tempo = hash("hopup_marksmans_tempo"),
    hopup_multiplexer = hash("hopup_multiplexer"),
    hopup_quickdraw_holster = hash("hopup_quickdraw_holster"),
    hopup_shatter_rounds = hash("hopup_shatter_rounds"),
    hopup_shield_breaker = hash("hopup_shield_breaker"),
    hopup_smart_reload = hash("hopup_smart_reload"),
    hopup_turbocharger = hash("hopup_turbocharger"),
    hopup_unshielded_dmg = hash("hopup_unshielded_dmg"),
    in_ads = hash("in_ads"),
    infinite_ammo_clips = hash("infinite_ammo_clips"),
    interception_pylon_super_charge = hash("interception_pylon_super_charge"),
    kinetic_choke = hash("kinetic_choke"),
    laser_sight_l1 = hash("laser_sight_l1"),
    laser_sight_l2 = hash("laser_sight_l2"),
    laser_sight_l3 = hash("laser_sight_l3"),
    legendary1 = hash("legendary1"),
    legendary2 = hash("legendary2"),
    legendary_nrg_crys_muzzle_flash = hash("legendary_nrg_crys_muzzle_flash"),
    legendary_nrg_crys_reactive_charge = hash("legendary_nrg_crys_reactive_charge"),
    legendary_nrg_ice_muzzle_flash = hash("legendary_nrg_ice_muzzle_flash"),
    legendary_nrg_ice_reactive_charge = hash("legendary_nrg_ice_reactive_charge"),
    marksmans_tempo_active = hash("marksmans_tempo_active"),
    marksmans_tempo_buildup = hash("marksmans_tempo_buildup"),
    melee_crypto_drone = hash("melee_crypto_drone"),
    melee_valk_helmet = hash("melee_valk_helmet"),
    mobile_hmg_active = hash("mobile_hmg_active"),
    mobile_hmg_fast_switch = hash("mobile_hmg_fast_switch"),
    mortar_ring_arc_disabled_mod = hash("mortar_ring_arc_disabled_mod"),
    mortar_ring_target_blocked_mod = hash("mortar_ring_target_blocked_mod"),
    newcastle_ult_leap_dash_mod = hash("newcastle_ult_leap_dash_mod"),
    newcastle_ult_leap_launch_mod = hash("newcastle_ult_leap_launch_mod"),
    newcastle_ult_leap_slam_mod = hash("newcastle_ult_leap_slam_mod"),
    npc_shotgunner = hash("npc_shotgunner"),
    npc_stalker = hash("npc_stalker"),
    optic_cq_hcog_bruiser = hash("optic_cq_hcog_bruiser"),
    optic_cq_hcog_classic = hash("optic_cq_hcog_classic"),
    optic_cq_holosight = hash("optic_cq_holosight"),
    optic_cq_holosight_variable = hash("optic_cq_holosight_variable"),
    optic_cq_threat = hash("optic_cq_threat"),
    optic_hawk_sniper = hash("optic_hawk_sniper"),
    optic_ranged_aog_variable = hash("optic_ranged_aog_variable"),
    optic_ranged_hcog = hash("optic_ranged_hcog"),
    optic_sniper = hash("optic_sniper"),
    optic_sniper_threat = hash("optic_sniper_threat"),
    optic_sniper_variable = hash("optic_sniper_variable"),
    optic_toggle = hash("optic_toggle"),
    overloaded_ammo = hash("overloaded_ammo"),
    pariah_ads_melee = hash("pariah_ads_melee"),
    pas_ordnance_pack = hash("pas_ordnance_pack"),
    pas_power_cell = hash("pas_power_cell"),
    proto_door_kick = hash("proto_door_kick"),
    proto_door_kick_impact_table = hash("proto_door_kick_impact_table"),
    purple_paintball = hash("purple_paintball"),
    rampart_gunner = hash("rampart_gunner"),
    redirect_mod = hash("redirect_mod"),
    s07_reactive_holo_ironsight = hash("s07_reactive_holo_ironsight"),
    s07_reactive_holo_ironsight_alt = hash("s07_reactive_holo_ironsight_alt"),
    seer_heartbeat_sensor_active = hash("seer_heartbeat_sensor_active"),
    seer_tac_movespeed_modifier = hash("seer_tac_movespeed_modifier"),
    shatter_rounds_hipfire = hash("shatter_rounds_hipfire"),
    shotgun_bolt_l1 = hash("shotgun_bolt_l1"),
    shotgun_bolt_l1_double_tap = hash("shotgun_bolt_l1_double_tap"),
    shotgun_bolt_l2 = hash("shotgun_bolt_l2"),
    shotgun_bolt_l2_double_tap = hash("shotgun_bolt_l2_double_tap"),
    shotgun_bolt_l3 = hash("shotgun_bolt_l3"),
    shotgun_bolt_l3_double_tap = hash("shotgun_bolt_l3_double_tap"),
    shotgun_bolt_l4 = hash("shotgun_bolt_l4"),
    shotgun_bolt_l4_double_tap = hash("shotgun_bolt_l4_double_tap"),
    sniper_mag_l1 = hash("sniper_mag_l1"),
    sniper_mag_l2 = hash("sniper_mag_l2"),
    sniper_mag_l3 = hash("sniper_mag_l3"),
    sniper_mag_l4 = hash("sniper_mag_l4"),
    sp_disable_arc_indicator = hash("sp_disable_arc_indicator"),
    spree_lvl1_grenade_smoke = hash("spree_lvl1_grenade_smoke"),
    spree_lvl2_grenade_smoke = hash("spree_lvl2_grenade_smoke"),
    spree_lvl3_grenade_smoke = hash("spree_lvl3_grenade_smoke"),
    std_charge_dmg_lv1 = hash("std_charge_dmg_lv1"),
    std_charge_dmg_lv2 = hash("std_charge_dmg_lv2"),
    std_charge_dmg_lv3 = hash("std_charge_dmg_lv3"),
    std_charge_dmg_lv4 = hash("std_charge_dmg_lv4"),
    std_charge_dmg_lv5 = hash("std_charge_dmg_lv5"),
    stock_sniper_l1 = hash("stock_sniper_l1"),
    stock_sniper_l2 = hash("stock_sniper_l2"),
    stock_sniper_l3 = hash("stock_sniper_l3"),
    stock_tactical_l1 = hash("stock_tactical_l1"),
    stock_tactical_l2 = hash("stock_tactical_l2"),
    stock_tactical_l3 = hash("stock_tactical_l3"),
    survival_ammo_regen_paused = hash("survival_ammo_regen_paused"),
    survival_ammo_regen_staging = hash("survival_ammo_regen_staging"),
    survival_armor_cooldown_mod = hash("survival_armor_cooldown_mod"),
    survival_finite_ordnance = hash("survival_finite_ordnance"),
    ult_active = hash("ult_active"),
    ultimate_active = hash("ultimate_active"),
    ultimate_active_no_regen = hash("ultimate_active_no_regen"),
    ultimates_charge_over_time_disabled = hash("ultimates_charge_over_time_disabled"),
    using_jets = hash("using_jets"),
    vantage_ads_melee = hash("vantage_ads_melee"),
}

pub fn mod_name_item(mod_name: ModName) -> ItemId {
    match mod_name {
        ModName::barrel_stabilizer_l1 => ItemId::BarrelStabilizerLv1,
        ModName::barrel_stabilizer_l2 => ItemId::BarrelStabilizerLv2,
        ModName::barrel_stabilizer_l3 => ItemId::BarrelStabilizerLv3,
        ModName::barrel_stabilizer_l4_flash_hider => ItemId::BarrelStabilizerLv4,
        ModName::laser_sight_l1 => ItemId::LaserSightLv1,
        ModName::laser_sight_l2 => ItemId::LaserSightLv2,
        ModName::laser_sight_l3 => ItemId::LaserSightLv3,
        ModName::bullets_mag_l1 => ItemId::LightMagazineLv1,
        ModName::bullets_mag_l2 => ItemId::LightMagazineLv2,
        ModName::bullets_mag_l3 => ItemId::LightMagazineLv3,
        ModName::bullets_mag_l4 => ItemId::LightMagazineLv4,
        ModName::highcal_mag_l1 => ItemId::HeavyMagazineLv1,
        ModName::highcal_mag_l2 => ItemId::HeavyMagazineLv2,
        ModName::highcal_mag_l3 => ItemId::HeavyMagazineLv3,
        ModName::highcal_mag_l4 => ItemId::HeavyMagazineLv4,
        ModName::energy_mag_l1 => ItemId::EnergyMagazineLv1,
        ModName::energy_mag_l2 => ItemId::EnergyMagazineLv2,
        ModName::energy_mag_l3 => ItemId::EnergyMagazineLv3,
        ModName::energy_mag_l4 => ItemId::EnergyMagazineLv4,
        ModName::sniper_mag_l1 => ItemId::SniperMagazineLv1,
        ModName::sniper_mag_l2 => ItemId::SniperMagazineLv2,
        ModName::sniper_mag_l3 => ItemId::SniperMagazineLv3,
        ModName::sniper_mag_l4 => ItemId::SniperMagazineLv4,
        ModName::shotgun_bolt_l1 => ItemId::ShotgunBoltLv1,
        ModName::shotgun_bolt_l2 => ItemId::ShotgunBoltLv2,
        ModName::shotgun_bolt_l3 => ItemId::ShotgunBoltLv3,
        ModName::shotgun_bolt_l4 => ItemId::ShotgunBoltLv4,
        ModName::shotgun_bolt_l1_double_tap => ItemId::ShotgunBoltLv1,
        ModName::shotgun_bolt_l2_double_tap => ItemId::ShotgunBoltLv2,
        ModName::shotgun_bolt_l3_double_tap => ItemId::ShotgunBoltLv3,
        ModName::shotgun_bolt_l4_double_tap => ItemId::ShotgunBoltLv4,
        ModName::stock_tactical_l1 => ItemId::StandardStockLv1,
        ModName::stock_tactical_l2 => ItemId::StandardStockLv2,
        ModName::stock_tactical_l3 => ItemId::StandardStockLv3,
        ModName::stock_sniper_l1 => ItemId::SniperStockLv1,
        ModName::stock_sniper_l2 => ItemId::SniperStockLv2,
        ModName::stock_sniper_l3 => ItemId::SniperStockLv3,
        ModName::optic_cq_hcog_classic => ItemId::HcogClassic,
        ModName::optic_cq_hcog_bruiser => ItemId::HcogBruiser,
        ModName::optic_ranged_hcog => ItemId::HcogRanger,
        ModName::optic_cq_holosight => ItemId::Holo,
        ModName::optic_cq_holosight_variable => ItemId::VariableHolo,
        ModName::optic_ranged_aog_variable => ItemId::VariableAOG,
        ModName::optic_cq_threat => ItemId::DigiThreat,
        ModName::optic_sniper => ItemId::Sniper,
        ModName::optic_sniper_variable => ItemId::VariableSniper,
        ModName::optic_sniper_threat => ItemId::DigiSniperThreat,
        _ => ItemId::None,
    }
}
