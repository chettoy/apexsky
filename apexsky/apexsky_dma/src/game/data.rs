macro_rules! define_item_id {
    ($(#[$attr:meta])* $vis:vis enum $name:ident {
        $($variant:ident = $value:expr),*,
    }) => {
        $(#[$attr])*
        $vis enum $name {
            $($variant = $value),*,
        }

        impl TryFrom<u64> for $name {
            type Error = ();

            fn try_from(value: u64) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok(Self::$variant),)*
                    _ => Err(()),
                }
            }
        }

        impl Into<u64> for $name {
            fn into(self) -> u64 {
                self as u64
            }
        }
    };
}

/*
 * GameVersion=v3.0.71.58
 */
pub const OFFSET_YAW: u64 = 0x223c - 0x8;
pub const OFFSET_SPECTATOR_LIST: u64 = 0x1eadd28;
pub const OFFSET_HIGHLIGHT_SETTINGS: u64 = 0xade5c40;
//pub const OFFSET_GLOW_CONTEXT_ID: u64 = 0x28c;
pub const OFFSET_GLOW_VISIBLE_TYPE: u64 = 0x26c;
pub const OFFSET_GLOW_DISTANCE: u64 = 0x264;
pub const OFFSET_GLOW_FIX: u64 = 0x268;

#[repr(C)]
#[allow(dead_code)]
pub enum WeaponId {
    R301 = 0,
    Sentinel = 1,
    Bow = 2,
    R2R5 = 3,
    Rampage = 6,
    SheilaStationary = 10,
    Sheila = 56,
    Melee,
    SnipersMark = 76,
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
    P2020 = 111,
    Spitfire = 112,
    TripleTake = 113,
    Wingman = 114,
    Volt = 116,
    _3030Repeater = 117,
    CarSmg = 118,
    Nemesis = 119,
    Hands = 120,
    ThrowingKnife = 173,
    GrenadeThermite = 174,
    GrenadeFrag = 175,
    GrenadeArcStar = 176,
    Max,
}

