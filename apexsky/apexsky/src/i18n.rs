use std::{borrow::Cow, collections::HashMap, fmt::Debug};

use crate::noobfstr as s;
use fluent::{FluentArgs, FluentBundle, FluentResource};
use strum::{EnumString, VariantNames};
use strum_macros::EnumIter;
use sys_locale::get_locale;
// Used to provide a locale for the bundle.
use unic_langid::LanguageIdentifier;

#[derive(Debug, EnumString, VariantNames, strum::Display, EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum MessageId {
    HelloWorld,
    MenuValueEnabled,
    MenuValueDisabled,
    MainMenuTitle,
    MenuItemFiringRange,
    MenuItemTdmToggle,
    MenuItemKeyboard,
    MenuItemGamepad,
    MenuItemItemGlow,
    MenuItemPlayerGlow,
    MenuItemSmoothValue,
    InputPromptSmoothValue,
    InfoInvalidSmoothValue,
    MenuItemChangeBoneAim,
    MenuValueBoneAuto,
    MenuValueBoneHitbox,
    MenuValueBoneHead,
    MenuValueBoneNeck,
    MenuValueBoneChest,
    MenuValueBoneGutShut,
    MenuValueBoneUnknown,
    InputPromptBoneValue,
    InfoInvalidBoneValue,
    InfoInvalidValue,
    MenuItemLootGlowFilled,
    MenuItemPlayerGlowFilled,
    MenuItemPlayerOutlineSize,
    InputPromptPlayerOutlines,
    InfoPlayerOutlineUpdated,
    InfoInvalidOutlineSize,
    MenuItemUpdateGlowColors,
    MenuItemChangeAdsFov,
    InputPromptAdsFov,
    InfoInvalidAdsFov,
    MenuItemChangeNonAdsFov,
    InputPromptNonAdsFov,
    InfoInvalidNonAdsFov,
    MenuItemSuperGlide,
    MenuItemItemFilterSettings,
    MenuItemHotkeySettings,
    MenuItemDeathBoxes,
    MenuItemSaveSettings,
    MenuItemLoadSettings,
    InfoSaved,
    InfoFailed,
    InfoLoaded,
    InfoFallbackConfig,
    MenuItemToggleNadeAim,
    MenuValueNoNadeAim,
    MenuValueNadeAimOn,
    MenuItemToggleOnevone,
    MenuItemToggleNoRecoil,
    MenuItemRecoilXValue,
    MenuItemRecoilYValue,
    InputPromptRecoilValue,
    InfoInvalidRecoilValue,
    MenuItemSetFpsPredict,
    MenuValueCalcFps,
    InputPromptFpsPredict,
    MenuItemBigMapFeat,
    MenuItemPlayerArmorGlowColor,
    MenuItemWeaponModelGlow,
    InfoWeaponModelGlow,
    MenuItemToggleOverlay,
    MenuValueNoOverlay,
    MenuValueExternalOverlay,
    InfoExpectingValueCount,
    InfoCannotParseInputValues,
    InfoValuesOutOfRange,
    InputPromptColorRgb,
    GlowColorMenuTitle,
    MenuItemGlowColors,
    ColorItemNotVizTarget,
    ColorItemVizTarget,
    ColorItemKnockedTarget,
    InfoGlowColorsUpdated,
    MenuItemBackToMainMenu,
    InputPromptKeycode,
    HotkeyMenuTitle,
    HotkeyItemAimbot1,
    HotkeyItemAimbot2,
    HotkeyItemTriggerBot,
    HotkeyItemQuickLooting,
    MenuItemKeyCodes,
    ItemFilterMenuTitle,
    InfoInvalidKeycode,
    ItemLightWeapons,
    ItemHeavyWeapons,
    ItemEnergyWeapons,
    ItemSniperWeapons,
    ItemArmors,
    ItemHealing,
    ItemNades,
    ItemBackpacks,
    ItemHopUps,
    ItemScopes,
    RedIsDisable,
    GreedIsEnabled,
    LightWeaponsMenuTitle,
    LightWeaponsSection,
    WeaponP2020,
    WeaponRe45,
    WeaponAlternator,
    WeaponR99,
    WeaponR301,
    WeaponM600,
    WeaponG7Scout,
    LootLightAmmo,
    LightWeaponMagsSection,
    LootLightWeaponMag,
    WeaponStocksSection,
    LootStandardStock,
    LootSniperStock,
    WeaponSuppressorsSection,
    LootWeaponSuppressors,
    WeaponLasersSection,
    LootWeaponLasers,
    LootQuickdrawHolster,
    WeaponHopUpsMenuTitle,
    WeaponHopUpsSection,
    LootTurboCharger,
    LootSkullPiecer,
    LootHammerPoints,
    LootDisruptorRounds,
    LootBoostedLoader,
    LootAnvilReceiver,
    LootDoubletapTrigger,
    LootDualShell,
    LootKineticFeeder,
    HeavyWeaponsMenuTitle,
    HeavyWeaponsSection,
    WeaponFlatline,
    WeaponHemlock,
    Weapon3030Repeater,
    WeaponRampage,
    WeaponProwler,
    WeaponCarSmg,
    LootHeavyAmmo,
    HeavyWeaponMagsSection,
    LootHeavyWeaponMag,
    EnergyWeaponsMenuTitle,
    EnergyWeaponsSection,
    WeaponLStar,
    WeaponNemesis,
    WeaponHavoc,
    WeaponDeovtion,
    WeaponTripleTake,
    WeaponVolt,
    LootEnergyAmmo,
    EnergyWeaponMagsSection,
    LootEnergyWeaponMag,
    SniperWeaponsMenuTitle,
    SniperWeaponsSection,
    WeaponWingman,
    WeaponLongbow,
    WeaponChargeRifle,
    WeaponSentinel,
    WeaponBow,
    LootSniperAmmo,
    SniperWeaponMagsSection,
    LootSniperWeaponMag,
    ArmorsMenuTitle,
    ArmorsSection,
    LootBodyShield,
    LootEvoShield,
    HelmetsSection,
    LootHelmet,
    KnockdownShieldsSection,
    LootKnockdownShield,
    HealingItemsMenuTitle,
    HealingItemsSection,
    LootAccelerant,
    LootPhoenix,
    LootSmallHealth,
    LootLargeHealth,
    LootSmallShieldBatt,
    LootLargeShieldBatt,
    NadesMenuTitle,
    NadeItemsSection,
    LootFragGrenade,
    LootArcStar,
    LootThermite,
    BackpacksMenuTitle,
    BackpacksSection,
    LootLightBackpack,
    LootMediumBackpack,
    LootHeavyBackpack,
    LootGoldBackpack,
    ScopesMenuTitle,
    ScopesSection,
    Loot1xHcog,
    Loot2xHcog,
    Loot1xHolo,
    Loot1x2xHolo,
    LootOpticThreat,
    Loot3xHcog,
    Loot2x4xAog,
    Loot6xSniperOptic,
    Loot4x8xSniperOptic,
    LootSniperThreat,
    KeyCodesMenuTitle,
    Keycode108Mouse1Left,
    Keycode109Mouse2Right,
    Keycode110Mouse3Middle,
    Keycode111Mouse4Side,
    Keycode112Mouse5Side,
    MenuItemBackToHotkeyMenu,
    MenuValuePrefix,
    MenuValueSuffix,
    LootLevel1Name,
    LootLevel2Name,
    LootLevel3Name,
    LootLevel4Name,
    LootLevel5Name,
    MenuItemFavoritePlayerGlow,
    MenuItemKbdBacklightCtrl,
    AimbotMenuTitle,
    MenuItemAimbotMode,
    MenuValueAimbotOff,
    MenuValueAimbotNoVisCheck,
    MenuValueAimbotOn,
    MenuValueAimbotAssist,
    InputPromptAimbotMode,
    MenuItemAimDist,
    InputPromptAimDist,
    MenuItemHeadshotDist,
    InputPromptHeadshotDist,
    MenuItemSkynadeSmooth,
    MenuItemSpectatorsMenu,
    SpectatorsMenuTitle,
    SpectatorsSection,
    MenuItemShotgunAutoShot,
    MenuItemSuperGrpple,
    MenuItemAutoTapstrafe,
    ApexskyMenuTitle,
    MenuItemOpenMainMenu,
    MenuItemManageDlc,
    MenuItemCreativeWorkshop,
    InfoComingSoon,
    DlcFieldLabel,
    DlcFieldDescription,
    DlcFieldPackageName,
    DlcFieldVersionName,
    DlcFieldDebug,
    DlcFieldPermissions,
    DlcInstalledVersion,
    DlcNotInstalledVersion,
    LabelDlcInstall,
    LabelDlcUninstall,
    LabelDlcRunning,
    LabelDlcStopped,
    DlcListLabelSeparator,
    DlcListLabelQuit,
    DlcListLabelMoveUp,
    DlcListLabelMoveDown,
    DlcListLabelInstall,
    DlcListLabelUninstall,
    DlcListLabelReload,
    InfoDlcInstallSuccess,
    InfoDlcUninstallSuccess,
    InfoDlcReloadSuccess,
    PermissionFieldAccessGameWorld,
    PermissionFieldAccessGameInput,
    PermissionFieldAccessGameMemory,
    PermissionFieldAccessGlobalSettings,
    PermissionFieldBluetooth,
    PermissionFieldCamera,
    PermissionFieldManageGlobalSettings,
    PermissionFieldInternet,
    PermissionFieldModifyMemory,
    PermissionFieldOverlay,
    PermissionFieldQueryAllPackages,
    PermissionFieldRecordAudio,
    PermissionFieldStorage,
    PermissionFieldSendAimbotActions,
    PermissionFieldSendInputActions,
    PermissionFieldApexInjectHighlight,
}

