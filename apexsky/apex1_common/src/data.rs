use std::sync::LazyLock as Lazy;

use named_constants::named_constants;
use secrecy::SecretString;

/*
 * GameVersion=v3.0.1.29
 */

pub static GAME_VER_DX12_PROCESS_NAME: Lazy<SecretString> =
    Lazy::new(|| SecretString::from(obfstr::obfstr!("r5apex_dx12.exe")));

// m_ammoPoolCount - 0x8
pub const OFFSET_YAW: u64 = 0x229c - 0x8;
//pub const OFFSET_GLOW_CONTEXT_ID: u64 = 0x29c;
pub const OFFSET_GLOW_VISIBLE_TYPE: u64 = 0x26c;
pub const OFFSET_GLOW_DISTANCE: u64 = 0x264;
pub const OFFSET_GLOW_FIX: u64 = 0x278;
//pub const OFFSET_MODULE_BASE: u64 = 0x140000000;

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
    Alternator = 83,
    // AlternatorDayzero = (was 85), removed
    Re45 = 84,
    Re45Crate = 85,
    // Re45Dayzero = (was 88), removed
    ChargeRifle = 86,
    // ChargeRifleCe = (was 90), removed
    Devotion = 87,
    DevotionCrate = 88,
    // DevotionDayzero = (was 93), removed
    Longbow = 89,
    // LongbowDayzero = (was 95), removed
    Havoc = 90,
    // HavocDayzero = (was 97), removed
    HavocCrate = 91,
    Eva8 = 92,
    Eva8Crate = 93,
    // Eva8Dayzero = (was 101), removed
    Flatline = 94,
    // FlatlineDayzero = (was 103), removed
    G7Scout = 95,
    // G7ScoutDayzero = (was 105), removed
    Hemlock = 96,
    HermlockCrate = 97,
    // HemlockDayzero = (was 108), removed
    Kraber = 98,
    // KraberDayzero = (was 110), removed
    Lstar = 99,
    LstarCrate = 100,
    // LstarDayzero = (was 113), removed
    Mastiff = 101,
    // MastiffDayzero = (was 115), removed
    MastiffCrate = 102,
    Mozambique = 103,
    MozambiqueLight = 104,
    MozambiqueEnergy = 105,
    MozambiqueSniper = 106,
    MozambiqueHeavy = 107,
    // MozambiqueDayzero = (was 122), removed
    Prowler = 108,
    ProwlerCrate = 109,
    // ProwlerDayzero = (was 125), removed
    Peacekeeper = 110,
    // PeacekeeperDayzero = (was 127), removed
    PeacekeeperCrate = 111,
    // R301Dayzero = (was 129), removed
    R99 = 112,
    // R99Dayzero = (was 131), removed
    R99Crate = 113,
    P2020 = 114,
    // P2020Dayzero = (was 134), removed
    Spitfire = 115,
    // SpitfireDayzero = (was 136), removed
    TripleTake = 116,
    // TripleTakeDayzero = (was 138), removed
    Wingman = 117,
    WingmanCrate = 118,
    // WingmanDayzero = (was 141), removed
    Volt = 119,
    _3030Repeater = 120,
    CarSmg = 121,
    Nemesis = 122,
    GrenadeFrag = 123,
    GrenadeArcStar = 124,
    GrenadeThermite = 125,
    Hands = 126,
    Epg = 180,
    EpgTethered = 181,
    ThrowingKnife = 182,
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
    WeaponLStar = 8,
    WeaponHavoc = 19,
    WeaponDevotion = 20,
    WeaponTripleTake = 26,
    WeaponFlatline = 31,
    WeaponHemlock = 36,
    WeaponG7Scout = 42,
    WeaponAlternator = 47,
    WeaponR99 = 52,
    WeaponProwler = 58,
    WeaponVolt = 64,
    WeaponLongbow = 69,
    WeaponChargeRifle = 74,
    WeaponSpitfire = 79,
    WeaponR301 = 84,
    WeaponEva8 = 92,
    WeaponPeacekeeper = 99,
    WeaponMozambique = 104,
    WeaponWingman = 120,
    WeaponP2020 = 126,
    WeaponRE45 = 135,
    WeaponSentinel = 141,
    WeaponBow = 148,
    Weapon3030Repeater = 150,
    WeaponNemesis = 155,
    LightAmmo = 160,
    EnergyAmmo = 161,
    ShotgunAmmo = 162,
    HeavyAmmo = 163,
    SniperAmmo = 164,
    WeaponRampage = 166,
    WeaponCARSMG = 171,
    Accelerant = 208,
    Phoenix = 209,
    HealthLarge = 210,
    HealthSmall = 211,
    ShieldBatteryLarge = 212,
    ShieldBatterySmall = 213,
    //ShieldUpgradeHead1,
    //ShieldUpgradeHead2,
    ShieldUpgradeHead3 = 218,
    ShieldUpgradeHead4 = 219,
    ArmorCore1 = 231,
    ArmorCore2 = 232,
    ArmorCore3 = 233,
    //ArmorCore4,
    ShieldDown1 = 236,
    ShieldDown2 = 237,
    ShieldDown3 = 238,
    //ShieldDown4,
    LightBackpack = 240,
    MedBackpack = 241,
    HeavyBackpack = 242,
    GoldBackpack = 243,
    ThrowingKnife = 244,
    GrenadeThermite = 245,
    GrenadeFrag = 246,
    GrenadeArcStar = 247,
    Optic1xHCOG = 248,
    Optic2xHCOG = 249,
    //OpticHolo1x,
    OpticHolo1x2x = 251,
    //OpticThreat,
    Optic3xHCOG = 253,
    Optic2x4x = 254,
    OpticSniper6x = 255,
    OpticSniper4x8x = 256,
    OpticSniperThreat = 257,
    Suppressor1 = 258,
    Suppressor2 = 259,
    Suppressor3 = 260,
    LaserSight1 = 261,
    LaserSight2 = 262,
    LaserSight3 = 263,
    LightAmmoMag1 = 264,
    LightAmmoMag2 = 265,
    LightAmmoMag3 = 266,
    LightAmmoMag4 = 267,
    HeavyAmmoMag1 = 268,
    HeavyAmmoMag2 = 269,
    HeavyAmmoMag3 = 270,
    HeavyAmmoMag4 = 271,
    EnergyAmmoMag1 = 272,
    EnergyAmmoMag2 = 273,
    EnergyAmmoMag3 = 274,
    EnergyAmmoMag4 = 275,
    SniperAmmoMag1 = 276,
    SniperAmmoMag2 = 277,
    SniperAmmoMag3 = 278,
    SniperAmmoMag4 = 279,
    ShotgunBolt1 = 280,
    ShotgunBolt2 = 281,
    ShotgunBolt3 = 282,
    ShotgunBolt4 = 283,
    StockRegular1 = 284,
    StockRegular2 = 285,
    StockRegular3 = 286,
    StockSniper1 = 287,
    StockSniper2 = 288,
    StockSniper3 = 289,
    TurboCharger = 290, // old
    SelectfireReceiver = 291,
    HammerPoint = 304,
    BoostedLoader = 305,
    GunShieldGenerator,
    SkullPiecer,     // old
    DisruptorRounds, // old
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
pub const HIGHLIGHT_WEAPON_RAINBOW: u8 = 81;

#[derive(Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct LootInt {
    pub int: i32,
    pub model: String,
}

#[cfg(feature = "data")]
pub static ITEM_LIST: Lazy<std::collections::HashMap<i32, String>> = Lazy::new(|| {
    include_flate::flate!(static ITEM_JSON: [u8] from "resource/default/item.json" with zstd);
    let data: Vec<LootInt> = serde_json::from_slice(&ITEM_JSON).unwrap();
    data.into_iter()
        .map(|item| (item.int, item.model))
        .collect()
});
#[cfg(feature = "data")]
pub static WEAPON_LIST: Lazy<Vec<String>> = Lazy::new(|| {
    include_flate::flate!(static WEAPON_JSON: [u8] from "resource/default/weapon.json" with zstd);
    serde_json::from_slice(&WEAPON_JSON).unwrap()
});
