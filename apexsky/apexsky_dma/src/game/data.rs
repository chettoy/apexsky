use std::collections::HashMap;

use named_constants::named_constants;
use once_cell::sync::Lazy;

use crate::workers::items::LootInt;

/*
 * GameVersion=v3.0.82.42
 */

pub static TARGET_PROCESS_NAME: Lazy<String> =
    Lazy::new(|| obfstr::obfstr!("r5apex.exe").to_string());

pub const OFFSET_YAW: u64 = 0x223c - 0x8;
//pub const OFFSET_GLOW_CONTEXT_ID: u64 = 0x29c;
pub const OFFSET_GLOW_VISIBLE_TYPE: u64 = 0x26c;
pub const OFFSET_GLOW_DISTANCE: u64 = 0x264;
pub const OFFSET_GLOW_FIX: u64 = 0x278;
pub const OFFSET_MODULE_BASE: u64 = 0x140000000;

#[allow(dead_code)]
#[named_constants]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum WeaponId {
    R301 = 0,
    Sentinel = 1,
    Bow = 2,
    R2R5 = 3,
    Rampage = 6,
    Alternator = 84,
    Re45 = 85,
    ChargeRifle = 87,
    Devotion = 88,
    Longbow = 90,
    Havoc = 91,
    Eva8 = 92,
    Flatline = 94,
    G7Scout = 95,
    Hemlock = 96,
    Kraber = 98,
    Lstar = 99,
    Mastiff = 101,
    Mozambique = 102,
    Prowler = 107,
    Peacekeeper = 109,
    R99 = 110,
    R99Crate = 111,
    P2020 = 112,
    Spitfire = 113,
    TripleTake = 114,
    Wingman = 115,
    WingmanCrate = 116,
    Volt = 117,
    _3030Repeater = 118,
    CarSmg = 119,
    Nemesis = 120,
    Hands = 121,
    ThrowingKnife = 178,
    GrenadeThermite = 179,
    GrenadeFrag = 180,
    GrenadeArcStar = 181,
    Max,
}

#[allow(dead_code)]
#[named_constants]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum ItemId {
    ApexskyItemDeathBox = -1,
    None = 0,
    WeaponKraber = 1,
    WeaponMastiff = 2,
    WeaponLStar = 7,
    WeaponHavoc = 13,
    WeaponDevotion = 19,
    WeaponTripleTake = 24,
    WeaponFlatline = 29,
    WeaponHemlock = 34,
    WeaponG7Scout = 39,
    WeaponAlternator = 44,
    WeaponR99 = 49,
    WeaponProwler = 55,
    WeaponVolt = 60,
    WeaponLongbow = 65,
    WeaponChargeRifle = 70,
    WeaponSpitfire = 75,
    WeaponR301 = 80,
    WeaponEva8 = 85,
    WeaponPeacekeeper = 90,
    WeaponMozambique = 95,
    WeaponWingman = 108,
    WeaponP2020 = 113,
    WeaponRE45 = 122,
    WeaponSentinel = 127,
    WeaponBow = 132,
    Weapon3030Repeater = 133,
    WeaponNemesis = 139,
    LightAmmo = 144,
    EnergyAmmo = 145,
    ShotgunAmmo = 146,
    HeavyAmmo = 147,
    SniperAmmo = 148,
    WeaponRampage = 150,
    WeaponCARSMG = 155,
    Accelerant = 193,
    Phoenix = 194,
    HealthLarge = 195,
    HealthSmall = 196,
    ShieldBatteryLarge = 197,
    ShieldBatterySmall = 198,
    ShieldUpgradeHead1 = 199,
    ShieldUpgradeHead2 = 200,
    ShieldUpgradeHead3 = 201,
    ShieldUpgradeHead4 = 202,
    ArmorCore1 = 214,
    ArmorCore2 = 215,
    ArmorCore3 = 216,
    ArmorCore4 = 217,
    ShieldDown1 = 219,
    ShieldDown2 = 220,
    ShieldDown3 = 221,
    ShieldDown4 = 222,
    LightBackpack = 223,
    MedBackpack = 224,
    HeavyBackpack = 225,
    GoldBackpack = 226,
    ThrowingKnife = 227,
    GrenadeThermite = 228,
    GrenadeFrag = 229,
    GrenadeArcStar = 230,
    Optic1xHCOG = 231,
    Optic2xHCOG = 232,
    OpticHolo1x = 233,
    OpticHolo1x2x = 234,
    OpticThreat = 235,
    Optic3xHCOG = 236,
    Optic2x4x = 247,
    OpticSniper6x = 238,
    OpticSniper4x8x = 239,
    OpticSniperThreat = 240,
    Suppressor1 = 241,
    Suppressor2 = 242,
    Suppressor3 = 243,
    LaserSight1 = 245,
    LaserSight2 = 246,
    LaserSight3 = 247,
    LightAmmoMag1 = 248,
    LightAmmoMag2 = 249,
    LightAmmoMag3 = 250,
    LightAmmoMag4 = 251,
    HeavyAmmoMag1 = 252,
    HeavyAmmoMag2 = 253,
    HeavyAmmoMag3 = 254,
    HeavyAmmoMag4 = 255,
    EnergyAmmoMag1 = 256,
    EnergyAmmoMag2 = 257,
    EnergyAmmoMag3 = 258,
    EnergyAmmoMag4 = 259,
    SniperAmmoMag1 = 260,
    SniperAmmoMag2 = 261,
    SniperAmmoMag3 = 262,
    SniperAmmoMag4 = 263,
    ShotgunBolt1 = 264,
    ShotgunBolt2 = 265,
    ShotgunBolt3 = 266,
    ShotgunBolt4 = 267,
    StockRegular1 = 268,
    StockRegular2 = 269,
    StockRegular3 = 270,
    StockSniper1 = 271,
    StockSniper2 = 272,
    StockSniper3 = 273,
    TurboCharger = 280,
    SkullPiecer = 282, // old
    DisruptorRounds = 288,
    HammerPoint = 289,
    BoostedLoader = 295, // old
}

