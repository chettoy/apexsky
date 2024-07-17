use fluent::FluentArgs;
use ratatui::{
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::ListItem,
};

use super::{format_label, item_text, GeneralMenu, MenuBuilder, MenuLevel, TerminalMenu};
use crate::{config, i18n::I18nBundle, i18n_msg, i18n_msg_format, lock_config};

pub(super) fn build_hotkey_menu(
    i18n_bundle: &I18nBundle,
    settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    fn menu_item_keycode(label: String, value: i32) -> ListItem<'static> {
        ListItem::new(Line::from(vec![
            format_label(label),
            Span::styled(format!("{}", value), Style::default().underlined()),
        ]))
    }
    macro_rules! prompt_text_keycode {
        ( $i18n_bundle:expr, $label_id:ident ) => {{
            let label = i18n_msg!($i18n_bundle, $label_id);
            let mut args = FluentArgs::new();
            args.set("item_label", label);
            i18n_msg_format!($i18n_bundle, InputPromptKeycode, args).to_string()
        }};
    }
    macro_rules! text_invalid_keycode {
        ( $i18n_bundle:expr, $label_id:ident ) => {{
            let label = i18n_msg!($i18n_bundle, $label_id);
            let mut args = FluentArgs::new();
            args.set("item_label", label);
            i18n_msg_format!($i18n_bundle, InfoInvalidKeycode, args).to_string()
        }};
    }

    MenuBuilder::new(MenuLevel::HotkeyMenu)
        .title(i18n_msg!(i18n_bundle, HotkeyMenuTitle))
        .add_input_item(
            menu_item_keycode(
                format!("1 - {}", i18n_msg!(i18n_bundle, HotkeyItemAimbot1)),
                settings.aimbot_hot_key_1,
            ),
            &prompt_text_keycode!(i18n_bundle, HotkeyItemAimbot1),
            |val| {
                if let Some(keycode) = val.parse::<u16>().ok() {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_hot_key_1 = keycode as i32;
                    return None;
                }
                let i18n_bundle = &I18nBundle::new();
                Some(text_invalid_keycode!(i18n_bundle, HotkeyItemAimbot1))
            },
        )
        .add_input_item(
            menu_item_keycode(
                format!("2 - {}", i18n_msg!(i18n_bundle, HotkeyItemAimbot2)),
                settings.aimbot_hot_key_2,
            ),
            &prompt_text_keycode!(i18n_bundle, HotkeyItemAimbot2),
            |val| {
                if let Some(keycode) = val.parse::<u16>().ok() {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_hot_key_2 = keycode as i32;
                    return None;
                }
                let i18n_bundle = &I18nBundle::new();
                Some(text_invalid_keycode!(i18n_bundle, HotkeyItemAimbot2))
            },
        )
        .add_input_item(
            menu_item_keycode(
                format!("3 - {}", i18n_msg!(i18n_bundle, HotkeyItemTriggerBot)),
                settings.trigger_bot_hot_key,
            ),
            &prompt_text_keycode!(i18n_bundle, HotkeyItemTriggerBot),
            |val| {
                if let Some(keycode) = val.parse::<u16>().ok() {
                    let settings = &mut lock_config!().settings;
                    settings.trigger_bot_hot_key = keycode as i32;
                    return None;
                }
                let i18n_bundle = &I18nBundle::new();
                Some(text_invalid_keycode!(i18n_bundle, HotkeyItemTriggerBot))
            },
        )
        .add_input_item(
            menu_item_keycode(
                format!("4 - {}", i18n_msg!(i18n_bundle, HotkeyItemQuickLooting)),
                settings.quick_looting_hot_key,
            ),
            &prompt_text_keycode!(i18n_bundle, HotkeyItemQuickLooting),
            |val| {
                if let Some(keycode) = val.parse::<u16>().ok() {
                    let settings = &mut lock_config!().settings;
                    settings.quick_looting_hot_key = keycode as i32;
                    return None;
                }
                let i18n_bundle = &I18nBundle::new();
                Some(text_invalid_keycode!(i18n_bundle, HotkeyItemQuickLooting))
            },
        )
        .add_dummy_item()
        .add_item(
            item_text(format!("5 - {}", i18n_msg!(i18n_bundle, MenuItemKeyCodes))),
            |handler: &mut TerminalMenu, _| {
                handler.nav_menu(MenuLevel::KeyCodesMenu);
                None
            },
            (),
        )
        .add_dummy_item()
        .add_item(
            item_text(format!(
                "6 - {}",
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

pub(super) fn build_key_codes_menu(
    i18n_bundle: &I18nBundle,
    _settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    MenuBuilder::new(MenuLevel::KeyCodesMenu)
        .title(i18n_msg!(i18n_bundle, HotkeyMenuTitle))
        .add_text_item(i18n_msg!(i18n_bundle, Keycode108Mouse1Left))
        .add_text_item(i18n_msg!(i18n_bundle, Keycode109Mouse2Right))
        .add_text_item(i18n_msg!(i18n_bundle, Keycode110Mouse3Middle))
        .add_text_item(i18n_msg!(i18n_bundle, Keycode111Mouse4Side))
        .add_text_item(i18n_msg!(i18n_bundle, Keycode112Mouse5Side))
        .add_text_item("79 SHIFT key")
        .add_text_item("81 ALT key")
        .add_text_item("83 CTRL key")
        .add_text_item("1 KEY_0")
        .add_text_item("2 KEY_1")
        .add_text_item("3 KEY_2")
        .add_text_item("4 KEY_3")
        .add_text_item("5 KEY_4")
        .add_text_item("6 KEY_5")
        .add_text_item("7 KEY_6")
        .add_text_item("8 KEY_7")
        .add_text_item("9 KEY_8")
        .add_text_item("10 KEY_9")
        .add_text_item("11 KEY_A")
        .add_text_item("12 KEY_B")
        .add_text_item("13 KEY_C")
        .add_text_item("14 KEY_D")
        .add_text_item("15 KEY_E")
        .add_text_item("16 KEY_F")
        .add_text_item("17 KEY_G")
        .add_text_item("18 KEY_H")
        .add_text_item("19 KEY_I")
        .add_text_item("20 KEY_J")
        .add_text_item("21 KEY_K")
        .add_text_item("22 KEY_L")
        .add_text_item("23 KEY_M")
        .add_text_item("24 KEY_N")
        .add_text_item("25 KEY_O")
        .add_text_item("26 KEY_P")
        .add_text_item("27 KEY_Q")
        .add_text_item("28 KEY_R")
        .add_text_item("29 KEY_S")
        .add_text_item("30 KEY_T")
        .add_text_item("31 KEY_U")
        .add_text_item("32 KEY_V")
        .add_text_item("33 KEY_W")
        .add_text_item("34 KEY_X")
        .add_text_item("35 KEY_Y")
        .add_text_item("36 KEY_Z")
        .add_text_item("37 KEY_PAD_0")
        .add_text_item("38 KEY_PAD_1")
        .add_text_item("39 KEY_PAD_2")
        .add_text_item("40 KEY_PAD_3")
        .add_text_item("41 KEY_PAD_4")
        .add_text_item("42 KEY_PAD_5")
        .add_text_item("43 KEY_PAD_6")
        .add_text_item("44 KEY_PAD_7")
        .add_text_item("45 KEY_PAD_8")
        .add_text_item("46 KEY_PAD_9")
        .add_text_item("47 KEY_PAD_DIVIDE")
        .add_text_item("48 KEY_PAD_MULTIPLY")
        .add_text_item("49 KEY_PAD_MINUS")
        .add_text_item("50 KEY_PAD_PLUS")
        .add_text_item("51 KEY_PAD_ENTER")
        .add_text_item("52 KEY_PAD_DECIMAL")
        .add_text_item("65 KEY_SPACE")
        .add_text_item("67 KEY_TAB")
        .add_text_item("68 KEY_CAPSLOCK")
        .add_text_item("69 KEY_NUMLOCK")
        .add_text_item("70 KEY_ESCAPE")
        .add_text_item("71 KEY_SCROLLLOCK")
        .add_text_item("72 KEY_INSERT")
        .add_text_item("73 KEY_DELETE")
        .add_text_item("74 KEY_HOME")
        .add_text_item("75 KEY_END")
        .add_text_item("76 KEY_PAGEUP")
        .add_text_item("77 KEY_PAGEDOWN")
        .add_text_item("78 KEY_BREAK")
        .add_text_item("88 KEY_UP")
        .add_text_item("89 KEY_LEFT")
        .add_text_item("90 KEY_DOWN")
        .add_text_item("91 KEY_RIGHT")
        .add_text_item("92 KEY_F1")
        .add_text_item("93 KEY_F2")
        .add_text_item("94 KEY_F3")
        .add_text_item("95 KEY_F4")
        .add_text_item("96 KEY_F5")
        .add_text_item("97 KEY_F6")
        .add_text_item("98 KEY_F7")
        .add_text_item("99 KEY_F8")
        .add_text_item("100 KEY_F9")
        .add_text_item("101 KEY_F10")
        .add_text_item("102 KEY_F11")
        .add_text_item("103 KEY_F12")
        .add_dummy_item()
        .add_item(
            item_text(i18n_msg!(i18n_bundle, MenuItemBackToHotkeyMenu)),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::HotkeyMenu);
                None
            },
            (),
        )
        .add_dummy_item()
        .add_item(
            item_text(i18n_msg!(i18n_bundle, MenuItemBackToMainMenu)),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
            (),
        )
        .into()
}
