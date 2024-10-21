use super::{
    handler_toggle_settings, ratatui, GeneralMenu, GeneralMenuFormat, LootLevel, MenuBuilder,
    MenuBuilderLootsExt, MenuFormatter, MenuLevel, TerminalMenu,
};
use crate::{config, i18n_msg, lock_config};
use ratatui::{
    style::Stylize,
    text::{Line, Span},
    widgets::ListItem,
};

pub(super) fn build_item_filter_menu(
    menu_fmt: &MenuFormatter,
    _settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    MenuBuilder::new(MenuLevel::ItemFilter, menu_fmt.clone())
        .title(i18n_msg!(menu_fmt, ItemFilterMenuTitle))
        .add_item(
            menu_fmt.item_text(format!(" 1 - {}", i18n_msg!(menu_fmt, ItemLightWeapons))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::LightWeapons);
                None
            },
            (),
        )
        .add_item(
            menu_fmt.item_text(format!(" 2 - {}", i18n_msg!(menu_fmt, ItemHeavyWeapons))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::HeavyWeapons);
                None
            },
            (),
        )
        .add_item(
            menu_fmt.item_text(format!(" 3 - {}", i18n_msg!(menu_fmt, ItemEnergyWeapons))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::EnergyWeapons);
                None
            },
            (),
        )
        .add_item(
            menu_fmt.item_text(format!(" 4 - {}", i18n_msg!(menu_fmt, ItemSniperWeapons))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::SniperWeapons);
                None
            },
            (),
        )
        .add_item(
            menu_fmt.item_text(format!(" 5 - {}", i18n_msg!(menu_fmt, ItemArmors))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Armors);
                None
            },
            (),
        )
        .add_item(
            menu_fmt.item_text(format!(" 6 - {}", i18n_msg!(menu_fmt, ItemHealing))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Healing);
                None
            },
            (),
        )
        .add_item(
            menu_fmt.item_text(format!(" 7 - {}", i18n_msg!(menu_fmt, ItemNades))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Grenades);
                None
            },
            (),
        )
        .add_item(
            menu_fmt.item_text(format!(" 8 - {}", i18n_msg!(menu_fmt, ItemBackpacks))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Backpacks);
                None
            },
            (),
        )
        .add_item(
            menu_fmt.item_text(format!(" 9 - {}", i18n_msg!(menu_fmt, ItemHopUps))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::HopUps);
                None
            },
            (),
        )
        .add_item(
            menu_fmt.item_text(format!("10 - {}", i18n_msg!(menu_fmt, ItemScopes))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Scopes);
                None
            },
            (),
        )
        .add_item(
            menu_fmt.item_text(format!(
                "11 - {}",
                i18n_msg!(menu_fmt, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Main);
                None
            },
            (),
        )
        .into()
}

macro_rules! i18n_id {
    ($x:ident) => {
        crate::menu::MessageId::$x
    };
}

