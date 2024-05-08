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
 * GameVersion=v3.0.67.34
 */
pub const OFFSET_YAW: u64 = 0x223c - 0x8;
pub const OFFSET_HIGHLIGHT_SETTINGS: u64 = 0xade39c0;
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
    Alternator = 81,
    Re45 = 82,
    ChargeRifle = 84,
    Devotion = 85,
    Longbow = 86,
    Havoc = 87,
    Eva8 = 88,
    Flatline = 90,
    G7Scout = 91,
    Hemlock = 92,
    Kraber = 94,
    Lstar = 95,
    Mastiff = 97,
    Mozambique = 98,
    Prowler = 103,
    Peacekeeper = 105,
    R99 = 106,
    P2020 = 107,
    Spitfire = 108,
    TripleTake = 109,
    Wingman = 110,
    Volt = 112,
    _3030Repeater = 113,
    CarSmg = 114,
    Nemesis = 115,
    Hands = 116,
    ThrowingKnife = 169,
    GrenadeThermite = 170,
    GrenadeFrag = 171,
    GrenadeArcStar = 172,
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
        WeaponDevotion = 18,
        WeaponTripleTake = 23,
        WeaponFlatline = 28,
        WeaponHemlock = 33,
        WeaponG7Scout = 39,
        WeaponAlternator = 44,
        WeaponR99 = 49,
        WeaponProwler = 54,
        WeaponVolt = 60,
        WeaponLongbow = 65,
        WeaponChargeRifle = 70,
        WeaponSpitfire = 75,
        WeaponR301 = 80,
        WeaponEva8 = 85,
        WeaponPeacekeeper = 91,
        WeaponMozambique = 96,
        WeaponWingman = 107,
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

const HL_OFF: u8 = 50;
pub const HIGHLIGHT_LOOT_HEAVY: u8 = HL_OFF + 0;
pub const HIGHLIGHT_LOOT_LIGHT: u8 = HL_OFF + 1;
pub const HIGHLIGHT_LOOT_RED: u8 = HL_OFF + 2;
pub const HIGHLIGHT_LOOT_BLUE: u8 = HL_OFF + 3;
pub const HIGHLIGHT_LOOT_GREY: u8 = HL_OFF + 4;
pub const HIGHLIGHT_LOOT_WHITE: u8 = HL_OFF + 5;
pub const HIGHLIGHT_LOOT_ENERGY: u8 = HL_OFF + 6;
pub const HIGHLIGHT_LOOT_PURPLE: u8 = HL_OFF + 7;
pub const HIGHLIGHT_LOOT_GOLD: u8 = HL_OFF + 8;
pub const HIGHLIGHT_DEATH_BOX: u8 = HL_OFF + 9;
pub const HIGHLIGHT_PLAYER_KNOCKED: u8 = HL_OFF + 10;
pub const HIGHLIGHT_PLAYER_VISIBLE: u8 = HL_OFF + 11;
pub const HIGHLIGHT_PLAYER_NOTVIZ: u8 = HL_OFF + 12;
pub const HIGHLIGHT_PLAYER_BLACK: u8 = HL_OFF + 13;
pub const HIGHLIGHT_PLAYER_ORANGE: u8 = HL_OFF + 14;
pub const HIGHLIGHT_PLAYER_WHITE: u8 = HL_OFF + 15;
pub const HIGHLIGHT_PLAYER_BLUE: u8 = HL_OFF + 16;
pub const HIGHLIGHT_PLAYER_PURPLE: u8 = HL_OFF + 17;
pub const HIGHLIGHT_PLAYER_RED: u8 = HL_OFF + 18;
pub const HIGHLIGHT_PLAYER_RAINBOW: u8 = HL_OFF + 19;
