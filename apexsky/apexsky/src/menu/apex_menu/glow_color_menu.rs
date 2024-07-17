use fluent::FluentArgs;
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::ListItem,
};

use super::{format_label, item_text, GeneralMenu, MenuBuilder, MenuLevel, TerminalMenu};
use crate::{config, i18n::I18nBundle, i18n_msg, i18n_msg_format, lock_config};

pub(super) fn build_glow_color_menu(
    i18n_bundle: &I18nBundle,
    settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    fn parse_rgb(val: &String) -> Result<(f32, f32, f32), String> {
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
        if r < 0.0 || r > 1.0 || g < 0.0 || g > 1.0 || b < 0.0 || b > 1.0 {
            return Err(i18n_msg!(i18n_bundle, InfoValuesOutOfRange).to_string());
        }
        Ok((r, g, b))
    }
    fn menu_item_rgb(label: String, (r, g, b): (f32, f32, f32)) -> ListItem<'static> {
        ListItem::new(Line::from(vec![
            format_label(label),
            Span::styled(
                format!("{},{},{}", r, g, b),
                Style::default()
                    .bg(Color::Rgb(
                        (r * 255.0) as u8,
                        (g * 255.0) as u8,
                        (b * 255.0) as u8,
                    ))
                    .black(),
            ),
        ]))
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

    MenuBuilder::new(MenuLevel::GlowColorMenu)
        .title(i18n_msg!(i18n_bundle, GlowColorMenuTitle))
        .add_input_item(
            menu_item_rgb(
                format!(
                    "1 - {}",
                    color_item_label!(i18n_bundle, ColorItemNotVizTarget)
                ),
                (
                    settings.glow_r_not,
                    settings.glow_g_not,
                    settings.glow_b_not,
                ),
            ),
            &prompt_text_rgb!(i18n_bundle, ColorItemNotVizTarget),
            |val| match parse_rgb(&val) {
                Ok((r, g, b)) => {
                    let settings = &mut lock_config!().settings;
                    (
                        settings.glow_r_not,
                        settings.glow_g_not,
                        settings.glow_b_not,
                    ) = (r, g, b);
                    let i18n_bundle = &I18nBundle::new();
                    Some(text_color_updated!(
                        i18n_bundle,
                        ColorItemNotVizTarget,
                        settings.glow_r_not,
                        settings.glow_g_not,
                        settings.glow_b_not
                    ))
                }
                Err(e) => Some(e),
            },
        )
        .add_input_item(
            menu_item_rgb(
                format!("2 - {}", color_item_label!(i18n_bundle, ColorItemVizTarget)),
                (
                    settings.glow_r_viz,
                    settings.glow_g_viz,
                    settings.glow_b_viz,
                ),
            ),
            &prompt_text_rgb!(i18n_bundle, ColorItemVizTarget),
            |val| match parse_rgb(&val) {
                Ok((r, g, b)) => {
                    let settings = &mut lock_config!().settings;
                    (
                        settings.glow_r_viz,
                        settings.glow_g_viz,
                        settings.glow_b_viz,
                    ) = (r, g, b);
                    let i18n_bundle = &I18nBundle::new();
                    Some(text_color_updated!(
                        i18n_bundle,
                        ColorItemVizTarget,
                        settings.glow_r_viz,
                        settings.glow_g_viz,
                        settings.glow_b_viz
                    ))
                }
                Err(e) => Some(e),
            },
        )
        .add_input_item(
            menu_item_rgb(
                format!(
                    "3 - {}",
                    color_item_label!(i18n_bundle, ColorItemKnockedTarget)
                ),
                (
                    settings.glow_r_knocked,
                    settings.glow_g_knocked,
                    settings.glow_b_knocked,
                ),
            ),
            &prompt_text_rgb!(i18n_bundle, ColorItemKnockedTarget),
            |val| match parse_rgb(&val) {
                Ok((r, g, b)) => {
                    let settings = &mut lock_config!().settings;
                    (
                        settings.glow_r_knocked,
                        settings.glow_g_knocked,
                        settings.glow_b_knocked,
                    ) = (r, g, b);
                    let i18n_bundle = &I18nBundle::new();
                    Some(text_color_updated!(
                        i18n_bundle,
                        ColorItemKnockedTarget,
                        settings.glow_r_knocked,
                        settings.glow_g_knocked,
                        settings.glow_b_knocked
                    ))
                }
                Err(e) => Some(e),
            },
        )
        .add_dummy_item()
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