pub(super) fn build_light_weapons_menu(
    menu_fmt: &MenuFormatter,
    settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    MenuBuilder::new(MenuLevel::LightWeapons, menu_fmt.clone())
        .title(i18n_msg!(menu_fmt, LightWeaponsMenuTitle))
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(menu_fmt, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(menu_fmt, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, LightWeaponsSection))
        .add_dummy_item()
        .add_pick_item(
            " 1 - ",
            i18n_id!(WeaponP2020),
            settings.loot.weapon_p2020,
            handler_toggle_settings!(.loot.weapon_p2020),
        )
        .add_pick_item(
            " 2 - ",
            i18n_id!(WeaponRe45),
            settings.loot.weapon_re45,
            handler_toggle_settings!(.loot.weapon_re45),
        )
        .add_pick_item(
            " 3 - ",
            i18n_id!(WeaponAlternator),
            settings.loot.weapon_alternator,
            handler_toggle_settings!(.loot.weapon_alternator),
        )
        .add_pick_item(
            " 4 - ",
            i18n_id!(WeaponR99),
            settings.loot.weapon_r99,
            handler_toggle_settings!(.loot.weapon_r99),
        )
        .add_pick_item(
            " 5 - ",
            i18n_id!(WeaponR301),
            settings.loot.weapon_r301,
            handler_toggle_settings!(.loot.weapon_r301),
        )
        .add_pick_item(
            " 6 - ",
            i18n_id!(WeaponM600),
            settings.loot.weapon_spitfire,
            handler_toggle_settings!(.loot.weapon_spitfire),
        )
        .add_pick_item(
            " 7 - ",
            i18n_id!(WeaponG7Scout),
            settings.loot.weapon_g7_scout,
            handler_toggle_settings!(.loot.weapon_g7_scout),
        )
        .add_pick_item(
            " 8 - ",
            i18n_id!(LootLightAmmo),
            settings.loot.lightammo,
            handler_toggle_settings!(.loot.lightammo),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, LightWeaponMagsSection))
        .add_dummy_item()
        .add_colored_loot_item(
            " 9 - ",
            i18n_id!(LootLightWeaponMag),
            LootLevel::White,
            settings.loot.lightammomag1,
            handler_toggle_settings!(.loot.lightammomag1),
        )
        .add_colored_loot_item(
            "10 - ",
            i18n_id!(LootLightWeaponMag),
            LootLevel::Blue,
            settings.loot.lightammomag2,
            handler_toggle_settings!(.loot.lightammomag2),
        )
        .add_colored_loot_item(
            "11 - ",
            i18n_id!(LootLightWeaponMag),
            LootLevel::Purple,
            settings.loot.lightammomag3,
            handler_toggle_settings!(.loot.lightammomag3),
        )
        .add_colored_loot_item(
            "12 - ",
            i18n_id!(LootLightWeaponMag),
            LootLevel::Gold,
            settings.loot.lightammomag4,
            handler_toggle_settings!(.loot.lightammomag4),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, WeaponStocksSection))
        .add_dummy_item()
        .add_colored_loot_item(
            "13 - ",
            i18n_id!(LootStandardStock),
            LootLevel::White,
            settings.loot.stockregular1,
            handler_toggle_settings!(.loot.stockregular1),
        )
        .add_colored_loot_item(
            "14 - ",
            i18n_id!(LootStandardStock),
            LootLevel::Blue,
            settings.loot.stockregular2,
            handler_toggle_settings!(.loot.stockregular2),
        )
        .add_colored_loot_item(
            "15 - ",
            i18n_id!(LootStandardStock),
            LootLevel::Purple,
            settings.loot.stockregular3,
            handler_toggle_settings!(.loot.stockregular3),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, WeaponSuppressorsSection))
        .add_dummy_item()
        .add_colored_loot_item(
            "16 - ",
            i18n_id!(LootWeaponSuppressors),
            LootLevel::White,
            settings.loot.suppressor1,
            handler_toggle_settings!(.loot.suppressor1),
        )
        .add_colored_loot_item(
            "17 - ",
            i18n_id!(LootWeaponSuppressors),
            LootLevel::Blue,
            settings.loot.suppressor2,
            handler_toggle_settings!(.loot.suppressor2),
        )
        .add_colored_loot_item(
            "18 - ",
            i18n_id!(LootWeaponSuppressors),
            LootLevel::Purple,
            settings.loot.suppressor3,
            handler_toggle_settings!(.loot.suppressor3),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, WeaponLasersSection))
        .add_dummy_item()
        .add_colored_loot_item(
            "19 - ",
            i18n_id!(LootWeaponLasers),
            LootLevel::White,
            settings.loot.lasersight1,
            handler_toggle_settings!(.loot.lasersight1),
        )
        .add_colored_loot_item(
            "20 - ",
            i18n_id!(LootWeaponLasers),
            LootLevel::Blue,
            settings.loot.lasersight2,
            handler_toggle_settings!(.loot.lasersight2),
        )
        .add_colored_loot_item(
            "21 - ",
            i18n_id!(LootWeaponLasers),
            LootLevel::Purple,
            settings.loot.lasersight3,
            handler_toggle_settings!(.loot.lasersight3),
        )
        .add_colored_loot_item(
            "22 - ",
            i18n_id!(LootWeaponLasers),
            LootLevel::Gold,
            settings.loot.lasersight4,
            handler_toggle_settings!(.loot.lasersight4),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, WeaponHopUpsSection))
        .add_dummy_item()
        .add_pick_item(
            "23 - ",
            i18n_id!(LootTurboCharger),
            settings.loot.turbo_charger,
            handler_toggle_settings!(.loot.turbo_charger),
        )
        .add_pick_item(
            "24 - ",
            i18n_id!(LootSkullPiecer),
            settings.loot.skull_piecer,
            handler_toggle_settings!(.loot.skull_piecer),
        )
        .add_pick_item(
            "25 - ",
            i18n_id!(LootHammerPoints),
            settings.loot.hammer_point,
            handler_toggle_settings!(.loot.hammer_point),
        )
        .add_pick_item(
            "26 - ",
            i18n_id!(LootDisruptorRounds),
            settings.loot.disruptor_rounds,
            handler_toggle_settings!(.loot.disruptor_rounds),
        )
        .add_pick_item(
            "27 - ",
            i18n_id!(LootBoostedLoader),
            settings.loot.boosted_loader,
            handler_toggle_settings!(.loot.boosted_loader),
        )
        .add_dummy_item()
        .add_item(
            menu_fmt.item_text(format!(
                "28 - {}",
                i18n_msg!(menu_fmt, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Main);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_heavy_weapons_menu(
    menu_fmt: &MenuFormatter,
    settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    MenuBuilder::new(MenuLevel::HeavyWeapons, menu_fmt.clone())
        .title(i18n_msg!(menu_fmt, HeavyWeaponsMenuTitle))
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(menu_fmt, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(menu_fmt, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, HeavyWeaponsSection))
        .add_dummy_item()
        .add_pick_item(
            " 1 - ",
            i18n_id!(WeaponFlatline),
            settings.loot.weapon_flatline,
            handler_toggle_settings!(.loot.weapon_flatline),
        )
        .add_pick_item(
            " 2 - ",
            i18n_id!(WeaponHemlock),
            settings.loot.weapon_hemlock,
            handler_toggle_settings!(.loot.weapon_hemlock),
        )
        .add_pick_item(
            " 3 - ",
            i18n_id!(Weapon3030Repeater),
            settings.loot.weapon_3030_repeater,
            handler_toggle_settings!(.loot.weapon_3030_repeater),
        )
        .add_pick_item(
            " 4 - ",
            i18n_id!(WeaponRampage),
            settings.loot.weapon_rampage,
            handler_toggle_settings!(.loot.weapon_rampage),
        )
        .add_pick_item(
            " 5 - ",
            i18n_id!(WeaponProwler),
            settings.loot.weapon_prowler,
            handler_toggle_settings!(.loot.weapon_prowler),
        )
        .add_pick_item(
            " 6 - ",
            i18n_id!(WeaponCarSmg),
            settings.loot.weapon_car_smg,
            handler_toggle_settings!(.loot.weapon_car_smg),
        )
        .add_pick_item(
            " 7 - ",
            i18n_id!(LootHeavyAmmo),
            settings.loot.heavyammo,
            handler_toggle_settings!(.loot.heavyammo),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, HeavyWeaponMagsSection))
        .add_dummy_item()
        .add_colored_loot_item(
            " 8 - ",
            i18n_id!(LootHeavyWeaponMag),
            LootLevel::White,
            settings.loot.heavyammomag1,
            handler_toggle_settings!(.loot.heavyammomag1),
        )
        .add_colored_loot_item(
            " 9 - ",
            i18n_id!(LootHeavyWeaponMag),
            LootLevel::Blue,
            settings.loot.heavyammomag2,
            handler_toggle_settings!(.loot.heavyammomag2),
        )
        .add_colored_loot_item(
            "10 - ",
            i18n_id!(LootHeavyWeaponMag),
            LootLevel::Purple,
            settings.loot.heavyammomag3,
            handler_toggle_settings!(.loot.heavyammomag3),
        )
        .add_colored_loot_item(
            "11 - ",
            i18n_id!(LootHeavyWeaponMag),
            LootLevel::Gold,
            settings.loot.heavyammomag4,
            handler_toggle_settings!(.loot.heavyammomag4),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, WeaponStocksSection))
        .add_dummy_item()
        .add_colored_loot_item(
            "12 - ",
            i18n_id!(LootStandardStock),
            LootLevel::White,
            settings.loot.stockregular1,
            handler_toggle_settings!(.loot.stockregular1),
        )
        .add_colored_loot_item(
            "13 - ",
            i18n_id!(LootStandardStock),
            LootLevel::Blue,
            settings.loot.stockregular2,
            handler_toggle_settings!(.loot.stockregular2),
        )
        .add_colored_loot_item(
            "14 - ",
            i18n_id!(LootStandardStock),
            LootLevel::Purple,
            settings.loot.stockregular3,
            handler_toggle_settings!(.loot.stockregular3),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, WeaponSuppressorsSection))
        .add_dummy_item()
        .add_colored_loot_item(
            "15 - ",
            i18n_id!(LootWeaponSuppressors),
            LootLevel::White,
            settings.loot.suppressor1,
            handler_toggle_settings!(.loot.suppressor1),
        )
        .add_colored_loot_item(
            "16 - ",
            i18n_id!(LootWeaponSuppressors),
            LootLevel::Blue,
            settings.loot.suppressor2,
            handler_toggle_settings!(.loot.suppressor2),
        )
        .add_colored_loot_item(
            "17 - ",
            i18n_id!(LootWeaponSuppressors),
            LootLevel::Purple,
            settings.loot.suppressor3,
            handler_toggle_settings!(.loot.suppressor3),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, WeaponLasersSection))
        .add_dummy_item()
        .add_colored_loot_item(
            "18 - ",
            i18n_id!(LootWeaponLasers),
            LootLevel::White,
            settings.loot.lasersight1,
            handler_toggle_settings!(.loot.lasersight1),
        )
        .add_colored_loot_item(
            "19 - ",
            i18n_id!(LootWeaponLasers),
            LootLevel::Blue,
            settings.loot.lasersight2,
            handler_toggle_settings!(.loot.lasersight2),
        )
        .add_colored_loot_item(
            "20 - ",
            i18n_id!(LootWeaponLasers),
            LootLevel::Purple,
            settings.loot.lasersight3,
            handler_toggle_settings!(.loot.lasersight3),
        )
        .add_colored_loot_item(
            "21 - ",
            i18n_id!(LootWeaponLasers),
            LootLevel::Gold,
            settings.loot.lasersight4,
            handler_toggle_settings!(.loot.lasersight4),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, WeaponHopUpsSection))
        .add_dummy_item()
        .add_pick_item(
            "22 - ",
            i18n_id!(LootTurboCharger),
            settings.loot.turbo_charger,
            handler_toggle_settings!(.loot.turbo_charger),
        )
        .add_pick_item(
            "23 - ",
            i18n_id!(LootSkullPiecer),
            settings.loot.skull_piecer,
            handler_toggle_settings!(.loot.skull_piecer),
        )
        .add_pick_item(
            "24 - ",
            i18n_id!(LootHammerPoints),
            settings.loot.hammer_point,
            handler_toggle_settings!(.loot.hammer_point),
        )
        .add_pick_item(
            "25 - ",
            i18n_id!(LootDisruptorRounds),
            settings.loot.disruptor_rounds,
            handler_toggle_settings!(.loot.disruptor_rounds),
        )
        .add_pick_item(
            "26 - ",
            i18n_id!(LootBoostedLoader),
            settings.loot.boosted_loader,
            handler_toggle_settings!(.loot.boosted_loader),
        )
        .add_dummy_item()
        .add_item(
            menu_fmt.item_text(format!(
                "27 - {}",
                i18n_msg!(menu_fmt, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Main);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_energy_weapons_menu(
    menu_fmt: &MenuFormatter,
    settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    MenuBuilder::new(MenuLevel::EnergyWeapons, menu_fmt.clone())
        .title(i18n_msg!(menu_fmt, EnergyWeaponsMenuTitle))
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(menu_fmt, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(menu_fmt, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, EnergyWeaponsSection))
        .add_dummy_item()
        .add_pick_item(
            " 1 - ",
            i18n_id!(WeaponLStar),
            settings.loot.weapon_lstar,
            handler_toggle_settings!(.loot.weapon_lstar),
        )
        .add_pick_item(
            " 2 - ",
            i18n_id!(WeaponNemesis),
            settings.loot.weapon_nemesis,
            handler_toggle_settings!(.loot.weapon_nemesis),
        )
        .add_pick_item(
            " 3 - ",
            i18n_id!(WeaponHavoc),
            settings.loot.weapon_havoc,
            handler_toggle_settings!(.loot.weapon_havoc),
        )
        .add_pick_item(
            " 4 - ",
            i18n_id!(WeaponDeovtion),
            settings.loot.weapon_devotion,
            handler_toggle_settings!(.loot.weapon_devotion),
        )
        .add_pick_item(
            " 5 - ",
            i18n_id!(WeaponTripleTake),
            settings.loot.weapon_triple_take,
            handler_toggle_settings!(.loot.weapon_triple_take),
        )
        .add_pick_item(
            " 6 - ",
            i18n_id!(WeaponVolt),
            settings.loot.weapon_volt,
            handler_toggle_settings!(.loot.weapon_volt),
        )
        .add_pick_item(
            " 7 - ",
            i18n_id!(LootEnergyAmmo),
            settings.loot.energyammo,
            handler_toggle_settings!(.loot.energyammo),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, EnergyWeaponMagsSection))
        .add_dummy_item()
        .add_colored_loot_item(
            " 8 - ",
            i18n_id!(LootEnergyWeaponMag),
            LootLevel::White,
            settings.loot.energyammomag1,
            handler_toggle_settings!(.loot.energyammomag1),
        )
        .add_colored_loot_item(
            " 9 - ",
            i18n_id!(LootEnergyWeaponMag),
            LootLevel::Blue,
            settings.loot.energyammomag2,
            handler_toggle_settings!(.loot.energyammomag2),
        )
        .add_colored_loot_item(
            "10 - ",
            i18n_id!(LootEnergyWeaponMag),
            LootLevel::Purple,
            settings.loot.energyammomag3,
            handler_toggle_settings!(.loot.energyammomag3),
        )
        .add_colored_loot_item(
            "11 - ",
            i18n_id!(LootEnergyWeaponMag),
            LootLevel::Gold,
            settings.loot.energyammomag4,
            handler_toggle_settings!(.loot.energyammomag4),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, WeaponStocksSection))
        .add_dummy_item()
        .add_colored_loot_item(
            "12 - ",
            i18n_id!(LootStandardStock),
            LootLevel::White,
            settings.loot.stockregular1,
            handler_toggle_settings!(.loot.stockregular1),
        )
        .add_colored_loot_item(
            "13 - ",
            i18n_id!(LootStandardStock),
            LootLevel::Blue,
            settings.loot.stockregular2,
            handler_toggle_settings!(.loot.stockregular2),
        )
        .add_colored_loot_item(
            "14 - ",
            i18n_id!(LootStandardStock),
            LootLevel::Purple,
            settings.loot.stockregular3,
            handler_toggle_settings!(.loot.stockregular3),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, WeaponSuppressorsSection))
        .add_dummy_item()
        .add_colored_loot_item(
            "15 - ",
            i18n_id!(LootWeaponSuppressors),
            LootLevel::White,
            settings.loot.suppressor1,
            handler_toggle_settings!(.loot.suppressor1),
        )
        .add_colored_loot_item(
            "16 - ",
            i18n_id!(LootWeaponSuppressors),
            LootLevel::Blue,
            settings.loot.suppressor2,
            handler_toggle_settings!(.loot.suppressor2),
        )
        .add_colored_loot_item(
            "17 - ",
            i18n_id!(LootWeaponSuppressors),
            LootLevel::Purple,
            settings.loot.suppressor3,
            handler_toggle_settings!(.loot.suppressor3),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, WeaponLasersSection))
        .add_dummy_item()
        .add_colored_loot_item(
            "18 - ",
            i18n_id!(LootWeaponLasers),
            LootLevel::White,
            settings.loot.lasersight1,
            handler_toggle_settings!(.loot.lasersight1),
        )
        .add_colored_loot_item(
            "19 - ",
            i18n_id!(LootWeaponLasers),
            LootLevel::Blue,
            settings.loot.lasersight2,
            handler_toggle_settings!(.loot.lasersight2),
        )
        .add_colored_loot_item(
            "20 - ",
            i18n_id!(LootWeaponLasers),
            LootLevel::Purple,
            settings.loot.lasersight3,
            handler_toggle_settings!(.loot.lasersight3),
        )
        .add_colored_loot_item(
            "21 - ",
            i18n_id!(LootWeaponLasers),
            LootLevel::Gold,
            settings.loot.lasersight4,
            handler_toggle_settings!(.loot.lasersight4),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, WeaponHopUpsSection))
        .add_dummy_item()
        .add_pick_item(
            "22 - ",
            i18n_id!(LootTurboCharger),
            settings.loot.turbo_charger,
            handler_toggle_settings!(.loot.turbo_charger),
        )
        .add_pick_item(
            "23 - ",
            i18n_id!(LootSkullPiecer),
            settings.loot.skull_piecer,
            handler_toggle_settings!(.loot.skull_piecer),
        )
        .add_pick_item(
            "24 - ",
            i18n_id!(LootHammerPoints),
            settings.loot.hammer_point,
            handler_toggle_settings!(.loot.hammer_point),
        )
        .add_pick_item(
            "25 - ",
            i18n_id!(LootDisruptorRounds),
            settings.loot.disruptor_rounds,
            handler_toggle_settings!(.loot.disruptor_rounds),
        )
        .add_pick_item(
            "26 - ",
            i18n_id!(LootBoostedLoader),
            settings.loot.boosted_loader,
            handler_toggle_settings!(.loot.boosted_loader),
        )
        .add_dummy_item()
        .add_item(
            menu_fmt.item_text(format!(
                "27 - {}",
                i18n_msg!(menu_fmt, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Main);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_sniper_weapons_menu(
    menu_fmt: &MenuFormatter,
    settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    MenuBuilder::new(MenuLevel::SniperWeapons, menu_fmt.clone())
        .title(i18n_msg!(menu_fmt, SniperWeaponsMenuTitle))
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(menu_fmt, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(menu_fmt, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, SniperWeaponsSection))
        .add_dummy_item()
        .add_pick_item(
            " 1 - ",
            i18n_id!(WeaponWingman),
            settings.loot.weapon_wingman,
            handler_toggle_settings!(.loot.weapon_wingman),
        )
        .add_pick_item(
            " 2 - ",
            i18n_id!(WeaponLongbow),
            settings.loot.weapon_longbow,
            handler_toggle_settings!(.loot.weapon_longbow),
        )
        .add_pick_item(
            " 3 - ",
            i18n_id!(WeaponChargeRifle),
            settings.loot.weapon_charge_rifle,
            handler_toggle_settings!(.loot.weapon_charge_rifle),
        )
        .add_pick_item(
            " 4 - ",
            i18n_id!(WeaponSentinel),
            settings.loot.weapon_sentinel,
            handler_toggle_settings!(.loot.weapon_sentinel),
        )
        .add_pick_item(
            " 5 - ",
            i18n_id!(WeaponBow),
            settings.loot.weapon_bow,
            handler_toggle_settings!(.loot.weapon_bow),
        )
        .add_pick_item(
            " 6 - ",
            i18n_id!(LootSniperAmmo),
            settings.loot.sniperammo,
            handler_toggle_settings!(.loot.sniperammo),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, SniperWeaponMagsSection))
        .add_dummy_item()
        .add_colored_loot_item(
            " 7 - ",
            i18n_id!(LootSniperWeaponMag),
            LootLevel::White,
            settings.loot.sniperammomag1,
            handler_toggle_settings!(.loot.sniperammomag1),
        )
        .add_colored_loot_item(
            " 8 - ",
            i18n_id!(LootSniperWeaponMag),
            LootLevel::Blue,
            settings.loot.sniperammomag2,
            handler_toggle_settings!(.loot.sniperammomag2),
        )
        .add_colored_loot_item(
            " 9 - ",
            i18n_id!(LootSniperWeaponMag),
            LootLevel::Purple,
            settings.loot.sniperammomag3,
            handler_toggle_settings!(.loot.sniperammomag3),
        )
        .add_colored_loot_item(
            "10 - ",
            i18n_id!(LootSniperWeaponMag),
            LootLevel::Gold,
            settings.loot.sniperammomag4,
            handler_toggle_settings!(.loot.sniperammomag4),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, WeaponStocksSection))
        .add_dummy_item()
        .add_colored_loot_item(
            "11 - ",
            i18n_id!(LootSniperStock),
            LootLevel::White,
            settings.loot.stocksniper1,
            handler_toggle_settings!(.loot.stocksniper1),
        )
        .add_colored_loot_item(
            "12 - ",
            i18n_id!(LootSniperStock),
            LootLevel::Blue,
            settings.loot.stocksniper2,
            handler_toggle_settings!(.loot.stocksniper2),
        )
        .add_colored_loot_item(
            "13 - ",
            i18n_id!(LootSniperStock),
            LootLevel::Purple,
            settings.loot.stocksniper3,
            handler_toggle_settings!(.loot.stocksniper3),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, WeaponSuppressorsSection))
        .add_dummy_item()
        .add_colored_loot_item(
            "14 - ",
            i18n_id!(LootWeaponSuppressors),
            LootLevel::White,
            settings.loot.suppressor1,
            handler_toggle_settings!(.loot.suppressor1),
        )
        .add_colored_loot_item(
            "15 - ",
            i18n_id!(LootWeaponSuppressors),
            LootLevel::Blue,
            settings.loot.suppressor2,
            handler_toggle_settings!(.loot.suppressor2),
        )
        .add_colored_loot_item(
            "16 - ",
            i18n_id!(LootWeaponSuppressors),
            LootLevel::Purple,
            settings.loot.suppressor3,
            handler_toggle_settings!(.loot.suppressor3),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, WeaponHopUpsSection))
        .add_dummy_item()
        .add_pick_item(
            "17 - ",
            i18n_id!(LootTurboCharger),
            settings.loot.turbo_charger,
            handler_toggle_settings!(.loot.turbo_charger),
        )
        .add_pick_item(
            "18 - ",
            i18n_id!(LootSkullPiecer),
            settings.loot.skull_piecer,
            handler_toggle_settings!(.loot.skull_piecer),
        )
        .add_pick_item(
            "19 - ",
            i18n_id!(LootHammerPoints),
            settings.loot.hammer_point,
            handler_toggle_settings!(.loot.hammer_point),
        )
        .add_pick_item(
            "20 - ",
            i18n_id!(LootDisruptorRounds),
            settings.loot.disruptor_rounds,
            handler_toggle_settings!(.loot.disruptor_rounds),
        )
        .add_pick_item(
            "21 - ",
            i18n_id!(LootBoostedLoader),
            settings.loot.boosted_loader,
            handler_toggle_settings!(.loot.boosted_loader),
        )
        .add_dummy_item()
        .add_item(
            menu_fmt.item_text(format!(
                "22 - {}",
                i18n_msg!(menu_fmt, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Main);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_armors_menu(
    menu_fmt: &MenuFormatter,
    settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    MenuBuilder::new(MenuLevel::Armors, menu_fmt.clone())
        .title(i18n_msg!(menu_fmt, ArmorsMenuTitle))
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(menu_fmt, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(menu_fmt, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, ArmorsSection))
        .add_dummy_item()
        .add_colored_loot_item(
            " 1 - ",
            i18n_id!(LootEvoShield),
            LootLevel::White,
            settings.loot.shieldupgrade1,
            handler_toggle_settings!(.loot.shieldupgrade1),
        )
        .add_colored_loot_item(
            " 2 - ",
            i18n_id!(LootEvoShield),
            LootLevel::Blue,
            settings.loot.shieldupgrade2,
            handler_toggle_settings!(.loot.shieldupgrade2),
        )
        .add_colored_loot_item(
            " 3 - ",
            i18n_id!(LootEvoShield),
            LootLevel::Purple,
            settings.loot.shieldupgrade3,
            handler_toggle_settings!(.loot.shieldupgrade3),
        )
        .add_colored_loot_item(
            " 4 - ",
            i18n_id!(LootBodyShield),
            LootLevel::Gold,
            settings.loot.shieldupgrade4,
            handler_toggle_settings!(.loot.shieldupgrade4),
        )
        .add_colored_loot_item(
            " 5 - ",
            i18n_id!(LootEvoShield),
            LootLevel::Red,
            settings.loot.shieldupgrade5,
            handler_toggle_settings!(.loot.shieldupgrade5),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, HelmetsSection))
        .add_dummy_item()
        .add_colored_loot_item(
            " 6 - ",
            i18n_id!(LootHelmet),
            LootLevel::White,
            settings.loot.shieldupgradehead1,
            handler_toggle_settings!(.loot.shieldupgradehead1),
        )
        .add_colored_loot_item(
            " 7 - ",
            i18n_id!(LootHelmet),
            LootLevel::Blue,
            settings.loot.shieldupgradehead2,
            handler_toggle_settings!(.loot.shieldupgradehead2),
        )
        .add_colored_loot_item(
            " 8 - ",
            i18n_id!(LootHelmet),
            LootLevel::Purple,
            settings.loot.shieldupgradehead3,
            handler_toggle_settings!(.loot.shieldupgradehead3),
        )
        .add_colored_loot_item(
            " 9 - ",
            i18n_id!(LootHelmet),
            LootLevel::Gold,
            settings.loot.shieldupgradehead4,
            handler_toggle_settings!(.loot.shieldupgradehead4),
        )
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, KnockdownShieldsSection))
        .add_dummy_item()
        .add_colored_loot_item(
            "10 - ",
            i18n_id!(LootKnockdownShield),
            LootLevel::White,
            settings.loot.shielddown1,
            handler_toggle_settings!(.loot.shielddown1),
        )
        .add_colored_loot_item(
            "11 - ",
            i18n_id!(LootKnockdownShield),
            LootLevel::Blue,
            settings.loot.shielddown2,
            handler_toggle_settings!(.loot.shielddown2),
        )
        .add_colored_loot_item(
            "12 - ",
            i18n_id!(LootKnockdownShield),
            LootLevel::Purple,
            settings.loot.shielddown3,
            handler_toggle_settings!(.loot.shielddown3),
        )
        .add_colored_loot_item(
            "13 - ",
            i18n_id!(LootKnockdownShield),
            LootLevel::Gold,
            settings.loot.shielddown4,
            handler_toggle_settings!(.loot.shielddown4),
        )
        .add_dummy_item()
        .add_item(
            menu_fmt.item_text(format!(
                "14 - {}",
                i18n_msg!(menu_fmt, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Main);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_healing_menu(
    menu_fmt: &MenuFormatter,
    settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    MenuBuilder::new(MenuLevel::Healing, menu_fmt.clone())
        .title(i18n_msg!(menu_fmt, HealingItemsMenuTitle))
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(menu_fmt, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(menu_fmt, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, HealingItemsSection))
        .add_dummy_item()
        .add_colored_loot_item(
            " 1 - ",
            i18n_id!(LootAccelerant),
            LootLevel::Blue,
            settings.loot.accelerant,
            handler_toggle_settings!(.loot.accelerant),
        )
        .add_colored_loot_item(
            " 2 - ",
            i18n_id!(LootPhoenix),
            LootLevel::Purple,
            settings.loot.phoenix,
            handler_toggle_settings!(.loot.phoenix),
        )
        .add_colored_loot_item(
            " 3 - ",
            i18n_id!(LootSmallHealth),
            LootLevel::White,
            settings.loot.healthsmall,
            handler_toggle_settings!(.loot.healthsmall),
        )
        .add_colored_loot_item(
            " 4 - ",
            i18n_id!(LootLargeHealth),
            LootLevel::White,
            settings.loot.healthlarge,
            handler_toggle_settings!(.loot.healthlarge),
        )
        .add_colored_loot_item(
            " 5 - ",
            i18n_id!(LootSmallShieldBatt),
            LootLevel::White,
            settings.loot.shieldbattsmall,
            handler_toggle_settings!(.loot.shieldbattsmall),
        )
        .add_colored_loot_item(
            " 6 - ",
            i18n_id!(LootLargeShieldBatt),
            LootLevel::White,
            settings.loot.shieldbattlarge,
            handler_toggle_settings!(.loot.shieldbattlarge),
        )
        .add_dummy_item()
        .add_item(
            menu_fmt.item_text(format!(
                "7 - {}",
                i18n_msg!(menu_fmt, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Main);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_nades_menu(
    menu_fmt: &MenuFormatter,
    settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    MenuBuilder::new(MenuLevel::Grenades, menu_fmt.clone())
        .title(i18n_msg!(menu_fmt, NadesMenuTitle))
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(menu_fmt, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(menu_fmt, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, NadeItemsSection))
        .add_dummy_item()
        .add_colored_loot_item(
            " 1 - ",
            i18n_id!(LootFragGrenade),
            LootLevel::Red,
            settings.loot.grenade_frag,
            handler_toggle_settings!(.loot.grenade_frag),
        )
        .add_colored_loot_item(
            " 2 - ",
            i18n_id!(LootArcStar),
            LootLevel::Blue,
            settings.loot.grenade_arc_star,
            handler_toggle_settings!(.loot.grenade_arc_star),
        )
        .add_colored_loot_item(
            " 3 - ",
            i18n_id!(LootThermite),
            LootLevel::Red,
            settings.loot.grenade_thermite,
            handler_toggle_settings!(.loot.grenade_thermite),
        )
        .add_dummy_item()
        .add_item(
            menu_fmt.item_text(format!(
                "4 - {}",
                i18n_msg!(menu_fmt, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Main);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_backpacks_menu(
    menu_fmt: &MenuFormatter,
    settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    MenuBuilder::new(MenuLevel::Backpacks, menu_fmt.clone())
        .title(i18n_msg!(menu_fmt, BackpacksMenuTitle))
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(menu_fmt, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(menu_fmt, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, BackpacksSection))
        .add_dummy_item()
        .add_colored_loot_item(
            " 1 - ",
            i18n_id!(LootLightBackpack),
            LootLevel::White,
            settings.loot.lightbackpack,
            handler_toggle_settings!(.loot.lightbackpack),
        )
        .add_colored_loot_item(
            " 2 - ",
            i18n_id!(LootMediumBackpack),
            LootLevel::Blue,
            settings.loot.medbackpack,
            handler_toggle_settings!(.loot.       medbackpack),
        )
        .add_colored_loot_item(
            " 3 - ",
            i18n_id!(LootHeavyBackpack),
            LootLevel::Purple,
            settings.loot.heavybackpack,
            handler_toggle_settings!(.loot.heavybackpack),
        )
        .add_colored_loot_item(
            " 4 - ",
            i18n_id!(LootGoldBackpack),
            LootLevel::Gold,
            settings.loot.goldbackpack,
            handler_toggle_settings!(.loot.       goldbackpack),
        )
        .add_dummy_item()
        .add_item(
            menu_fmt.item_text(format!(
                "5 - {}",
                i18n_msg!(menu_fmt, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Main);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_hopups_menu(
    menu_fmt: &MenuFormatter,
    settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    MenuBuilder::new(MenuLevel::HopUps, menu_fmt.clone())
        .title(i18n_msg!(menu_fmt, WeaponHopUpsMenuTitle))
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(menu_fmt, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(menu_fmt, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, WeaponHopUpsSection))
        .add_dummy_item()
        .add_pick_item(
            " 1 - ",
            i18n_id!(LootTurboCharger),
            settings.loot.turbo_charger,
            handler_toggle_settings!(.loot.turbo_charger),
        )
        .add_pick_item(
            " 2 - ",
            i18n_id!(LootSkullPiecer),
            settings.loot.skull_piecer,
            handler_toggle_settings!(.loot.skull_piecer),
        )
        .add_pick_item(
            " 3 - ",
            i18n_id!(LootHammerPoints),
            settings.loot.hammer_point,
            handler_toggle_settings!(.loot.hammer_point),
        )
        .add_pick_item(
            " 4 - ",
            i18n_id!(LootDisruptorRounds),
            settings.loot.disruptor_rounds,
            handler_toggle_settings!(.loot.disruptor_rounds),
        )
        .add_pick_item(
            " 5 - ",
            i18n_id!(LootBoostedLoader),
            settings.loot.boosted_loader,
            handler_toggle_settings!(.loot.boosted_loader),
        )
        .add_pick_item(
            " 6 - ",
            i18n_id!(LootAnvilReceiver),
            settings.loot.anvil_receiver,
            handler_toggle_settings!(.loot.anvil_receiver),
        )
        .add_pick_item(
            " 7 - ",
            i18n_id!(LootDoubletapTrigger),
            settings.loot.doubletap_trigger,
            handler_toggle_settings!(.loot.doubletap_trigger),
        )
        .add_pick_item(
            " 8 - ",
            i18n_id!(LootDualShell),
            settings.loot.dual_shell,
            handler_toggle_settings!(.loot.dual_shell),
        )
        .add_pick_item(
            " 9 - ",
            i18n_id!(LootKineticFeeder),
            settings.loot.kinetic_feeder,
            handler_toggle_settings!(.loot.kinetic_feeder),
        )
        .add_pick_item(
            "10 - ",
            i18n_id!(LootQuickdrawHolster),
            settings.loot.quickdraw_holster,
            handler_toggle_settings!(.loot.quickdraw_holster),
        )
        .add_pick_item(
            "11 - ",
            i18n_id!(LootGunShieldGenerator),
            settings.loot.gun_shield_generator,
            handler_toggle_settings!(.loot.gun_shield_generator),
        )
        .add_pick_item(
            "12 - ",
            i18n_id!(LootSelectfireReceiver),
            settings.loot.selectfire_receiver,
            handler_toggle_settings!(.loot.selectfire_receiver),
        )
        .add_dummy_item()
        .add_item(
            menu_fmt.item_text(format!(
                "13 - {}",
                i18n_msg!(menu_fmt, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Main);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_scopes_menu(
    menu_fmt: &MenuFormatter,
    settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    MenuBuilder::new(MenuLevel::Scopes, menu_fmt.clone())
        .title(i18n_msg!(menu_fmt, ScopesMenuTitle))
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(menu_fmt, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(menu_fmt, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(menu_fmt, ScopesSection))
        .add_dummy_item()
        .add_colored_loot_item(
            " 1 - ",
            i18n_id!(Loot1xHcog),
            LootLevel::White,
            settings.loot.optic1xhcog,
            handler_toggle_settings!(.loot.optic1xhcog),
        )
        .add_colored_loot_item(
            " 2 - ",
            i18n_id!(Loot2xHcog),
            LootLevel::Blue,
            settings.loot.optic2xhcog,
            handler_toggle_settings!(.loot.optic2xhcog),
        )
        .add_colored_loot_item(
            " 3 - ",
            i18n_id!(Loot1xHolo),
            LootLevel::White,
            settings.loot.opticholo1x,
            handler_toggle_settings!(.loot.opticholo1x),
        )
        .add_colored_loot_item(
            " 4 - ",
            i18n_id!(Loot1x2xHolo),
            LootLevel::Blue,
            settings.loot.opticholo1x2x,
            handler_toggle_settings!(.loot.opticholo1x2x),
        )
        .add_colored_loot_item(
            " 5 - ",
            i18n_id!(LootOpticThreat),
            LootLevel::Gold,
            settings.loot.opticthreat,
            handler_toggle_settings!(.loot.opticthreat),
        )
        .add_colored_loot_item(
            " 6 - ",
            i18n_id!(Loot3xHcog),
            LootLevel::Purple,
            settings.loot.optic3xhcog,
            handler_toggle_settings!(.loot.optic3xhcog),
        )
        .add_colored_loot_item(
            " 7 - ",
            i18n_id!(Loot2x4xAog),
            LootLevel::Purple,
            settings.loot.optic2x4x,
            handler_toggle_settings!(.loot.optic2x4x),
        )
        .add_colored_loot_item(
            " 8 - ",
            i18n_id!(Loot6xSniperOptic),
            LootLevel::Blue,
            settings.loot.opticsniper6x,
            handler_toggle_settings!(.loot.opticsniper6x),
        )
        .add_colored_loot_item(
            " 9 - ",
            i18n_id!(Loot4x8xSniperOptic),
            LootLevel::Purple,
            settings.loot.opticsniper4x8x,
            handler_toggle_settings!(.loot.opticsniper4x8x),
        )
        .add_colored_loot_item(
            "10 - ",
            i18n_id!(LootSniperThreat),
            LootLevel::Gold,
            settings.loot.opticsniperthreat,
            handler_toggle_settings!(.loot.opticsniperthreat),
        )
        .add_dummy_item()
        .add_item(
            menu_fmt.item_text(format!(
                "11 - {}",
                i18n_msg!(menu_fmt, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Main);
                None
            },
            (),
        )
        .into()
}
