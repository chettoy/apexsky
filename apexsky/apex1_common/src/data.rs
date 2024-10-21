use std::sync::LazyLock as Lazy;

use named_constants::named_constants;
use secrecy::SecretString;

/*
 * GameVersion=v3.0.6.37
 */

pub static GAME_VER_DX12_PROCESS_NAME: Lazy<SecretString> =
    Lazy::new(|| SecretString::from(obfstr::obfstr!("r5apex_dx12.exe")));
pub static GAME_VER_DX11_PROCESS_NAME: Lazy<SecretString> =
    Lazy::new(|| SecretString::from(obfstr::obfstr!("r5apex.exe")));

// m_ammoPoolCount - 0x8
pub const OFFSET_YAW: u64 = 0x22bc - 0x8;
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
    Sentinel = 2,
    Bow = 3,
    R2R5 = 4,
    Rampage = 7,
    Alternator = 87,
    AlternatorDayzero = 88,
    Re45 = 89,
    Re45Crate = 90,
    Re45Dayzero = 91,
    ChargeRifle = 92,
    ChargeRifleCe = 93,
    Devotion = 94,
    DevotionCrate = 95,
    DevotionDayzero = 96,
    Longbow = 97,
    LongbowDayzero = 98,
    Havoc = 99,
    HavocDayzero = 100,
    HavocCrate = 101,
    Eva8 = 102,
    Eva8Crate = 103,
    Eva8Dayzero = 104,
    Flatline = 105,
    FlatlineDayzero = 106,
    G7Scout = 107,
    G7ScoutDayzero = 108,
    Hemlock = 109,
    HermlockCrate = 110,
    HemlockDayzero = 111,
    Kraber = 112,
    KraberDayzero = 113,
    Lstar = 114,
    LstarCrate = 115,
    LstarDayzero = 116,
    Mastiff = 117,
    MastiffDayzero = 118,
    MastiffCrate = 119,
    Mozambique = 120,
    MozambiqueLight = 121,
    MozambiqueEnergy = 122,
    MozambiqueSniper = 123,
    MozambiqueHeavy = 124,
    MozambiqueDayzero = 125,
    Prowler = 126,
    ProwlerCrate = 127,
    ProwlerDayzero = 128,
    Peacekeeper = 129,
    PeacekeeperDayzero = 130,
    R301Dayzero = 131,
    R99 = 132,
    R99Dayzero = 133,
    R99Crate = 134,
    P2020 = 135,
    P2020Dayzero = 136,
    Spitfire = 137,
    SpitfireDayzero = 138,
    TripleTake = 139,
    TripleTakeDayzero = 140,
    Wingman = 141,
    WingmanCrate = 142,
    WingmanDayzero = 143,
    Volt = 144,
    _3030Repeater = 145,
    CarSmg = 146,
    Nemesis = 147,
    Hands = 148,
    Epg = 199,
    EpgTethered = 200,
    ThrowingKnife = 201,
    GrenadeThermite = 202,
    GrenadeFrag = 203,
    GrenadeArcStar = 204,
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
    WeaponMastiff = 3,
    WeaponLStar = 11,
    WeaponHavoc = 29,
    WeaponDevotion = 30,
    WeaponTripleTake = 41,
    WeaponFlatline = 50,
    WeaponHemlock = 59,
    WeaponG7Scout = 69,
    WeaponAlternator = 79,
    WeaponR99 = 98,
    WeaponProwler = 99,
    WeaponVolt = 111,
    WeaponLongbow = 116,
    WeaponChargeRifle = 126,
    WeaponSpitfire = 133,
    WeaponR301 = 143,
    WeaponEva8 = 153,
    WeaponPeacekeeper = 164,
    WeaponMozambique = 174,
    WeaponWingman = 191,
    WeaponP2020 = 203,
    WeaponRE45 = 216,
    WeaponSentinel = 226,
    WeaponBow = 234,
    Weapon3030Repeater = 235,
    WeaponNemesis = 242,
    LightAmmo = 247,
    EnergyAmmo = 248,
    ShotgunAmmo = 249,
    HeavyAmmo = 250,
    SniperAmmo = 251,
    WeaponRampage = 253,
    WeaponCARSMG = 259,
    Accelerant = 297,
    Phoenix = 298,
    HealthLarge = 299,
    HealthSmall = 300,
    ShieldBatteryLarge = 301,
    ShieldBatterySmall = 302,
    ShieldUpgradeHead1 = 303,
    ShieldUpgradeHead2 = 304,
    ShieldUpgradeHead3 = 305,
    ShieldUpgradeHead4 = 306,
    ArmorCore1 = 319,
    ArmorCore2 = 320,
    ArmorCore3 = 321,
    ArmorCore4 = 322,
    ShieldDown1 = 324,
    ShieldDown2 = 325,
    ShieldDown3 = 326,
    ShieldDown4 = 327,
    LightBackpack = 329,
    MedBackpack = 330,
    HeavyBackpack = 331,
    GoldBackpack = 332,
    ThrowingKnife = 334,
    GrenadeThermite = 321,
    GrenadeFrag = 336,
    GrenadeArcStar = 337,
    Optic1xHCOG = 338,
    Optic2xHCOG = 339,
    OpticHolo1x = 340,
    OpticHolo1x2x = 341,
    OpticThreat = 342,
    Optic3xHCOG = 344,
    Optic2x4x = 345,
    OpticSniper6x = 346,
    OpticSniper4x8x = 347,
    OpticSniperThreat = 348,
    Suppressor1 = 349,
    Suppressor2 = 350,
    Suppressor3 = 351,
    LaserSight1 = 353,
    LaserSight2 = 354,
    LaserSight3 = 355,
    LightAmmoMag1 = 356,
    LightAmmoMag2 = 357,
    LightAmmoMag3 = 358,
    LightAmmoMag4 = 359,
    HeavyAmmoMag1 = 360,
    HeavyAmmoMag2 = 361,
    HeavyAmmoMag3 = 362,
    HeavyAmmoMag4 = 363,
    EnergyAmmoMag1 = 364,
    EnergyAmmoMag2 = 365,
    EnergyAmmoMag3 = 366,
    EnergyAmmoMag4 = 367,
    SniperAmmoMag1 = 368,
    SniperAmmoMag2 = 369,
    SniperAmmoMag3 = 370,
    SniperAmmoMag4 = 371,
    ShotgunBolt1 = 372,
    ShotgunBolt2 = 373,
    ShotgunBolt3 = 374,
    ShotgunBolt4 = 375,
    StockRegular1 = 376,
    StockRegular2 = 377,
    StockRegular3 = 378,
    StockSniper1 = 379,
    StockSniper2 = 380,
    StockSniper3 = 381,
    TurboCharger = 382, // old
    SelectfireReceiver = 383,
    HammerPoint = 389,
    BoostedLoader = 396,
    GunShieldGenerator = 397,
    SkullPiecer = 398,     // old
    DisruptorRounds = 401, // old
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
