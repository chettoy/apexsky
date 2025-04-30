use crate::{
    i18n::{I18nBundle, MessageId, load_fluent_bundle},
    i18n_msg, lock_config,
};

pub use ohosky_menu::ratatui;
use ohosky_menu::{
    MenuState, TerminalMenu,
    general_menu::{GeneralMenu, GeneralMenuBuilder, GeneralMenuFormat, GeneralMenuName},
};

mod aimbot_menu;
mod glow_color_menu;
mod glow_features_menu;
mod hotkey_menu;
mod loot_menu;
mod main_menu;
mod players_menu;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MenuLevel {
    #[default]
    Main,
    Aimbot,
    GlowFeats,
    GlowColor,
    ItemFilter,
    LightWeapons,
    HeavyWeapons,
    EnergyWeapons,
    SniperWeapons,
    Armors,
    Healing,
    Grenades,
    Backpacks,
    HopUps,
    Scopes,
    KeyCodes,
    Hotkey,
    Players,
}

impl GeneralMenuName for MenuLevel {
    fn rebuild_state(self) -> Box<dyn MenuState> {
        self.into()
    }
}

impl From<MenuLevel> for Box<GeneralMenu<'_, MenuLevel>> {
    fn from(value: MenuLevel) -> Self {
        let data = lock_config!().settings.to_owned();
        let i18n_bundle = load_fluent_bundle();
        let menu_fmt = MenuFormatter { i18n_bundle };
        Box::new(match value {
            MenuLevel::Main => main_menu::build_main_menu(&menu_fmt, data),
            MenuLevel::Aimbot => aimbot_menu::build_aimbot_menu(&menu_fmt, data),
            MenuLevel::GlowFeats => glow_features_menu::build_glow_features_menu(&menu_fmt, data),
            MenuLevel::GlowColor => glow_color_menu::build_glow_color_menu(&menu_fmt, data),
            MenuLevel::ItemFilter => loot_menu::build_item_filter_menu(&menu_fmt, data),
            MenuLevel::LightWeapons => loot_menu::build_light_weapons_menu(&menu_fmt, data),
            MenuLevel::HeavyWeapons => loot_menu::build_heavy_weapons_menu(&menu_fmt, data),
            MenuLevel::EnergyWeapons => loot_menu::build_energy_weapons_menu(&menu_fmt, data),
            MenuLevel::SniperWeapons => loot_menu::build_sniper_weapons_menu(&menu_fmt, data),
            MenuLevel::Armors => loot_menu::build_armors_menu(&menu_fmt, data),
            MenuLevel::Healing => loot_menu::build_healing_menu(&menu_fmt, data),
            MenuLevel::Grenades => loot_menu::build_nades_menu(&menu_fmt, data),
            MenuLevel::Backpacks => loot_menu::build_backpacks_menu(&menu_fmt, data),
            MenuLevel::HopUps => loot_menu::build_hopups_menu(&menu_fmt, data),
            MenuLevel::Scopes => loot_menu::build_scopes_menu(&menu_fmt, data),
            MenuLevel::KeyCodes => hotkey_menu::build_key_codes_menu(&menu_fmt, data),
            MenuLevel::Hotkey => hotkey_menu::build_hotkey_menu(&menu_fmt, data),
            MenuLevel::Players => players_menu::build_players_menu(&menu_fmt, data),
        })
    }
}

impl From<MenuLevel> for Box<dyn MenuState> {
    fn from(val: MenuLevel) -> Self {
        let menu: Box<GeneralMenu<_>> = val.into();
        menu
    }
}

// impls MenuFormatter

#[derive(Debug, Clone)]
struct MenuFormatter {
    i18n_bundle: I18nBundle,
}

impl std::ops::Deref for MenuFormatter {
    type Target = I18nBundle;

    fn deref(&self) -> &Self::Target {
        &self.i18n_bundle
    }
}

impl MenuFormatter {
    fn format_item_rgb<S: Into<String>>(
        &self,
        label: S,
        (r, g, b): (f32, f32, f32),
    ) -> ratatui::widgets::ListItem<'static> {
        use ratatui::prelude::*;
        ratatui::widgets::ListItem::new(Line::from(vec![
            self.format_label(label),
            Span::styled(
                format!("{r},{g},{b}"),
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

    fn format_item_label_and_value<S: Into<String>, V: std::fmt::Display>(
        &self,
        label: S,
        value: V,
    ) -> ratatui::widgets::ListItem<'static> {
        use ratatui::prelude::*;
        ratatui::widgets::ListItem::new(Line::from(vec![
            self.format_label(label),
            Span::styled(format!("{value}"), Style::default().underlined()),
        ]))
    }
}

impl GeneralMenuFormat for MenuFormatter {
    fn format_item<'a, S: Into<String>>(
        &self,
        label: S,
        value: ratatui::prelude::Span<'a>,
    ) -> ratatui::widgets::ListItem<'a> {
        use ratatui::prelude::*;
        ratatui::widgets::ListItem::new(ratatui::prelude::Line::from(std::vec![
            self.format_label(label),
            Span::styled(
                i18n_msg!(&self.i18n_bundle, MenuValuePrefix).to_string(),
                Style::default().fg(Color::DarkGray)
            ),
            value,
            Span::styled(
                i18n_msg!(&self.i18n_bundle, MenuValueSuffix).to_string(),
                Style::default().fg(Color::DarkGray)
            ),
        ]))
    }

    fn span_enabled(&self, v: bool) -> ratatui::prelude::Span<'static> {
        if v {
            ratatui::prelude::Span::styled(
                i18n_msg!(&self.i18n_bundle, MenuValueEnabled).to_string(),
                ratatui::prelude::Style::default().fg(ratatui::prelude::Color::Green),
            )
        } else {
            ratatui::prelude::Span::from(
                i18n_msg!(&self.i18n_bundle, MenuValueDisabled).to_string(),
            )
        }
    }
}