define_item_id!(
    #[repr(C)]
    #[allow(dead_code)]
    pub enum ItemId {
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
        WeaponProwler = 55,
        WeaponVolt = 61,
        WeaponLongbow = 66,
        WeaponChargeRifle = 71,
        WeaponSpitfire = 76,
        WeaponR301 = 81,
        WeaponEva8 = 87,
        WeaponPeacekeeper = 92,
        WeaponMozambique = 97,
        WeaponWingman = 106,
        WeaponP2020 = 112,
        WeaponRE45 = 117,
        WeaponSentinel = 123,
        WeaponBow = 128,
        Weapon3030Repeater = 130,
        WeaponNemesis = 136,
        LightAmmo = 141,
        EnergyAmmo = 142,
        ShotgunAmmo = 143,
        HeavyAmmo = 144,
        SniperAmmo = 145,
        WeaponRampage = 147,
        WeaponCARSMG = 152,
        Accelerant = 190,
        Phoenix = 191,
        HealthLarge = 192,
        HealthSmall = 193,
        ShieldBatteryLarge = 194,
        ShieldBatterySmall = 195,
        ShieldUpgradeHead1 = 196,
        ShieldUpgradeHead2 = 197,
        ShieldUpgradeHead3 = 198,
        ShieldUpgradeHead4 = 199,
        ArmorCore1 = 211,
        ArmorCore2 = 212,
        ArmorCore3 = 213,
        ArmorCore4 = 214,
        ShieldDown1 = 216,
        ShieldDown2 = 217,
        ShieldDown3 = 218,
        ShieldDown4 = 219,
        LightBackpack = 220,
        MedBackpack = 221,
        HeavyBackpack = 222,
        GoldBackpack = 223,
        ThrowingKnife = 224,
        GrenadeThermite = 225,
        GrenadeFrag = 226,
        GrenadeArcStar = 227,
        Optic1xHCOG = 228,
        Optic2xHCOG = 229,
        OpticHolo1x = 230,
        OpticHolo1x2x = 231,
        OpticThreat = 232,
        Optic3xHCOG = 233,
        Optic2x4x = 234,
        OpticSniper6x = 235,
        OpticSniper4x8x = 236,
        OpticSniperThreat = 237,
        Suppressor1 = 238,
        Suppressor2 = 239,
        Suppressor3 = 240,
        LaserSight1 = 242,
        LaserSight2 = 243,
        LaserSight3 = 244,
        LightAmmoMag1 = 245,
        LightAmmoMag2 = 246,
        LightAmmoMag3 = 247,
        LightAmmoMag4 = 248,
        HeavyAmmoMag1 = 249,
        HeavyAmmoMag2 = 250,
        HeavyAmmoMag3 = 251,
        HeavyAmmoMag4 = 252,
        EnergyAmmoMag1 = 253,
        EnergyAmmoMag2 = 254,
        EnergyAmmoMag3 = 255,
        EnergyAmmoMag4 = 256,
        SniperAmmoMag1 = 257,
        SniperAmmoMag2 = 258,
        SniperAmmoMag3 = 259,
        SniperAmmoMag4 = 260,
        ShotgunBolt1 = 261,
        ShotgunBolt2 = 262,
        ShotgunBolt3 = 263,
        ShotgunBolt4 = 264,
        StockRegular1 = 265,
        StockRegular2 = 266,
        StockRegular3 = 267,
        StockSniper1 = 268,
        StockSniper2 = 269,
        StockSniper3 = 270,
        TurboCharger = 271,
        SkullPiecer = 273,
        DisruptorRounds = 276,
        HammerPoint = 277,
        BoostedLoader = 999999999,
        ShieldUpgrade1_0 = 214748364993,
        ShieldUpgrade1_1 = 14073963583897798,
        ShieldUpgrade2_0 = 322122547394,
        ShieldUpgrade2_1 = 21110945375846599,
        ShieldUpgrade3_0 = 429496729795,
        ShieldUpgrade3_1 = 52776987629977800,
        ShieldUpgrade4 = 429496729796,
        ShieldUpgrade5 = 536870912201,
        Unknown = 999999999999999999,
    }
);

pub const HIGHLIGHT_LOOT_GOLD: u8 = 15;
pub const HIGHLIGHT_LOOT_RED: u8 = 40;
pub const HIGHLIGHT_LOOT_PURPLE: u8 = 45;
pub const HIGHLIGHT_LOOT_BLUE: u8 = 52;
pub const HIGHLIGHT_LOOT_WHITE: u8 = 63;
pub const HIGHLIGHT_LOOT_ENERGY: u8 = 21;
pub const HIGHLIGHT_LOOT_HEAVY: u8 = 54;
pub const HIGHLIGHT_LOOT_LIGHT: u8 = 56;
pub const HIGHLIGHT_LOOT_GREY: u8 = 55;
pub const HIGHLIGHT_DEATH_BOX: u8 = 57;
pub const HIGHLIGHT_PLAYER_KNOCKED: u8 = 60;
pub const HIGHLIGHT_PLAYER_VISIBLE: u8 = 61;
pub const HIGHLIGHT_PLAYER_NOTVIZ: u8 = 58;
pub const HIGHLIGHT_PLAYER_ORANGE: u8 = 64;
pub const HIGHLIGHT_PLAYER_WHITE: u8 = 65;
pub const HIGHLIGHT_PLAYER_BLUE: u8 = 66;
pub const HIGHLIGHT_PLAYER_PURPLE: u8 = 67;
pub const HIGHLIGHT_PLAYER_RED: u8 = 68;
pub const HIGHLIGHT_PLAYER_RAINBOW: u8 = 69;
pub const HIGHLIGHT_PLAYER_BLACK: u8 = 70;