pub struct I18nBundle(pub FluentBundle<FluentResource>);

impl I18nBundle {
    pub fn new() -> Self {
        load_fluent_bundle()
    }

    #[inline]
    pub fn msg(&self, id: MessageId) -> Cow<str> {
        self.msg_fmt(id, None)
    }

    #[inline]
    pub fn msg_fmt(&self, id: MessageId, args: Option<&FluentArgs>) -> Cow<str> {
        let msg = self
            .0
            .get_message(&id.to_string())
            .expect("Message doesn't exist.");
        let mut errors = vec![];
        let pattern = msg.value().expect("Message has no value.");
        let value = self.0.format_pattern(&pattern, args, &mut errors);
        value
    }
}

impl Clone for I18nBundle {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl Debug for I18nBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("I18nBundle").finish()
    }
}

pub fn load_fluent_bundle() -> I18nBundle {
    let locale = get_locale().unwrap_or_else(|| String::from("en-US"));
    I18nBundle(get_bundle(&locale))
}

fn get_bundle(accept_locale: &str) -> FluentBundle<FluentResource> {
    include_flate::flate!(static FTL_ZH_TW: str from "./resource/i18n/zh-TW.ftl");
    include_flate::flate!(static FTL_ZH_CN: str from "./resource/i18n/zh-CN.ftl");
    include_flate::flate!(static FTL_EN_US: str from "./resource/i18n/en-US.ftl");
    let (locale, ftl_string) = match accept_locale {
        "zh-TW" => ("zh-TW", FTL_ZH_TW.to_owned()),
        "zh-CN" => ("zh-CN", FTL_ZH_CN.to_owned()),
        _ => ("en-US", FTL_EN_US.to_owned()),
    };
    let res = FluentResource::try_new(ftl_string).expect(s!("Failed to parse an FTL string."));

    let lang_id: LanguageIdentifier = locale.parse().expect(s!("Parsing failed."));
    let mut bundle = FluentBundle::new(vec![lang_id]);

    bundle
        .add_resource(res)
        .expect(s!("Failed to add FTL resources to the bundle."));
    bundle
}