/// MenuBuilder
/// ```no_run
/// let menu_fmt = MenuFormatter { i18n_bundle: load_fluent_bundle() };
///
/// let menu: GeneralMenu<'static, MenuLevel> =
///     MenuBuilder::new(MenuLevel::MainMenu, menu_fmt)
///         .add_toggle_item(
///             "item",
///             settings.debug_mode,
///             handler_toggle_settings!(.debug_mode),
///         )
///         .into();
/// ```
type MenuBuilder<'a> = GeneralMenuBuilder<'a, MenuFormatter, MenuLevel>;

macro_rules! handler_toggle_settings {
    ( $(.$x:ident)+ ) => {{
        |_handle: &mut TerminalMenu, _| {
            let settings = &mut lock_config!().settings;
            settings$(.$x)+ = !settings$(.$x)+;
            None
        }
    }};
}

use handler_toggle_settings;

trait MenuBuilderExt {
    fn add_toggle_item<S, F>(self, label: S, value: bool, handler: F) -> Self
    where
        S: Into<String>,
        F: FnOnce(&mut TerminalMenu, ()) -> Option<String> + Clone + 'static;
}

impl MenuBuilderExt for MenuBuilder<'_> {
    fn add_toggle_item<S, F>(self, label: S, value: bool, handler: F) -> Self
    where
        S: Into<String>,
        F: FnOnce(&mut TerminalMenu, ()) -> Option<String> + Clone + 'static,
    {
        let item = self.get_menu_formatter().item_enabled(label, value);
        self.add_item(item, handler, ())
    }
}

enum LootLevel {
    White,
    Blue,
    Purple,
    Gold,
    Red,
}

trait MenuBuilderLootsExt {
    fn add_pick_item<S, F>(self, label_prefix: S, loot: MessageId, value: bool, handler: F) -> Self
    where
        S: Into<String>,
        F: FnOnce(&mut TerminalMenu, ()) -> Option<String> + Clone + 'static;

    fn add_colored_loot_item<S, F>(
        self,
        label_prefix: S,
        loot: MessageId,
        loot_level: LootLevel,
        value: bool,
        handler: F,
    ) -> Self
    where
        S: Into<String>,
        F: FnOnce(&mut TerminalMenu, ()) -> Option<String> + Clone + 'static;
}

impl MenuBuilderLootsExt for MenuBuilder<'_> {
    fn add_pick_item<S, F>(self, label_prefix: S, loot: MessageId, value: bool, handler: F) -> Self
    where
        S: Into<String>,
        F: FnOnce(&mut TerminalMenu, ()) -> Option<String> + Clone + 'static,
    {
        use ratatui::prelude::*;
        let menu_fmt = self.get_menu_formatter();
        let label = menu_fmt.msg(loot).to_string();
        let (pick_color, pick_mark) = if value {
            (Color::Green, "[x]")
        } else {
            (Color::Red, "[ ]")
        };
        self.add_item(
            ratatui::widgets::ListItem::new(Line::from(vec![
                Span::from(label_prefix.into()),
                Span::styled(label, Style::default().fg(pick_color)),
                Span::from(pick_mark),
            ])),
            handler,
            (),
        )
    }

    fn add_colored_loot_item<S, F>(
        self,
        label_prefix: S,
        loot: MessageId,
        loot_level: LootLevel,
        value: bool,
        handler: F,
    ) -> Self
    where
        S: Into<String>,
        F: FnOnce(&mut TerminalMenu, ()) -> Option<String> + Clone + 'static,
    {
        use ratatui::prelude::*;
        let (label, color_label, color) = {
            let menu_fmt = self.get_menu_formatter();
            let label = menu_fmt.msg(loot);
            let (color_label, color) = match loot_level {
                LootLevel::White => (i18n_msg!(menu_fmt, LootLevel1Name), Color::White),
                LootLevel::Blue => (i18n_msg!(menu_fmt, LootLevel2Name), Color::Blue),
                LootLevel::Purple => (i18n_msg!(menu_fmt, LootLevel3Name), Color::Magenta),
                LootLevel::Gold => (i18n_msg!(menu_fmt, LootLevel4Name), Color::Yellow),
                LootLevel::Red => (i18n_msg!(menu_fmt, LootLevel5Name), Color::Red),
            };
            (label.to_string(), color_label.to_string(), color)
        };
        let (pick_color, pick_mark) = if value {
            (Color::Green, "[x]")
        } else {
            #[allow(clippy::invisible_characters)]
            (Color::Red, "[‌​‌‌​​​‌‌‌‍‌​‌‌​‌​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌​‌‌‌‌‌‍‌​‌‌‌‌​​‌‌‍‌​‌‌​​​​‌‌‍‌​‌‌‌​​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌‌​​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​‌‌‍‌​‌‌​‌​​‌‌‍‌​‌‌​‌‌​‌‌‍‌​‌‌​​‌​‌‌‍‌​‌‌​‌‌‌​‌‍‌​‌‌‌​‌​‌‌ ]")
        };
        self.add_item(
            ratatui::widgets::ListItem::new(Line::from(vec![
                Span::from(label_prefix.into()),
                Span::styled(format!("{label}: "), Style::default().fg(pick_color)),
                Span::styled(format!("{color_label} "), Style::default().fg(color)),
                Span::from(pick_mark),
            ])),
            handler,
            (),
        )
    }
}
