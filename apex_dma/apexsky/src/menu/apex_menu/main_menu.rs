use crate::{
    config, global_state::G_CONTEXT, i18n::get_fluent_bundle, i18n_msg, i18n_msg_format,
    lock_config, menu_add_toggle_item,
};
use fluent::{FluentArgs, FluentBundle, FluentResource};
use ratatui::{
    style::{Style, Stylize},
    text::Span,
};

use super::{
    format_item, item_dummy, item_enabled, item_text, MenuBuilder, MenuLevel, MenuState,
    TerminalMenu,
};

pub(super) fn build_main_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    settings: config::Settings,
) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title(i18n_msg!(i18n_bundle, MainMenuTitle));
    menu = menu_add_toggle_item!(
        menu,
        &i18n_bundle,
        format!(" 1 - {}", i18n_msg!(i18n_bundle, MenuItemFiringRange)),
        settings.firing_range,
        firing_range
    );
    menu = menu_add_toggle_item!(
        menu,
        &i18n_bundle,
        format!(" 2 - {}", i18n_msg!(i18n_bundle, MenuItemTdmToggle)),
        settings.tdm_toggle,
        tdm_toggle
    );
    menu = menu
        .add_item(
            item_enabled(
                &i18n_bundle,
                format!(" 3 - {}", i18n_msg!(i18n_bundle, MenuItemKeyboard)),
                !settings.aimbot_settings.gamepad,
            ),
            |_, _| {
                let settings = &mut lock_config!().settings;
                settings.aimbot_settings.gamepad = !settings.aimbot_settings.gamepad;
                None
            },
            (),
        )
        .add_item(
            item_enabled(
                &i18n_bundle,
                format!(" 4 - {}", i18n_msg!(i18n_bundle, MenuItemGamepad)),
                settings.aimbot_settings.gamepad,
            ),
            |_, _| {
                let settings = &mut lock_config!().settings;
                settings.aimbot_settings.gamepad = !settings.aimbot_settings.gamepad;
                None
            },
            (),
        );
    menu = menu_add_toggle_item!(
        menu,
        &i18n_bundle,
        format!(" 5 - {}", i18n_msg!(i18n_bundle, MenuItemItemGlow)),
        settings.item_glow,
        item_glow
    );
    menu = menu_add_toggle_item!(
        menu,
        &i18n_bundle,
        format!(" 6 - {}", i18n_msg!(i18n_bundle, MenuItemPlayerGlow)),
        settings.player_glow,
        player_glow
    );
    menu = menu
        .add_item(
            item_text(format!(" 7 - {}", i18n_msg!(i18n_bundle, AimbotMenuTitle))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::AimbotMenu);
                None
            },
            (),
        )
        .add_item(
            item_text(format!(
                " 8 - {}",
                i18n_msg!(i18n_bundle, MenuItemHotkeySettings)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::HotkeyMenu);
                None
            },
            (),
        )
        .add_item(
            item_enabled(
                &i18n_bundle,
                format!(" 9 - {}", i18n_msg!(i18n_bundle, MenuItemLootGlowFilled)),
                settings.loot_filled_toggle,
            ),
            |_, _| {
                let settings = &mut lock_config!().settings;
                settings.loot_filled_toggle = !settings.loot_filled_toggle;
                settings.loot_filled = if settings.loot_filled_toggle { 14 } else { 0 };
                None
            },
            (),
        )
        .add_item(
            item_enabled(
                &i18n_bundle,
                format!("10 - {}", i18n_msg!(i18n_bundle, MenuItemPlayerGlowFilled)),
                settings.player_filled_toggle,
            ),
            |_, _| {
                let settings = &mut lock_config!().settings;
                settings.player_filled_toggle = !settings.player_filled_toggle;
                settings.player_glow_inside_value =
                    if settings.player_filled_toggle { 14 } else { 0 };
                None
            },
            (),
        )
        .add_input_item(
            item_text(format!(
                "11 - {}",
                i18n_msg!(i18n_bundle, MenuItemPlayerOutlineSize)
            )),
            &i18n_msg!(i18n_bundle, InputPromptPlayerOutlines),
            |val| {
                let i18n_bundle = get_fluent_bundle();
                if let Some(new_val) = val.parse::<u8>().ok() {
                    let settings = &mut lock_config!().settings;
                    settings.player_glow_outline_size = new_val; //[0, 255]
                    return Some({
                        let mut args = FluentArgs::new();
                        args.set("value", settings.player_glow_outline_size);
                        i18n_msg_format!(i18n_bundle, InfoPlayerOutlineUpdated, args).to_string()
                    });
                }
                Some(i18n_msg!(i18n_bundle, InfoInvalidOutlineSize).to_string())
            },
        )
        .add_item(
            item_text(format!(
                "12 - {}",
                i18n_msg!(i18n_bundle, MenuItemUpdateGlowColors)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::GlowColorMenu);
                None
            },
            (),
        )
        .skip_id();
    menu = menu_add_toggle_item!(
        menu,
        &i18n_bundle,
        format!(
            "14 - {}",
            i18n_msg!(i18n_bundle, MenuItemPlayerArmorGlowColor)
        ),
        settings.player_glow_armor_color,
        player_glow_armor_color
    );
    menu = menu_add_toggle_item!(
        menu,
        &i18n_bundle,
        format!(
            "15 - {}",
            i18n_msg!(i18n_bundle, MenuItemFavoritePlayerGlow)
        ),
        settings.player_glow_love_user,
        player_glow_love_user
    );
    menu = menu
        .add_item(
            item_text(format!(
                "16 - {}",
                i18n_msg!(i18n_bundle, MenuItemItemFilterSettings)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::ItemFilterMenu);
                None
            },
            (),
        )
        .add_input_item(
            format_item(
                &i18n_bundle,
                format!("17 - {}", i18n_msg!(i18n_bundle, MenuItemSetFpsPredict)),
                Span::from(if settings.calc_game_fps {
                    i18n_msg!(i18n_bundle, MenuValueCalcFps).to_string()
                } else {
                    format!("{:.1}", settings.game_fps)
                }),
            ),
            &i18n_msg!(i18n_bundle, InputPromptFpsPredict),
            |val| {
                if let Some(new_val) = val.parse::<u16>().ok() {
                    let settings = &mut lock_config!().settings;
                    if new_val == 0 {
                        settings.calc_game_fps = true;
                    } else if new_val > 0 && new_val <= 500 {
                        settings.calc_game_fps = false;
                        settings.game_fps = new_val.into();
                    }
                }
                None
            },
        )
        .add_item(
            if settings.load_settings {
                item_dummy()
            } else {
                item_text("18.5 -‌​‌‌​​​‌‌‌‍‌​‌‌​‌​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌​‌‌‌‌‌‍‌​‌‌‌‌​​‌‌‍‌​‌‌​​​​‌‌‍‌​‌‌‌​​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌‌​​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​‌‌‍‌​‌‌​‌​​‌‌‍‌​‌‌​‌‌​‌‌‍‌​‌‌​​‌​‌‌‍‌​‌‌​‌‌‌​‌‍‌​‌‌‌​‌​‌‌ ")
            },
            |_, _| {
                let config = &mut lock_config!();
                config.settings.load_settings = !config.settings.load_settings;
                if config.settings.load_settings {
                    None
                } else {
                    let i18n_bundle = get_fluent_bundle();
                    Some(i18n_msg!(i18n_bundle, HelloWorld).to_string())
                }
            },
            (),
        )
        .skip_id();
    menu = menu_add_toggle_item!(
        menu,
        &i18n_bundle,
        format!("20 - {}", i18n_msg!(i18n_bundle, MenuItemDeathBoxes)),
        settings.deathbox,
        deathbox
    );
    menu = menu
        .add_dummy_item()
        .add_item(
            item_text(format!(
                "21 - {}",
                i18n_msg!(i18n_bundle, MenuItemSaveSettings)
            )),
            |_, _| {
                let i18n_bundle = get_fluent_bundle();
                Some(
                    if crate::save_settings() {
                        i18n_msg!(i18n_bundle, InfoSaved)
                    } else {
                        i18n_msg!(i18n_bundle, InfoFailed)
                    }
                    .to_string(),
                )
            },
            (),
        )
        .add_item(
            item_text(format!(
                "22 - {}",
                i18n_msg!(i18n_bundle, MenuItemLoadSettings)
            )),
            |_, _| {
                let i18n_bundle = get_fluent_bundle();
                let mut result = i18n_msg!(i18n_bundle, InfoLoaded).to_string();
                let config_state = crate::config::get_configuration().unwrap_or_else(|e| {
                    let i18n_bundle = get_fluent_bundle();
                    result = format!("{}\n{}", e, i18n_msg!(i18n_bundle, InfoFallbackConfig));
                    crate::config::Config::default()
                });
                lock_config!() = config_state;
                Some(result)
            },
            (),
        )
        .add_dummy_item();
    menu = menu_add_toggle_item!(
        menu,
        &i18n_bundle,
        format!("23 - {}", i18n_msg!(i18n_bundle, MenuItemSuperGlide)),
        settings.super_key_toggle,
        super_key_toggle
    );
    menu = menu_add_toggle_item!(
        menu,
        &i18n_bundle,
        format!("24 - {}", i18n_msg!(i18n_bundle, MenuItemToggleOnevone)),
        settings.onevone,
        onevone
    );
    menu = menu.add_item(
        item_enabled(
            &i18n_bundle,
            format!("25 - {}", i18n_msg!(i18n_bundle, MenuItemWeaponModelGlow)),
            settings.weapon_model_glow,
        ),
        |_handle: &mut TerminalMenu, _| {
            let settings = &mut lock_config!().settings;
            settings.weapon_model_glow = !settings.weapon_model_glow;
            if settings.weapon_model_glow {
                let i18n_bundle = get_fluent_bundle();
                Some(i18n_msg!(i18n_bundle, InfoWeaponModelGlow).to_string())
            } else {
                None
            }
        },
        (),
    );
    menu = menu.add_item(
        item_enabled(
            &i18n_bundle,
            format!("26 - {}", i18n_msg!(i18n_bundle, MenuItemKbdBacklightCtrl)),
            settings.kbd_backlight_control,
        ),
        |_handle: &mut TerminalMenu, _| {
            let settings = &mut lock_config!().settings;
            settings.kbd_backlight_control = !settings.kbd_backlight_control;
            if settings.kbd_backlight_control {
                if let Err(e) = G_CONTEXT.lock().unwrap().kbd_backlight_test() {
                    return Some(e.to_string());
                }
            }
            None
        },
        (),
    );
    menu = menu_add_toggle_item!(
        menu,
        &i18n_bundle,
        format!("27 - {}", i18n_msg!(i18n_bundle, MenuItemBigMapFeat)),
        settings.map_radar_testing,
        map_radar_testing
    );
    menu.add_dummy_item()
        .add_item(
            item_text(format!(
                "28 - {}",
                i18n_msg!(i18n_bundle, MenuItemSpectatorsMenu)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::SpectatorsMenu);
                None
            },
            (),
        )
        .skip_id()
        .skip_id()
        .skip_id()
        .add_item(
            format_item(
                &i18n_bundle,
                format!("32 - {}", i18n_msg!(i18n_bundle, MenuItemToggleOverlay)),
                if settings.no_overlay {
                    Span::from(i18n_msg!(i18n_bundle, MenuValueNoOverlay).to_string())
                } else {
                    Span::styled(
                        i18n_msg!(i18n_bundle, MenuValueExternalOverlay).to_string(),
                        Style::default().green(),
                    )
                },
            ),
            |_, _| {
                let settings = &mut lock_config!().settings;
                settings.no_overlay = !settings.no_overlay;
                None
            },
            (),
        )
        .into()
}
