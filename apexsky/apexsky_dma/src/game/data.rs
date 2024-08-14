use std::collections::HashMap;

use named_constants::named_constants;
use once_cell::sync::Lazy;

use crate::workers::items::LootInt;

/*
 * GameVersion=v3.0.77.28
 */
pub const OFFSET_YAW: u64 = 0x223c - 0x8;
//pub const OFFSET_GLOW_CONTEXT_ID: u64 = 0x29c;
pub const OFFSET_GLOW_VISIBLE_TYPE: u64 = 0x26c;
pub const OFFSET_GLOW_DISTANCE: u64 = 0x264;
pub const OFFSET_GLOW_FIX: u64 = 0x278;

#[allow(dead_code)]
#[repr(C)]
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
    ThrowingKnife = 176,
    GrenadeThermite = 177,
    GrenadeFrag = 178,
    GrenadeArcStar = 179,
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
    WeaponG7Scout = 40,
    WeaponAlternator = 45,
    WeaponR99 = 50,
    WeaponProwler = 56,
    WeaponVolt = 62,
    WeaponLongbow = 67,
    WeaponChargeRifle = 72,
    WeaponSpitfire = 77,
    WeaponR301 = 82,
    WeaponEva8 = 87,
    WeaponPeacekeeper = 93,
    WeaponMozambique = 98,
    WeaponWingman = 111,
    WeaponP2020 = 117,
    WeaponRE45 = 126,
    WeaponSentinel = 132,
    WeaponBow = 137,
    Weapon3030Repeater = 139,
    WeaponNemesis = 145,
    LightAmmo = 150,
    EnergyAmmo = 151,
    ShotgunAmmo = 152,
    HeavyAmmo = 153,
    SniperAmmo = 154,
    WeaponRampage = 156,
    WeaponCARSMG = 161,
    Accelerant = 199,
    Phoenix = 200,
    HealthLarge = 201,
    HealthSmall = 202,
    ShieldBatteryLarge = 203,
    ShieldBatterySmall = 204,
    ShieldUpgradeHead1 = 205,
    ShieldUpgradeHead2 = 206,
    ShieldUpgradeHead3 = 207,
    ShieldUpgradeHead4 = 208,
    ArmorCore1 = 220,
    ArmorCore2 = 221,
    ArmorCore3 = 222,
    ArmorCore4 = 223,
    ShieldDown1 = 225,
    ShieldDown2 = 226,
    ShieldDown3 = 227,
    ShieldDown4 = 228,
    LightBackpack = 229,
    MedBackpack = 230,
    HeavyBackpack = 231,
    GoldBackpack = 232,
    ThrowingKnife = 233,
    GrenadeThermite = 234,
    GrenadeFrag = 235,
    GrenadeArcStar = 236,
    Optic1xHCOG = 237,
    Optic2xHCOG = 238,
    OpticHolo1x = 239,
    OpticHolo1x2x = 240,
    OpticThreat = 241,
    Optic3xHCOG = 242,
    Optic2x4x = 243,
    OpticSniper6x = 244,
    OpticSniper4x8x = 245,
    OpticSniperThreat = 246,
    Suppressor1 = 247,
    Suppressor2 = 248,
    Suppressor3 = 249,
    LaserSight1 = 251,
    LaserSight2 = 252,
    LaserSight3 = 253,
    LightAmmoMag1 = 254,
    LightAmmoMag2 = 255,
    LightAmmoMag3 = 256,
    LightAmmoMag4 = 257,
    HeavyAmmoMag1 = 258,
    HeavyAmmoMag2 = 259,
    HeavyAmmoMag3 = 260,
    HeavyAmmoMag4 = 261,
    EnergyAmmoMag1 = 262,
    EnergyAmmoMag2 = 263,
    EnergyAmmoMag3 = 264,
    EnergyAmmoMag4 = 265,
    SniperAmmoMag1 = 266,
    SniperAmmoMag2 = 267,
    SniperAmmoMag3 = 268,
    SniperAmmoMag4 = 269,
    ShotgunBolt1 = 270,
    ShotgunBolt2 = 271,
    ShotgunBolt3 = 272,
    ShotgunBolt4 = 273,
    StockRegular1 = 274,
    StockRegular2 = 275,
    StockRegular3 = 276,
    StockSniper1 = 277,
    StockSniper2 = 278,
    StockSniper3 = 279,
    TurboCharger = 280,
    SkullPiecer = 282,
    DisruptorRounds = 286,
    HammerPoint = 294,
    BoostedLoader = 295,
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
