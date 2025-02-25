use std::sync::LazyLock as Lazy;

use named_constants::named_constants;
use secrecy::SecretString;

/*
 * GameVersion=v3.0.1.25
 */

pub static GAME_VER_DX12_PROCESS_NAME: Lazy<SecretString> =
    Lazy::new(|| SecretString::from(obfstr::obfstr!("r5apex_dx12.exe")));
pub static GAME_VER_DX11_PROCESS_NAME: Lazy<SecretString> =
    Lazy::new(|| SecretString::from(obfstr::obfstr!("r5apex.exe")));

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
    Sentinel = 2,
    Bow = 3,
    R2R5 = 4,
    Rampage = 7,
    Alternator = 84,
    AlternatorDayzero = 85,
    Re45 = 86,
    Re45Crate = 87,
    Re45Dayzero = 88,
    ChargeRifle = 89,
    ChargeRifleCe = 90,
    Devotion = 91,
    DevotionCrate = 92,
    DevotionDayzero = 93,
    Longbow = 94,
    LongbowDayzero = 95,
    Havoc = 96,
    HavocDayzero = 97,
    HavocCrate = 98,
    Eva8 = 99,
    Eva8Crate = 100,
    Eva8Dayzero = 101,
    Flatline = 102,
    FlatlineDayzero = 103,
    G7Scout = 104,
    G7ScoutDayzero = 105,
    Hemlock = 106,
    HermlockCrate = 107,
    HemlockDayzero = 108,
    Kraber = 109,
    KraberDayzero = 110,
    Lstar = 111,
    LstarCrate = 112,
    LstarDayzero = 113,
    Mastiff = 114,
    MastiffDayzero = 115,
    MastiffCrate = 116,
    Mozambique = 117,
    MozambiqueLight = 118,
    MozambiqueEnergy = 119,
    MozambiqueSniper = 120,
    MozambiqueHeavy = 121,
    MozambiqueDayzero = 122,
    Prowler = 123,
    ProwlerCrate = 124,
    ProwlerDayzero = 125,
    Peacekeeper = 126,
    PeacekeeperDayzero = 127,
    PeacekeeperCrate = 128,
    R301Dayzero = 129,
    R99 = 130,
    R99Dayzero = 131,
    R99Crate = 132,
    P2020 = 133,
    P2020Dayzero = 134,
    Spitfire = 135,
    SpitfireDayzero = 136,
    TripleTake = 137,
    TripleTakeDayzero = 138,
    Wingman = 139,
    WingmanCrate = 140,
    WingmanDayzero = 141,
    Volt = 142,
    _3030Repeater = 143,
    CarSmg = 144,
    Nemesis = 145,
    GrenadeFrag = 146,
    GrenadeArcStar = 147,
    GrenadeThermite = 148,
    Hands = 149,
    Epg = 200,
    EpgTethered = 201,
    ThrowingKnife = 202,
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
    WeaponHavoc = 26,
    WeaponDevotion = 27,
    WeaponTripleTake = 35,
    WeaponFlatline = 41,
    WeaponHemlock = 47,
    WeaponG7Scout = 54,
    WeaponAlternator = 62,
    WeaponR99 = 70,
    WeaponProwler = 77,
    WeaponVolt = 86,
    WeaponLongbow = 91,
    WeaponChargeRifle = 98,
    WeaponSpitfire = 106,
    WeaponR301 = 114,
    WeaponEva8 = 124,
    WeaponPeacekeeper = 136,
    WeaponMozambique = 141,
    WeaponWingman = 155,
    WeaponP2020 = 164,
    WeaponRE45 = 174,
    WeaponSentinel = 181,
    WeaponBow = 190,
    Weapon3030Repeater = 191,
    WeaponNemesis = 198,
    LightAmmo = 203,
    EnergyAmmo = 204,
    ShotgunAmmo = 205,
    HeavyAmmo = 206,
    SniperAmmo = 207,
    WeaponRampage = 209,
    WeaponCARSMG = 216,
    Accelerant = 253,
    Phoenix = 254,
    HealthLarge = 255,
    HealthSmall = 256,
    ShieldBatteryLarge = 257,
    ShieldBatterySmall = 258,
    //ShieldUpgradeHead1,
    //ShieldUpgradeHead2,
    ShieldUpgradeHead3 = 263,
    ShieldUpgradeHead4 = 264,
    ArmorCore1 = 277,
    ArmorCore2 = 278,
    ArmorCore3 = 279,
    ArmorCore4 = 280,
    ShieldDown1 = 282,
    ShieldDown2 = 283,
    ShieldDown3 = 284,
    //ShieldDown4,
    LightBackpack = 287,
    MedBackpack = 288,
    HeavyBackpack = 289,
    GoldBackpack = 290,
    ThrowingKnife = 292,
    GrenadeThermite = 294,
    GrenadeFrag = 295,
    GrenadeArcStar = 296,
    Optic1xHCOG = 297,
    Optic2xHCOG = 298,
    //OpticHolo1x,
    OpticHolo1x2x = 300,
    OpticThreat = 301,
    Optic3xHCOG = 302,
    Optic2x4x = 303,
    OpticSniper6x = 304,
    OpticSniper4x8x = 305,
    OpticSniperThreat = 306,
    Suppressor1 = 307,
    Suppressor2 = 308,
    Suppressor3 = 309,
    LaserSight1 = 311,
    LaserSight2 = 312,
    LaserSight3 = 313,
    LightAmmoMag1 = 314,
    LightAmmoMag2 = 315,
    LightAmmoMag3 = 316,
    LightAmmoMag4 = 317,
    HeavyAmmoMag1 = 318,
    HeavyAmmoMag2 = 319,
    HeavyAmmoMag3 = 320,
    HeavyAmmoMag4 = 321,
    EnergyAmmoMag1 = 322,
    EnergyAmmoMag2 = 323,
    EnergyAmmoMag3 = 324,
    EnergyAmmoMag4 = 325,
    SniperAmmoMag1 = 326,
    SniperAmmoMag2 = 327,
    SniperAmmoMag3 = 328,
    SniperAmmoMag4 = 329,
    ShotgunBolt1 = 330,
    ShotgunBolt2 = 331,
    ShotgunBolt3 = 332,
    ShotgunBolt4 = 333,
    StockRegular1 = 334,
    StockRegular2 = 335,
    StockRegular3 = 336,
    StockSniper1 = 337,
    StockSniper2 = 338,
    StockSniper3 = 339,
    TurboCharger = 340, // old
    SelectfireReceiver = 341,
    HammerPoint = 355,
    BoostedLoader = 356,
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
