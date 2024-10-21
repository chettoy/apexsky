use super::{
    handler_toggle_settings, ratatui, GeneralMenu, GeneralMenuFormat, MenuBuilder, MenuBuilderExt,
    MenuFormatter, MenuLevel, TerminalMenu,
};
use crate::{config, global_state::G_CONTEXT, i18n::I18nBundle, i18n_msg, lock_config};
use ratatui::{
    style::{Style, Stylize},
    text::Span,
};

pub(super) fn build_main_menu(
    menu_fmt: &MenuFormatter,
    settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    #[allow(clippy::invisible_characters)]
    MenuBuilder::new(MenuLevel::Main, menu_fmt.clone())
        .title(i18n_msg!(menu_fmt, MainMenuTitle))
        .add_toggle_item(
            format!(" 1 - {}", i18n_msg!(menu_fmt, MenuItemFiringRange)),
            settings.firing_range,
            handler_toggle_settings!(.firing_range),
        )
        .add_toggle_item(
            format!(" 2 - {}", i18n_msg!(menu_fmt, MenuItemTdmToggle)),
            settings.team_death_match,
            handler_toggle_settings!(.team_death_match),
        )
        .skip_id()
        .add_toggle_item(
            format!(" 4 - {}", i18n_msg!(menu_fmt, MenuItemGamepad)),
            settings.aimbot_settings.gamepad,
            |_, _| {
                let settings = &mut lock_config!().settings;
                settings.aimbot_settings.gamepad = !settings.aimbot_settings.gamepad;
                if settings.aimbot_settings.gamepad {
                    settings.aimbot_settings.aimbot_on_ads = true;
                    settings.aimbot_settings.aimbot_on_fire = true;
                }
                None
            },
        )
        .skip_id()
        .skip_id()
        .add_item(
            menu_fmt.item_text(format!(" 7 - {}", i18n_msg!(menu_fmt, AimbotMenuTitle))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Aimbot);
                None
            },
            (),
        )
        .add_item(
            menu_fmt.item_text(format!(
                " 8 - {}",
                i18n_msg!(menu_fmt, MenuItemHotkeySettings)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Hotkey);
                None
            },
            (),
        )
        .skip_id()
        .skip_id()
        .add_item(
            menu_fmt.item_text(format!(
                "11 - {}",
                i18n_msg!(menu_fmt, MenuItemGlowFeatures)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::GlowFeats);
                None
            },
            (),
        )
        .add_item(
            menu_fmt.item_text(format!(
                "12 - {}",
                i18n_msg!(menu_fmt, MenuItemUpdateGlowColors)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::GlowColor);
                None
            },
            (),
        )
        .skip_id()
        .skip_id()
        .skip_id()
        .add_item(
            menu_fmt.item_text(format!(
                "16 - {}",
                i18n_msg!(menu_fmt, MenuItemItemFilterSettings)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::ItemFilter);
                None
            },
            (),
        )
        .add_input_item(
            menu_fmt.format_item(
                format!("17 - {}", i18n_msg!(menu_fmt, MenuItemSetFpsPredict)),
                Span::from(if settings.calc_game_fps {
                    i18n_msg!(menu_fmt, MenuValueCalcFps).to_string()
                } else {
                    format!("{:.1}", settings.game_fps)
                }),
            ),
            &i18n_msg!(menu_fmt, InputPromptFpsPredict),
            |_ctx, val, _| {
                if let Ok(new_val) = val.parse::<u16>() {
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
            (),
        )
        .add_item(
            if settings.load_settings {
                menu_fmt.item_dummy()
            } else {
                menu_fmt.item_text("18.5 -‌​‌‌​​​‌‌‌‍‌​‌‌​‌​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌​‌‌‌‌‌‍‌​‌‌‌‌​​‌‌‍‌​‌‌​​​​‌‌‍‌​‌‌‌​​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌‌​​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​‌‌‍‌​‌‌​‌​​‌‌‍‌​‌‌​‌‌​‌‌‍‌​‌‌​​‌​‌‌‍‌​‌‌​‌‌‌​‌‍‌​‌‌‌​‌​‌‌ ")
            },
            |_, _| {
                let config = &mut lock_config!();
                config.settings.load_settings = !config.settings.load_settings;
                if config.settings.load_settings {
                    None
                } else {
                    let i18n_bundle = &I18nBundle::new();
                    Some(i18n_msg!(i18n_bundle, HelloWorld).to_string())
                }
            },
            (),
        )
        .skip_id()
        .add_toggle_item(
            format!("20 - {}", i18n_msg!(menu_fmt, MenuItemDeathBoxes)),
            settings.feature_settings.show_deathbox,
            handler_toggle_settings!(.feature_settings.show_deathbox),
        )
        .add_dummy_item()
        .add_item(
            menu_fmt.item_text(format!(
                "21 - {}",
                i18n_msg!(menu_fmt, MenuItemSaveSettings)
            )),
            |_, _| {
                let i18n_bundle = &I18nBundle::new();
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
            menu_fmt.item_text(format!(
                "22 - {}",
                i18n_msg!(menu_fmt, MenuItemLoadSettings)
            )),
            |_, _| {
                let i18n_bundle = &I18nBundle::new();
                let mut result = i18n_msg!(i18n_bundle, InfoLoaded).to_string();
                let config_state = crate::config::get_configuration(&crate::get_config_file_path())
                    .unwrap_or_else(|e| {
                        result = format!("{}\n{}", e, i18n_msg!(i18n_bundle, InfoFallbackConfig));
                        crate::config::Config::default()
                    });
                lock_config!() = config_state;
                Some(result)
            },
            (),
        )
        .add_dummy_item()
        .add_toggle_item(
            format!("23 - {}", i18n_msg!(menu_fmt, MenuItemSuperGlide)),
            settings.feature_settings.auto_super_glide,
            handler_toggle_settings!(.feature_settings.auto_super_glide),
        )
        .add_toggle_item(
            format!("24 - {}", i18n_msg!(menu_fmt, MenuItemSuperGrapple)),
            settings.feature_settings.auto_super_grapple,
            handler_toggle_settings!(.feature_settings.auto_super_grapple),
        )
        .add_toggle_item(
            format!("25 - {}", i18n_msg!(menu_fmt, MenuItemAutoTapstrafe)),
            settings.feature_settings.auto_tap_strafe,
            handler_toggle_settings!(.feature_settings.auto_tap_strafe),
        )
        .skip_id()
        .add_toggle_item(
            format!("27 - {}", i18n_msg!(menu_fmt, MenuItemToggleOnevone)),
            settings.feature_settings.onevone,
            handler_toggle_settings!(.feature_settings.onevone),
        )
        .skip_id()
        .add_item(
            menu_fmt.item_enabled(
                format!("29 - {}", i18n_msg!(menu_fmt, MenuItemKbdBacklightCtrl)),
                settings.feature_settings.kbd_backlight_control,
            ),
            |_handle: &mut TerminalMenu, _| {
                let settings = &mut lock_config!().settings;
                settings.feature_settings.kbd_backlight_control =
                    !settings.feature_settings.kbd_backlight_control;
                if settings.feature_settings.kbd_backlight_control {
                    if let Err(e) = G_CONTEXT.lock().unwrap().kbd_backlight_test() {
                        return Some(e.to_string());
                    }
                }
                None
            },
            (),
        )
        .add_toggle_item(
            format!("30 - {}", i18n_msg!(menu_fmt, MenuItemBigMapFeat)),
            settings.feature_settings.map_radar_testing,
            handler_toggle_settings!(.feature_settings.map_radar_testing),
        )
        .add_dummy_item()
        .add_item(
            menu_fmt.item_text(format!("31 - {}", i18n_msg!(menu_fmt, MenuItemPlayersMenu))),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Players);
                None
            },
            (),
        )
        .add_item(
            menu_fmt.format_item(
                format!("32 - {}", i18n_msg!(menu_fmt, MenuItemToggleEspService)),
                if settings.no_esp_service {
                    Span::from(i18n_msg!(menu_fmt, MenuValueEspServiceOff).to_string())
                } else {
                    Span::styled(
                        i18n_msg!(menu_fmt, MenuValueEspServiceOn).to_string(),
                        Style::default().green(),
                    )
                },
            ),
            |_, _| {
                let settings = &mut lock_config!().settings;
                settings.no_esp_service = !settings.no_esp_service;
                None
            },
            (),
        )
        .into()
}
