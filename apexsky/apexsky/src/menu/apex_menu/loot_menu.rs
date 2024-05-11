use crate::{config, i18n_msg, lock_config, menu_add_colored_loot_item, menu_add_pick_item};
use fluent::{FluentBundle, FluentResource};
use ratatui::{
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::ListItem,
};

use super::{item_text, LootLevel, MenuBuilder, MenuLevel, MenuState, TerminalMenu};

pub(super) fn build_item_filter_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    _settings: config::Settings,
) -> MenuState<'static> {
    MenuBuilder::new()
        .title(i18n_msg!(i18n_bundle, ItemFilterMenuTitle))
        .add_item(
            item_text(format!("1 - {}", i18n_msg!(i18n_bundle, ItemLightWeapons))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::LightWeaponsMenu);
                None
            },
            (),
        )
        .add_item(
            item_text(format!("2 - {}", i18n_msg!(i18n_bundle, ItemHeavyWeapons))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::HeavyWeaponsMenu);
                None
            },
            (),
        )
        .add_item(
            item_text(format!("3 - {}", i18n_msg!(i18n_bundle, ItemEnergyWeapons))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::EnergyWeaponsMenu);
                None
            },
            (),
        )
        .add_item(
            item_text(format!("4 - {}", i18n_msg!(i18n_bundle, ItemSniperWeapons))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::SniperWeaponsMenu);
                None
            },
            (),
        )
        .add_item(
            item_text(format!("5 - {}", i18n_msg!(i18n_bundle, ItemArmors))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::ArmorsMenu);
                None
            },
            (),
        )
        .add_item(
            item_text(format!("6 - {}", i18n_msg!(i18n_bundle, ItemHealing))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::HealingMenu);
                None
            },
            (),
        )
        .add_item(
            item_text(format!("7 - {}", i18n_msg!(i18n_bundle, ItemNades))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::NadesMenu);
                None
            },
            (),
        )
        .add_item(
            item_text(format!("8 - {}", i18n_msg!(i18n_bundle, ItemBackpacks))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::BackpacksMenu);
                None
            },
            (),
        )
        .add_item(
            item_text(format!("9 - {}", i18n_msg!(i18n_bundle, ItemHopUps))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::HopUpsMenu);
                None
            },
            (),
        )
        .add_item(
            item_text(format!("10 - {}", i18n_msg!(i18n_bundle, ItemScopes))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::ScopesMenu);
                None
            },
            (),
        )
        .add_item(
            item_text(format!(
                "11 - {}",
                i18n_msg!(i18n_bundle, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_light_weapons_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    settings: config::Settings,
) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title(i18n_msg!(i18n_bundle, LightWeaponsMenuTitle));
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(i18n_bundle, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(i18n_bundle, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, LightWeaponsSection))
        .add_dummy_item();
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "1 - ",
        WeaponP2020,
        settings.loot.weapon_p2020,
        weapon_p2020
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "2 - ",
        WeaponRe45,
        settings.loot.weapon_re45,
        weapon_re45
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "3 - ",
        WeaponAlternator,
        settings.loot.weapon_alternator,
        weapon_alternator
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "4 - ",
        WeaponR99,
        settings.loot.weapon_r99,
        weapon_r99
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "5 - ",
        WeaponR301,
        settings.loot.weapon_r301,
        weapon_r301
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "6 - ",
        WeaponM600,
        settings.loot.weapon_spitfire,
        weapon_spitfire
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "7 - ",
        WeaponG7Scout,
        settings.loot.weapon_g7_scout,
        weapon_g7_scout
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "8 - ",
        LootLightAmmo,
        settings.loot.lightammo,
        lightammo
    );
    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, LightWeaponMagsSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "9 - ",
        LootLightWeaponMag,
        LootLevel::White,
        settings.loot.lightammomag1,
        lightammomag1
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "10 - ",
        LootLightWeaponMag,
        LootLevel::Blue,
        settings.loot.lightammomag2,
        lightammomag2
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "11 - ",
        LootLightWeaponMag,
        LootLevel::Purple,
        settings.loot.lightammomag3,
        lightammomag3
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "12 - ",
        LootLightWeaponMag,
        LootLevel::Gold,
        settings.loot.lightammomag4,
        lightammomag4
    );

    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, WeaponStocksSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "13 - ",
        LootStandardStock,
        LootLevel::White,
        settings.loot.stockregular1,
        stockregular1
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "14 - ",
        LootStandardStock,
        LootLevel::Blue,
        settings.loot.stockregular2,
        stockregular2
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "15 - ",
        LootStandardStock,
        LootLevel::Purple,
        settings.loot.stockregular3,
        stockregular3
    );
    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, WeaponSuppressorsSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "16 - ",
        LootWeaponSuppressors,
        LootLevel::White,
        settings.loot.suppressor1,
        suppressor1
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "17 - ",
        LootWeaponSuppressors,
        LootLevel::Blue,
        settings.loot.suppressor2,
        suppressor2
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "18 - ",
        LootWeaponSuppressors,
        LootLevel::Purple,
        settings.loot.suppressor3,
        suppressor3
    );
    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, WeaponLasersSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "19 - ",
        LootWeaponLasers,
        LootLevel::White,
        settings.loot.lasersight1,
        lasersight1
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "20 - ",
        LootWeaponLasers,
        LootLevel::Blue,
        settings.loot.lasersight2,
        lasersight2
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "21 - ",
        LootWeaponLasers,
        LootLevel::Purple,
        settings.loot.lasersight3,
        lasersight3
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "22 - ",
        LootWeaponLasers,
        LootLevel::Gold,
        settings.loot.lasersight4,
        lasersight4
    );
    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, WeaponHopUpsSection))
        .add_dummy_item();
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "23 - ",
        LootTurboCharger,
        settings.loot.turbo_charger,
        turbo_charger
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "24 - ",
        LootSkullPiecer,
        settings.loot.skull_piecer,
        skull_piecer
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "25 - ",
        LootHammerPoints,
        settings.loot.hammer_point,
        hammer_point
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "26 - ",
        LootDisruptorRounds,
        settings.loot.disruptor_rounds,
        disruptor_rounds
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "27 - ",
        LootBoostedLoader,
        settings.loot.boosted_loader,
        boosted_loader
    );
    menu.add_dummy_item()
        .add_item(
            item_text(format!(
                "28 - {}",
                i18n_msg!(i18n_bundle, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_heavy_weapons_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    settings: config::Settings,
) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title(i18n_msg!(i18n_bundle, HeavyWeaponsMenuTitle));
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(i18n_bundle, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(i18n_bundle, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, HeavyWeaponsSection))
        .add_dummy_item();
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "1 - ",
        WeaponFlatline,
        settings.loot.weapon_flatline,
        weapon_flatline
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "2 - ",
        WeaponHemlock,
        settings.loot.weapon_hemlock,
        weapon_hemlock
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "3 - ",
        Weapon3030Repeater,
        settings.loot.weapon_3030_repeater,
        weapon_3030_repeater
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "4 - ",
        WeaponRampage,
        settings.loot.weapon_rampage,
        weapon_rampage
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "5 - ",
        WeaponProwler,
        settings.loot.weapon_prowler,
        weapon_prowler
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "6 - ",
        WeaponCarSmg,
        settings.loot.weapon_car_smg,
        weapon_car_smg
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "7 - ",
        LootHeavyAmmo,
        settings.loot.heavyammo,
        heavyammo
    );
    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, HeavyWeaponMagsSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "8 - ",
        LootHeavyWeaponMag,
        LootLevel::White,
        settings.loot.heavyammomag1,
        heavyammomag1
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "9 - ",
        LootHeavyWeaponMag,
        LootLevel::Blue,
        settings.loot.heavyammomag2,
        heavyammomag2
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "10 - ",
        LootHeavyWeaponMag,
        LootLevel::Purple,
        settings.loot.heavyammomag3,
        heavyammomag3
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "11 - ",
        LootHeavyWeaponMag,
        LootLevel::Gold,
        settings.loot.heavyammomag4,
        heavyammomag4
    );

    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, WeaponStocksSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "12 - ",
        LootStandardStock,
        LootLevel::White,
        settings.loot.stockregular1,
        stockregular1
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "13 - ",
        LootStandardStock,
        LootLevel::Blue,
        settings.loot.stockregular2,
        stockregular2
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "14 - ",
        LootStandardStock,
        LootLevel::Purple,
        settings.loot.stockregular3,
        stockregular3
    );
    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, WeaponSuppressorsSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "15 - ",
        LootWeaponSuppressors,
        LootLevel::White,
        settings.loot.suppressor1,
        suppressor1
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "16 - ",
        LootWeaponSuppressors,
        LootLevel::Blue,
        settings.loot.suppressor2,
        suppressor2
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "17 - ",
        LootWeaponSuppressors,
        LootLevel::Purple,
        settings.loot.suppressor3,
        suppressor3
    );
    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, WeaponLasersSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "18 - ",
        LootWeaponLasers,
        LootLevel::White,
        settings.loot.lasersight1,
        lasersight1
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "19 - ",
        LootWeaponLasers,
        LootLevel::Blue,
        settings.loot.lasersight2,
        lasersight2
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "20 - ",
        LootWeaponLasers,
        LootLevel::Purple,
        settings.loot.lasersight3,
        lasersight3
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "21 - ",
        LootWeaponLasers,
        LootLevel::Gold,
        settings.loot.lasersight4,
        lasersight4
    );
    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, WeaponHopUpsSection))
        .add_dummy_item();
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "22 - ",
        LootTurboCharger,
        settings.loot.turbo_charger,
        turbo_charger
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "23 - ",
        LootSkullPiecer,
        settings.loot.skull_piecer,
        skull_piecer
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "24 - ",
        LootHammerPoints,
        settings.loot.hammer_point,
        hammer_point
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "25 - ",
        LootDisruptorRounds,
        settings.loot.disruptor_rounds,
        disruptor_rounds
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "26 - ",
        LootBoostedLoader,
        settings.loot.boosted_loader,
        boosted_loader
    );
    menu.add_dummy_item()
        .add_item(
            item_text(format!(
                "27 - {}",
                i18n_msg!(i18n_bundle, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_energy_weapons_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    settings: config::Settings,
) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title(i18n_msg!(i18n_bundle, EnergyWeaponsMenuTitle));
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(i18n_bundle, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(i18n_bundle, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, EnergyWeaponsSection))
        .add_dummy_item();
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "1 - ",
        WeaponLStar,
        settings.loot.weapon_lstar,
        weapon_lstar
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "2 - ",
        WeaponNemesis,
        settings.loot.weapon_nemesis,
        weapon_nemesis
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "3 - ",
        WeaponHavoc,
        settings.loot.weapon_havoc,
        weapon_havoc
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "4 - ",
        WeaponDeovtion,
        settings.loot.weapon_devotion,
        weapon_devotion
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "5 - ",
        WeaponTripleTake,
        settings.loot.weapon_triple_take,
        weapon_triple_take
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "6 - ",
        WeaponVolt,
        settings.loot.weapon_volt,
        weapon_volt
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "7 - ",
        LootEnergyAmmo,
        settings.loot.energyammo,
        energyammo
    );
    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, EnergyWeaponMagsSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "8 - ",
        LootEnergyWeaponMag,
        LootLevel::White,
        settings.loot.energyammomag1,
        energyammomag1
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "9 - ",
        LootEnergyWeaponMag,
        LootLevel::Blue,
        settings.loot.energyammomag2,
        energyammomag2
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "10 - ",
        LootEnergyWeaponMag,
        LootLevel::Purple,
        settings.loot.energyammomag3,
        energyammomag3
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "11 - ",
        LootEnergyWeaponMag,
        LootLevel::Gold,
        settings.loot.energyammomag4,
        energyammomag4
    );

    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, WeaponStocksSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "12 - ",
        LootStandardStock,
        LootLevel::White,
        settings.loot.stockregular1,
        stockregular1
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "13 - ",
        LootStandardStock,
        LootLevel::Blue,
        settings.loot.stockregular2,
        stockregular2
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "14 - ",
        LootStandardStock,
        LootLevel::Purple,
        settings.loot.stockregular3,
        stockregular3
    );
    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, WeaponSuppressorsSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "15 - ",
        LootWeaponSuppressors,
        LootLevel::White,
        settings.loot.suppressor1,
        suppressor1
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "16 - ",
        LootWeaponSuppressors,
        LootLevel::Blue,
        settings.loot.suppressor2,
        suppressor2
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "17 - ",
        LootWeaponSuppressors,
        LootLevel::Purple,
        settings.loot.suppressor3,
        suppressor3
    );
    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, WeaponLasersSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "18 - ",
        LootWeaponLasers,
        LootLevel::White,
        settings.loot.lasersight1,
        lasersight1
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "19 - ",
        LootWeaponLasers,
        LootLevel::Blue,
        settings.loot.lasersight2,
        lasersight2
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "20 - ",
        LootWeaponLasers,
        LootLevel::Purple,
        settings.loot.lasersight3,
        lasersight3
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "21 - ",
        LootWeaponLasers,
        LootLevel::Gold,
        settings.loot.lasersight4,
        lasersight4
    );
    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, WeaponHopUpsSection))
        .add_dummy_item();
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "22 - ",
        LootTurboCharger,
        settings.loot.turbo_charger,
        turbo_charger
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "23 - ",
        LootSkullPiecer,
        settings.loot.skull_piecer,
        skull_piecer
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "24 - ",
        LootHammerPoints,
        settings.loot.hammer_point,
        hammer_point
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "25 - ",
        LootDisruptorRounds,
        settings.loot.disruptor_rounds,
        disruptor_rounds
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "26 - ",
        LootBoostedLoader,
        settings.loot.boosted_loader,
        boosted_loader
    );
    menu.add_dummy_item()
        .add_item(
            item_text(format!(
                "27 - {}",
                i18n_msg!(i18n_bundle, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_sniper_weapons_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    settings: config::Settings,
) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title(i18n_msg!(i18n_bundle, SniperWeaponsMenuTitle));
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(i18n_bundle, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(i18n_bundle, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, SniperWeaponsSection))
        .add_dummy_item();
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "1 - ",
        WeaponWingman,
        settings.loot.weapon_wingman,
        weapon_wingman
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "2 - ",
        WeaponLongbow,
        settings.loot.weapon_longbow,
        weapon_longbow
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "3 - ",
        WeaponChargeRifle,
        settings.loot.weapon_charge_rifle,
        weapon_charge_rifle
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "4 - ",
        WeaponSentinel,
        settings.loot.weapon_sentinel,
        weapon_sentinel
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "5 - ",
        WeaponBow,
        settings.loot.weapon_bow,
        weapon_bow
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "6 - ",
        LootSniperAmmo,
        settings.loot.sniperammo,
        sniperammo
    );
    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, SniperWeaponMagsSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "7 - ",
        LootSniperWeaponMag,
        LootLevel::White,
        settings.loot.sniperammomag1,
        sniperammomag1
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "8 - ",
        LootSniperWeaponMag,
        LootLevel::Blue,
        settings.loot.sniperammomag2,
        sniperammomag2
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "9 - ",
        LootSniperWeaponMag,
        LootLevel::Purple,
        settings.loot.sniperammomag3,
        sniperammomag3
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "10 - ",
        LootSniperWeaponMag,
        LootLevel::Gold,
        settings.loot.sniperammomag4,
        sniperammomag4
    );

    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, WeaponStocksSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "11 - ",
        LootSniperStock,
        LootLevel::White,
        settings.loot.stocksniper1,
        stocksniper1
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "12 - ",
        LootSniperStock,
        LootLevel::Blue,
        settings.loot.stocksniper2,
        stocksniper2
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "13 - ",
        LootSniperStock,
        LootLevel::Purple,
        settings.loot.stocksniper3,
        stocksniper3
    );
    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, WeaponSuppressorsSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "14 - ",
        LootWeaponSuppressors,
        LootLevel::White,
        settings.loot.suppressor1,
        suppressor1
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "15 - ",
        LootWeaponSuppressors,
        LootLevel::Blue,
        settings.loot.suppressor2,
        suppressor2
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "16 - ",
        LootWeaponSuppressors,
        LootLevel::Purple,
        settings.loot.suppressor3,
        suppressor3
    );
    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, WeaponHopUpsSection))
        .add_dummy_item();
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "17 - ",
        LootTurboCharger,
        settings.loot.turbo_charger,
        turbo_charger
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "18 - ",
        LootSkullPiecer,
        settings.loot.skull_piecer,
        skull_piecer
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "19 - ",
        LootHammerPoints,
        settings.loot.hammer_point,
        hammer_point
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "20 - ",
        LootDisruptorRounds,
        settings.loot.disruptor_rounds,
        disruptor_rounds
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "21 - ",
        LootBoostedLoader,
        settings.loot.boosted_loader,
        boosted_loader
    );
    menu.add_dummy_item()
        .add_item(
            item_text(format!(
                "22 - {}",
                i18n_msg!(i18n_bundle, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_armors_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    settings: config::Settings,
) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title(i18n_msg!(i18n_bundle, ArmorsMenuTitle));
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(i18n_bundle, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(i18n_bundle, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, ArmorsSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "1 - ",
        LootEvoShield,
        LootLevel::White,
        settings.loot.shieldupgrade1,
        shieldupgrade1
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "2 - ",
        LootEvoShield,
        LootLevel::Blue,
        settings.loot.shieldupgrade2,
        shieldupgrade2
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "3 - ",
        LootEvoShield,
        LootLevel::Purple,
        settings.loot.shieldupgrade3,
        shieldupgrade3
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "4 - ",
        LootBodyShield,
        LootLevel::Gold,
        settings.loot.shieldupgrade4,
        shieldupgrade4
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "5 - ",
        LootEvoShield,
        LootLevel::Red,
        settings.loot.shieldupgrade5,
        shieldupgrade5
    );
    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, HelmetsSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "6 - ",
        LootHelmet,
        LootLevel::White,
        settings.loot.shieldupgradehead1,
        shieldupgradehead1
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "7 - ",
        LootHelmet,
        LootLevel::Blue,
        settings.loot.shieldupgradehead2,
        shieldupgradehead2
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "8 - ",
        LootHelmet,
        LootLevel::Purple,
        settings.loot.shieldupgradehead3,
        shieldupgradehead3
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "9 - ",
        LootHelmet,
        LootLevel::Gold,
        settings.loot.shieldupgradehead4,
        shieldupgradehead4
    );
    menu = menu
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, KnockdownShieldsSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "10 - ",
        LootKnockdownShield,
        LootLevel::White,
        settings.loot.shielddown1,
        shielddown1
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "11 - ",
        LootKnockdownShield,
        LootLevel::Blue,
        settings.loot.shielddown2,
        shielddown2
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "12 - ",
        LootKnockdownShield,
        LootLevel::Purple,
        settings.loot.shielddown3,
        shielddown3
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "13 - ",
        LootKnockdownShield,
        LootLevel::Gold,
        settings.loot.shielddown4,
        shielddown4
    );
    menu.add_dummy_item()
        .add_item(
            item_text(format!(
                "14 - {}",
                i18n_msg!(i18n_bundle, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_healing_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    settings: config::Settings,
) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title(i18n_msg!(i18n_bundle, HealingItemsMenuTitle));
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(i18n_bundle, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(i18n_bundle, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, HealingItemsSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "1 - ",
        LootAccelerant,
        LootLevel::Blue,
        settings.loot.accelerant,
        accelerant
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "2 - ",
        LootPhoenix,
        LootLevel::Purple,
        settings.loot.phoenix,
        phoenix
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "3 - ",
        LootSmallHealth,
        LootLevel::White,
        settings.loot.healthsmall,
        healthsmall
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "4 - ",
        LootLargeHealth,
        LootLevel::White,
        settings.loot.healthlarge,
        healthlarge
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "5 - ",
        LootSmallShieldBatt,
        LootLevel::White,
        settings.loot.shieldbattsmall,
        shieldbattsmall
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "6 - ",
        LootLargeShieldBatt,
        LootLevel::White,
        settings.loot.shieldbattlarge,
        shieldbattlarge
    );
    menu.add_dummy_item()
        .add_item(
            item_text(format!(
                "7 - {}",
                i18n_msg!(i18n_bundle, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_nades_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    settings: config::Settings,
) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title(i18n_msg!(i18n_bundle, NadesMenuTitle));
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(i18n_bundle, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(i18n_bundle, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, NadeItemsSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "1 - ",
        LootFragGrenade,
        LootLevel::Red,
        settings.loot.grenade_frag,
        grenade_frag
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "2 - ",
        LootArcStar,
        LootLevel::Blue,
        settings.loot.grenade_arc_star,
        grenade_arc_star
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "3 - ",
        LootThermite,
        LootLevel::Red,
        settings.loot.grenade_thermite,
        grenade_thermite
    );
    menu.add_dummy_item()
        .add_item(
            item_text(format!(
                "4 - {}",
                i18n_msg!(i18n_bundle, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_backpacks_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    settings: config::Settings,
) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title(i18n_msg!(i18n_bundle, BackpacksMenuTitle));
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(i18n_bundle, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(i18n_bundle, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, BackpacksSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "1 - ",
        LootLightBackpack,
        LootLevel::White,
        settings.loot.lightbackpack,
        lightbackpack
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "2 - ",
        LootMediumBackpack,
        LootLevel::Blue,
        settings.loot.medbackpack,
        medbackpack
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "3 - ",
        LootHeavyBackpack,
        LootLevel::Purple,
        settings.loot.heavybackpack,
        heavybackpack
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "4 - ",
        LootGoldBackpack,
        LootLevel::Gold,
        settings.loot.goldbackpack,
        goldbackpack
    );
    menu.add_dummy_item()
        .add_item(
            item_text(format!(
                "5 - {}",
                i18n_msg!(i18n_bundle, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_hopups_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    settings: config::Settings,
) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title(i18n_msg!(i18n_bundle, WeaponHopUpsMenuTitle));
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(i18n_bundle, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(i18n_bundle, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, WeaponHopUpsSection))
        .add_dummy_item();
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "1 - ",
        LootTurboCharger,
        settings.loot.turbo_charger,
        turbo_charger
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "2 - ",
        LootSkullPiecer,
        settings.loot.skull_piecer,
        skull_piecer
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "3 - ",
        LootHammerPoints,
        settings.loot.hammer_point,
        hammer_point
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "4 - ",
        LootDisruptorRounds,
        settings.loot.disruptor_rounds,
        disruptor_rounds
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "5 - ",
        LootBoostedLoader,
        settings.loot.boosted_loader,
        boosted_loader
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "6 - ",
        LootAnvilReceiver,
        settings.loot.anvil_receiver,
        anvil_receiver
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "7 - ",
        LootDoubletapTrigger,
        settings.loot.doubletap_trigger,
        doubletap_trigger
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "8 - ",
        LootDualShell,
        settings.loot.dual_shell,
        dual_shell
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "9 - ",
        LootKineticFeeder,
        settings.loot.kinetic_feeder,
        kinetic_feeder
    );
    menu = menu_add_pick_item!(
        menu,
        i18n_bundle,
        "10 - ",
        LootQuickdrawHolster,
        settings.loot.quickdraw_holster,
        quickdraw_holster
    );
    menu.add_dummy_item()
        .add_item(
            item_text(format!(
                "11 - {}",
                i18n_msg!(i18n_bundle, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
            (),
        )
        .into()
}

pub(super) fn build_scopes_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    settings: config::Settings,
) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title(i18n_msg!(i18n_bundle, ScopesMenuTitle));
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(i18n_bundle, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(i18n_bundle, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, ScopesSection))
        .add_dummy_item();
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "1 - ",
        Loot1xHcog,
        LootLevel::White,
        settings.loot.optic1xhcog,
        optic1xhcog
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "2 - ",
        Loot2xHcog,
        LootLevel::Blue,
        settings.loot.optic2xhcog,
        optic2xhcog
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "3 - ",
        Loot1xHolo,
        LootLevel::White,
        settings.loot.opticholo1x,
        opticholo1x
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "4 - ",
        Loot1x2xHolo,
        LootLevel::Blue,
        settings.loot.opticholo1x2x,
        opticholo1x2x
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "5 - ",
        LootOpticThreat,
        LootLevel::Gold,
        settings.loot.opticthreat,
        opticthreat
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "6 - ",
        Loot3xHcog,
        LootLevel::Purple,
        settings.loot.optic3xhcog,
        optic3xhcog
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "7 - ",
        Loot2x4xAog,
        LootLevel::Purple,
        settings.loot.optic2x4x,
        optic2x4x
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "8 - ",
        Loot6xSniperOptic,
        LootLevel::Blue,
        settings.loot.opticsniper6x,
        opticsniper6x
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "9 - ",
        Loot4x8xSniperOptic,
        LootLevel::Purple,
        settings.loot.opticsniper4x8x,
        opticsniper4x8x
    );
    menu = menu_add_colored_loot_item!(
        menu,
        i18n_bundle,
        "10 - ",
        LootSniperThreat,
        LootLevel::Gold,
        settings.loot.opticsniperthreat,
        opticsniperthreat
    );
    menu.add_dummy_item()
        .add_item(
            item_text(format!(
                "11 - {}",
                i18n_msg!(i18n_bundle, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
            (),
        )
        .into()
}