pub const HIGHLIGHT_LOOT_GOLD: u8 = 31;
pub const HIGHLIGHT_LOOT_RED: u8 = 32;
pub const HIGHLIGHT_LOOT_PURPLE: u8 = 33;
pub const HIGHLIGHT_LOOT_BLUE: u8 = 34;
pub const HIGHLIGHT_LOOT_WHITE: u8 = 35;
pub const HIGHLIGHT_LOOT_ENERGY: u8 = 36;
pub const HIGHLIGHT_LOOT_HEAVY: u8 = 37;
pub const HIGHLIGHT_LOOT_LIGHT: u8 = 38;
pub const HIGHLIGHT_LOOT_GREY: u8 = 39;
pub const HIGHLIGHT_DEATH_BOX: u8 = 40;
pub const HIGHLIGHT_PLAYER_KNOCKED: u8 = 70;
pub const HIGHLIGHT_PLAYER_VISIBLE: u8 = 71;
pub const HIGHLIGHT_PLAYER_NOTVIZ: u8 = 72;
pub const HIGHLIGHT_PLAYER_ORANGE: u8 = 74;
pub const HIGHLIGHT_PLAYER_WHITE: u8 = 75;
pub const HIGHLIGHT_PLAYER_BLUE: u8 = 76;
pub const HIGHLIGHT_PLAYER_PURPLE: u8 = 77;
pub const HIGHLIGHT_PLAYER_RED: u8 = 78;
pub const HIGHLIGHT_PLAYER_RAINBOW: u8 = 79;
pub const HIGHLIGHT_PLAYER_BLACK: u8 = 80;

pub static ITEM_LIST: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    include_flate::flate!(static ITEM_JSON: str from "../apexsky/resource/default/item.json");
    let data: Vec<LootInt> = serde_json::from_str(&ITEM_JSON).unwrap();
    data.into_iter()
        .map(|item| (item.int, item.model))
        .collect()
});
pub static WEAPON_LIST: Lazy<Vec<String>> = Lazy::new(|| {
    include_flate::flate!(static WEAPON_JSON: str from "../apexsky/resource/default/weapon.json");
    serde_json::from_str(&WEAPON_JSON).unwrap()
});
