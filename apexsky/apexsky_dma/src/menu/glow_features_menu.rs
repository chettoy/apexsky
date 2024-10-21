use fluent::FluentArgs;

use super::{
    handler_toggle_settings, GeneralMenu, GeneralMenuFormat, MenuBuilder, MenuBuilderExt,
    MenuFormatter, MenuLevel, TerminalMenu,
};
use crate::{config, i18n::I18nBundle, i18n_msg, i18n_msg_format, lock_config};

pub(super) fn build_glow_features_menu(
    menu_fmt: &MenuFormatter,
    settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    MenuBuilder::new(MenuLevel::GlowFeats, menu_fmt.clone())
        .title(i18n_msg!(menu_fmt, GlowFeaturesMenuTitle))
        .add_toggle_item(
            format!(" 1 - {}", i18n_msg!(menu_fmt, MenuItemItemGlow)),
            settings.glow_settings.item_glow,
            handler_toggle_settings!(.glow_settings.item_glow),
        )
        .add_toggle_item(
            format!(" 2 - {}", i18n_msg!(menu_fmt, MenuItemPlayerGlow)),
            settings.glow_settings.player_glow,
            handler_toggle_settings!(.glow_settings.player_glow),
        )
        .add_toggle_item(
            format!(" 3 - {}", i18n_msg!(menu_fmt, MenuItemLootGlowFilled)),
            settings.glow_settings.loot_filled_toggle,
            |_, _| {
                let settings = &mut lock_config!().settings;
                settings.glow_settings.loot_filled_toggle =
                    !settings.glow_settings.loot_filled_toggle;
                settings.glow_settings.loot_filled = if settings.glow_settings.loot_filled_toggle {
                    14
                } else {
                    0
                };
                None
            },
        )
        .add_toggle_item(
            format!(" 4 - {}", i18n_msg!(menu_fmt, MenuItemPlayerGlowFilled)),
            settings.glow_settings.player_filled_toggle,
            |_, _| {
                let settings = &mut lock_config!().settings;
                settings.glow_settings.player_filled_toggle =
                    !settings.glow_settings.player_filled_toggle;
                settings.glow_settings.player_glow_inside_value =
                    if settings.glow_settings.player_filled_toggle {
                        14
                    } else {
                        0
                    };
                None
            },
        )
        .add_input_item(
            menu_fmt.item_text(format!(
                " 5 - {}",
                i18n_msg!(menu_fmt, MenuItemPlayerOutlineSize)
            )),
            &i18n_msg!(menu_fmt, InputPromptPlayerOutlines),
            |_ctx, val, _| {
                let i18n_bundle = &I18nBundle::new();
                if let Ok(new_val) = val.parse::<u8>() {
                    let settings = &mut lock_config!().settings;
                    settings.glow_settings.player_glow_outline_size = new_val; //[0, 255]
                    return Some({
                        let mut args = FluentArgs::new();
                        args.set("value", settings.glow_settings.player_glow_outline_size);
                        i18n_msg_format!(i18n_bundle, InfoPlayerOutlineUpdated, args).to_string()
                    });
                }
                Some(i18n_msg!(i18n_bundle, InfoInvalidOutlineSize).to_string())
            },
            (),
        )
        .add_toggle_item(
            format!(" 6 - {}", i18n_msg!(menu_fmt, MenuItemPlayerArmorGlowColor)),
            settings.glow_settings.player_glow_armor_color,
            handler_toggle_settings!(.glow_settings.player_glow_armor_color),
        )
        .add_toggle_item(
            format!(" 7 - {}", i18n_msg!(menu_fmt, MenuItemFavoritePlayerGlow)),
            settings.glow_settings.player_glow_love_user,
            handler_toggle_settings!(.glow_settings.player_glow_love_user),
        )
        .add_item(
            menu_fmt.item_enabled(
                format!(" 8 - {}", i18n_msg!(menu_fmt, MenuItemWeaponModelGlow)),
                settings.glow_settings.weapon_model_glow,
            ),
            |_handle: &mut TerminalMenu, _| {
                let settings = &mut lock_config!().settings;
                settings.glow_settings.weapon_model_glow =
                    !settings.glow_settings.weapon_model_glow;
                if settings.glow_settings.weapon_model_glow {
                    let i18n_bundle = &I18nBundle::new();
                    Some(i18n_msg!(i18n_bundle, InfoWeaponModelGlow).to_string())
                } else {
                    None
                }
            },
            (),
        )
        .add_toggle_item(
            format!(
                "10 - {}",
                i18n_msg!(menu_fmt, MenuItemWeaponModelTransparent)
            ),
            settings.glow_settings.weapon_model_transparent,
            handler_toggle_settings!(.glow_settings.weapon_model_transparent),
        )
        .add_dummy_item()
        .add_item(
            menu_fmt.item_text(format!(
                "10 - {}",
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