#[macro_export]
macro_rules! i18n_msg {
    ( $bundle:expr, $message_id:ident) => {{
        use $crate::i18n::{I18nBundle, MessageId};
        I18nBundle::msg($bundle, MessageId::$message_id)
    }};
}

#[macro_export]
macro_rules! i18n_msg_format {
    ( $bundle:expr, $message_id:ident, $args:expr) => {{
        use $crate::i18n::{I18nBundle, MessageId};
        I18nBundle::msg_fmt($bundle, MessageId::$message_id, Some(&($args)))
    }};
}

#[allow(dead_code)]
pub fn get(
    accept_language: &str,
    message_ids: Vec<&'static str>,
) -> Result<HashMap<&'static str, String>, ()> {
    let bundle = get_bundle(accept_language);
    let mut result = HashMap::with_capacity(message_ids.len());
    let mut errors = vec![];
    for message_id in message_ids {
        let msg = bundle
            .get_message(message_id)
            .expect(s!("Message doesn't exist."));
        let pattern = msg.value().expect(s!("Message has no value."));
        let value = bundle.format_pattern(pattern, None, &mut errors);
        errors.clear();
        result.insert(message_id, value.to_string());
    }
    Ok(result)
}

#[test]
fn get_all_message_ids() {
    use strum::IntoEnumIterator;
    println!("{}", s!("- i18n message id -----------"));
    MessageId::iter().for_each(|i| println!("{} = ", i));
    println!("{}", s!("- i18n message id END--------"));
}
