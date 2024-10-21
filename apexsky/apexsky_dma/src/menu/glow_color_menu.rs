use super::{GeneralMenu, GeneralMenuFormat, MenuBuilder, MenuFormatter, MenuLevel, TerminalMenu};
use crate::{config, i18n::I18nBundle, i18n_msg, i18n_msg_format, lock_config};
use fluent::FluentArgs;

pub(super) fn build_glow_color_menu(
    menu_fmt: &MenuFormatter,
    settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    fn parse_rgb(val: &str) -> Result<(f32, f32, f32), String> {
        let i18n_bundle = &I18nBundle::new();
        let val: Vec<&str> = val.split(" ").collect();
        if val.len() != 3 {
            let mut args = FluentArgs::new();
            args.set("getting", val.len());
            return Err(i18n_msg_format!(i18n_bundle, InfoExpectingValueCount, args).to_string());
        }
        let r = val[0].parse::<f32>().ok();
        let g = val[1].parse::<f32>().ok();
        let b = val[2].parse::<f32>().ok();
        if r.is_none() || g.is_none() || b.is_none() {
            return Err(i18n_msg!(i18n_bundle, InfoCannotParseInputValues).to_string());
        }
        let (r, g, b) = (r.unwrap(), g.unwrap(), b.unwrap());
        if ![r, g, b].iter().all(|value| (0.0..1.0).contains(value)) {
            return Err(i18n_msg!(i18n_bundle, InfoValuesOutOfRange).to_string());
        }
        Ok((r, g, b))
    }

    macro_rules! prompt_text_rgb {
        ( $i18n_bundle:expr, $label_id:ident ) => {{
            let label = i18n_msg!($i18n_bundle, $label_id);
            let mut args = FluentArgs::new();
            args.set("item_label", label);
            i18n_msg_format!($i18n_bundle, InputPromptColorRgb, args).to_string()
        }};
    }
    macro_rules! color_item_label {
        ( $i18n_bundle:expr, $label_id:ident ) => {{
            let label = i18n_msg!($i18n_bundle, $label_id);
            let mut args = FluentArgs::new();
            args.set("item_label", label);
            i18n_msg_format!($i18n_bundle, MenuItemGlowColors, args).to_string()
        }};
    }
    macro_rules! text_color_updated {
        ( $i18n_bundle:expr, $label_id:ident, $r:expr, $g:expr, $b:expr ) => {{
            let label = i18n_msg!($i18n_bundle, $label_id);
            let mut args = FluentArgs::new();
            args.set("item_label", label);
            args.set("r", $r);
            args.set("g", $g);
            args.set("b", $b);
            i18n_msg_format!($i18n_bundle, InfoGlowColorsUpdated, args).to_string()
        }};
    }

    MenuBuilder::new(MenuLevel::GlowColor, menu_fmt.clone())
        .title(i18n_msg!(menu_fmt, GlowColorMenuTitle))
        .add_input_item(
            menu_fmt.format_item_rgb(
                format!("1 - {}", color_item_label!(menu_fmt, ColorItemNotVizTarget)),
                (
                    settings.color_settings.glow_r_not,
                    settings.color_settings.glow_g_not,
                    settings.color_settings.glow_b_not,
                ),
            ),
            &prompt_text_rgb!(menu_fmt, ColorItemNotVizTarget),
            |_ctx, val, _| match parse_rgb(&val) {
                Ok((r, g, b)) => {
                    let settings = &mut lock_config!().settings;
                    (
                        settings.color_settings.glow_r_not,
                        settings.color_settings.glow_g_not,
                        settings.color_settings.glow_b_not,
                    ) = (r, g, b);
                    let i18n_bundle = &I18nBundle::new();
                    Some(text_color_updated!(
                        i18n_bundle,
                        ColorItemNotVizTarget,
                        settings.color_settings.glow_r_not,
                        settings.color_settings.glow_g_not,
                        settings.color_settings.glow_b_not
                    ))
                }
                Err(e) => Some(e),
            },
            (),
        )
        .add_input_item(
            menu_fmt.format_item_rgb(
                format!("2 - {}", color_item_label!(menu_fmt, ColorItemVizTarget)),
                (
                    settings.color_settings.glow_r_viz,
                    settings.color_settings.glow_g_viz,
                    settings.color_settings.glow_b_viz,
                ),
            ),
            &prompt_text_rgb!(menu_fmt, ColorItemVizTarget),
            |_ctx, val, _| match parse_rgb(&val) {
                Ok((r, g, b)) => {
                    let settings = &mut lock_config!().settings;
                    (
                        settings.color_settings.glow_r_viz,
                        settings.color_settings.glow_g_viz,
                        settings.color_settings.glow_b_viz,
                    ) = (r, g, b);
                    let i18n_bundle = &I18nBundle::new();
                    Some(text_color_updated!(
                        i18n_bundle,
                        ColorItemVizTarget,
                        settings.color_settings.glow_r_viz,
                        settings.color_settings.glow_g_viz,
                        settings.color_settings.glow_b_viz
                    ))
                }
                Err(e) => Some(e),
            },
            (),
        )
        .add_input_item(
            menu_fmt.format_item_rgb(
                format!(
                    "3 - {}",
                    color_item_label!(menu_fmt, ColorItemKnockedTarget)
                ),
                (
                    settings.color_settings.glow_r_knocked,
                    settings.color_settings.glow_g_knocked,
                    settings.color_settings.glow_b_knocked,
                ),
            ),
            &prompt_text_rgb!(menu_fmt, ColorItemKnockedTarget),
            |_ctx, val, _| match parse_rgb(&val) {
                Ok((r, g, b)) => {
                    let settings = &mut lock_config!().settings;
                    (
                        settings.color_settings.glow_r_knocked,
                        settings.color_settings.glow_g_knocked,
                        settings.color_settings.glow_b_knocked,
                    ) = (r, g, b);
                    let i18n_bundle = &I18nBundle::new();
                    Some(text_color_updated!(
                        i18n_bundle,
                        ColorItemKnockedTarget,
                        settings.color_settings.glow_r_knocked,
                        settings.color_settings.glow_g_knocked,
                        settings.color_settings.glow_b_knocked
                    ))
                }
                Err(e) => Some(e),
            },
            (),
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
