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
 * GameVersion=v3.0.62.29
 */
pub const OFFSET_YAW: u64 = 0x221c - 0x8;
pub const OFFSET_HIGHLIGHT_SETTINGS: u64 = 0xbb24350;
pub const OFFSET_GLOW_CONTEXT_ID: u64 = 0x28c;
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
    Eva8 = 88,
    Flatline = 89,
    G7Scout = 90,
    Hemlock = 91,
    Kraber = 93,
    Lstar = 94,
    Mastiff = 96,
    Mozambique = 97,
    Prowler = 102,
    Peacekeeper = 104,
    R99 = 105,
    P2020 = 106,
    Spitfire = 107,
    TripleTake = 108,
    Wingman = 109,
    Volt = 111,
    _3030Repeater = 112,
    CarSmg = 113,
    Nemesis = 114,
    Hands = 115,
    ThrowingKnife = 166,
    GrenadeThermite = 167,
    GrenadeFrag = 168,
    GrenadeArcStar = 169,
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
        WeaponWingman = 106,
        WeaponP2020 = 111,
        WeaponRE45 = 116,
        WeaponSentinel = 122,
        WeaponBow = 127,
        Weapon3030Repeater = 129,
        WeaponNemesis = 135,
        LightAmmo = 140,
        EnergyAmmo = 141,
        ShotgunAmmo = 142,
        HeavyAmmo = 143,
        SniperAmmo = 144,
        WeaponRampage = 146,
        WeaponCARSMG = 151,
        Accelerant = 189,
        Phoenix = 190,
        HealthLarge = 191,
        HealthSmall = 192,
        ShieldBatteryLarge = 193,
        ShieldBatterySmall = 194,
        ShieldUpgradeHead1 = 195,
        ShieldUpgradeHead2 = 196,
        ShieldUpgradeHead3 = 197,
        ShieldUpgradeHead4 = 198,
        ArmorCore1 = 210,
        ArmorCore2 = 211,
        ArmorCore3 = 212,
        ArmorCore4 = 213,
        ShieldDown1 = 215,
        ShieldDown2 = 216,
        ShieldDown3 = 217,
        ShieldDown4 = 218,
        LightBackpack = 219,
        MedBackpack = 220,
        HeavyBackpack = 221,
        GoldBackpack = 222,
        ThrowingKnife = 223,
        GrenadeThermite = 224,
        GrenadeFrag = 225,
        GrenadeArcStar = 226,
        Optic1xHCOG = 227,
        Optic2xHCOG = 228,
        OpticHolo1x = 229,
        OpticHolo1x2x = 230,
        OpticThreat = 231,
        Optic3xHCOG = 232,
        Optic2x4x = 233,
        OpticSniper6x = 234,
        OpticSniper4x8x = 235,
        OpticSniperThreat = 236,
        Suppressor1 = 237,
        Suppressor2 = 238,
        Suppressor3 = 239,
        LaserSight1 = 241,
        LaserSight2 = 242,
        LaserSight3 = 243,
        LightAmmoMag1 = 244,
        LightAmmoMag2 = 245,
        LightAmmoMag3 = 246,
        LightAmmoMag4 = 247,
        HeavyAmmoMag1 = 248,
        HeavyAmmoMag2 = 249,
        HeavyAmmoMag3 = 250,
        HeavyAmmoMag4 = 251,
        EnergyAmmoMag1 = 252,
        EnergyAmmoMag2 = 253,
        EnergyAmmoMag3 = 254,
        EnergyAmmoMag4 = 255,
        SniperAmmoMag1 = 256,
        SniperAmmoMag2 = 257,
        SniperAmmoMag3 = 258,
        SniperAmmoMag4 = 259,
        ShotgunBolt1 = 260,
        ShotgunBolt2 = 261,
        ShotgunBolt3 = 262,
        ShotgunBolt4 = 263,
        StockRegular1 = 264,
        StockRegular2 = 265,
        StockRegular3 = 266,
        StockSniper1 = 267,
        StockSniper2 = 268,
        StockSniper3 = 269,
        TurboCharger = 270,
        SkullPiecer = 272,
        DisruptorRounds = 275,
        HammerPoint = 276,
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

pub const HIGHLIGHT_LOOT_HEAVY: u8 = 65;
pub const HIGHLIGHT_LOOT_LIGHT: u8 = 66;
pub const HIGHLIGHT_LOOT_RED: u8 = 67;
pub const HIGHLIGHT_LOOT_BLUE: u8 = 69;
pub const HIGHLIGHT_LOOT_GREY: u8 = 70;
pub const HIGHLIGHT_LOOT_WHITE: u8 = 72;
pub const HIGHLIGHT_LOOT_ENERGY: u8 = 73;
pub const HIGHLIGHT_LOOT_PURPLE: u8 = 74;
pub const HIGHLIGHT_LOOT_GOLD: u8 = 75;
pub const HIGHLIGHT_DEATH_BOX: u8 = 88;
pub const HIGHLIGHT_PLAYER_KNOCKED: u8 = 80;
pub const HIGHLIGHT_PLAYER_VISIBLE: u8 = 81;
pub const HIGHLIGHT_PLAYER_NOTVIZ: u8 = 82;
pub const HIGHLIGHT_PLAYER_BLACK: u8 = 90;
pub const HIGHLIGHT_PLAYER_ORANGE: u8 = 91;
pub const HIGHLIGHT_PLAYER_WHITE: u8 = 92;
pub const HIGHLIGHT_PLAYER_BLUE: u8 = 93;
pub const HIGHLIGHT_PLAYER_PURPLE: u8 = 94;
pub const HIGHLIGHT_PLAYER_RED: u8 = 95;
pub const HIGHLIGHT_PLAYER_RAINBOW: u8 = 96;
