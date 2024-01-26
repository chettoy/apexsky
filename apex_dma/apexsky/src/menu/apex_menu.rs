use super::{alert, prompt};
use crate::{config, i18n::get_fluent_bundle, i18n_msg, i18n_msg_format, lock_config};
use chrono::Datelike;
use fluent::{FluentArgs, FluentBundle, FluentResource};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::{borrow::Cow, collections::HashMap, fmt::Debug, sync::Arc};
use unicode_width::UnicodeWidthStr;

mod loot_menu;
mod main_menu;
mod spectators_menu;

pub struct TerminalMenu<'a> {
    app_model: super::Model,
    menu_level: Vec<MenuLevel>,
    menu_state: Option<MenuState<'a>>,
    scroll_height: usize,
}

#[derive(Debug)]
struct CallbackItem<F, T>
where
    F: FnOnce(&mut TerminalMenu, T) -> Option<String>,
{
    callback: F,
    state: T,
}

trait CallbackTrait {
    fn call(&self, context: &mut TerminalMenu) -> Option<String>;
}

impl<F, T> CallbackTrait for CallbackItem<F, T>
where
    F: FnOnce(&mut TerminalMenu, T) -> Option<String> + Clone,
    T: Clone,
{
    fn call(&self, context: &mut TerminalMenu) -> Option<String> {
        (self.callback.to_owned())(context, self.state.clone())
    }
}

#[derive(Clone)]
pub(super) struct MenuState<'a> {
    title: Cow<'a, str>,
    items: Vec<ListItem<'a>>,
    handler: HashMap<usize, Arc<dyn CallbackTrait>>,
    input_handlers: HashMap<usize, (String, Box<fn(String) -> Option<String>>)>,
    num_ids: HashMap<usize, usize>, // id, index
    nav_index: usize,
    scroll_top: usize,
}

impl<'a> Debug for MenuState<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MenuState")
            .field("title", &self.title)
            .field("items", &self.items)
            .field("handler", &self.handler.keys())
            .field("input_handlers", &self.input_handlers.keys())
            .field("num_ids", &self.num_ids)
            .field("nav_index", &self.nav_index)
            .field("scroll_top", &self.scroll_top)
            .finish()
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub(super) enum MenuLevel {
    #[default]
    MainMenu,
    AimbotMenu,
    GlowColorMenu,
    ItemFilterMenu,
    LightWeaponsMenu,
    HeavyWeaponsMenu,
    EnergyWeaponsMenu,
    SniperWeaponsMenu,
    ArmorsMenu,
    HealingMenu,
    NadesMenu,
    BackpacksMenu,
    HopUpsMenu,
    ScopesMenu,
    KeyCodesMenu,
    HotkeyMenu,
    SpectatorsMenu,
}

impl<'a> TerminalMenu<'a> {
    pub fn new(app_model: super::Model) -> Self {
        let mut instance = Self {
            app_model,
            menu_level: Vec::new(),
            menu_state: None,
            scroll_height: 0,
        };
        instance.nav_menu(MenuLevel::MainMenu);
        instance
    }

    pub(crate) fn app_model(&self) -> &super::Model {
        &self.app_model
    }

    pub fn app_model_mut(&mut self) -> &mut super::Model {
        &mut self.app_model
    }

    pub fn resize(&mut self, f: &mut Frame) {
        self.scroll_height = (f.size().height - 4).into();
        if let Some(state) = &mut self.menu_state {
            if state.nav_index + 1 > self.scroll_height {
                state.scroll_top = (state.nav_index + 1) - self.scroll_height;
            } else {
                state.scroll_top = 0;
            }
        }
    }

    pub fn render(&self, f: &mut Frame) {
        self.render_menu(f);
    }

    pub fn nav_up(&mut self) {
        if let Some(state) = &mut self.menu_state {
            if state.nav_index > 0 {
                state.nav_index -= 1;
            }
        }
    }

    pub fn nav_down(&mut self) {
        if let Some(state) = &mut self.menu_state {
            if state.nav_index < state.items.len() - 1 {
                state.nav_index += 1;
            }
        }
    }

    pub fn nav_jump(&mut self, num: usize) {
        if let Some(state) = &mut self.menu_state {
            if let Some(index) = state.num_ids.get(&num) {
                if index < &state.items.len() {
                    state.nav_index = *index;
                }
            }
        }
    }

    pub fn nav_back(&mut self) {
        // Do nothing when the main menu is at the top‌​‌‌​​​‌‌‌‍‌​‌‌​‌​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌​‌‌‌‌‌‍‌​‌‌‌‌​​‌‌‍‌​‌‌​​​​‌‌‍‌​‌‌‌​​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌‌​​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​‌‌‍‌​‌‌​‌​​‌‌‍‌​‌‌​‌‌​‌‌‍‌​‌‌​​‌​‌‌‍‌​‌‌​‌‌‌​‌‍‌​‌‌‌​‌​‌‌ of the stack
        if self.get_menu_level() == MenuLevel::MainMenu {
            return;
        }
        if self.menu_level.pop().is_some() {
            self.menu_state = None;
            self.update_menu();
        } else {
            self.nav_menu(MenuLevel::MainMenu);
        }
    }

    pub fn nav_enter(&mut self) {
        if self.menu_state.is_none() {
            return;
        }
        let state = self.menu_state.to_owned().unwrap();

        if let Some(f) = state.handler.get(&state.nav_index) {
            let result = f.call(self);
            self.update_menu();
            if let Some(text) = result {
                alert(self.app_model_mut(), text);
            }
        } else if let Some((prompt_text, f)) = state.input_handlers.get(&state.nav_index) {
            prompt(self.app_model_mut(), prompt_text.to_owned(), **f);
        }
    }

    pub(super) fn nav_menu(&mut self, menu_level: MenuLevel) {
        if self.menu_level.is_empty() {
            self.menu_level.push(menu_level);
        }
        let nav_index = self.menu_state.as_ref().map_or_else(
            || 0,
            |state| {
                if self.get_menu_level() == menu_level {
                    state.nav_index
                } else {
                    0
                }
            },
        );
        // Move the target ‌​‌‌​​​‌‌‌‍‌​‌‌​‌​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌​‌‌‌‌‌‍‌​‌‌‌‌​​‌‌‍‌​‌‌​​​​‌‌‍‌​‌‌‌​​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌‌​​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​‌‌‍‌​‌‌​‌​​‌‌‍‌​‌‌​‌‌​‌‌‍‌​‌‌​​‌​‌‌‍‌​‌‌​‌‌‌​‌‍‌​‌‌‌​‌​‌‌menu to the top of the stack
        self.menu_level.retain(|&x| x != menu_level);
        self.menu_level.push(menu_level);

        let data = lock_config!().settings.to_owned();
        let i18n_bundle = get_fluent_bundle();
        let mut new_menu_state = match self.get_menu_level() {
            MenuLevel::MainMenu => main_menu::build_main_menu(i18n_bundle, data),
            MenuLevel::AimbotMenu => build_aimbot_menu(i18n_bundle, data),
            MenuLevel::GlowColorMenu => build_glow_color_menu(i18n_bundle, data),
            MenuLevel::ItemFilterMenu => loot_menu::build_item_filter_menu(i18n_bundle, data),
            MenuLevel::LightWeaponsMenu => loot_menu::build_light_weapons_menu(i18n_bundle, data),
            MenuLevel::HeavyWeaponsMenu => loot_menu::build_heavy_weapons_menu(i18n_bundle, data),
            MenuLevel::EnergyWeaponsMenu => loot_menu::build_energy_weapons_menu(i18n_bundle, data),
            MenuLevel::SniperWeaponsMenu => loot_menu::build_sniper_weapons_menu(i18n_bundle, data),
            MenuLevel::ArmorsMenu => loot_menu::build_armors_menu(i18n_bundle, data),
            MenuLevel::HealingMenu => loot_menu::build_healing_menu(i18n_bundle, data),
            MenuLevel::NadesMenu => loot_menu::build_nades_menu(i18n_bundle, data),
            MenuLevel::BackpacksMenu => loot_menu::build_backpacks_menu(i18n_bundle, data),
            MenuLevel::HopUpsMenu => loot_menu::build_hopups_menu(i18n_bundle, data),
            MenuLevel::ScopesMenu => loot_menu::build_scopes_menu(i18n_bundle, data),
            MenuLevel::KeyCodesMenu => build_key_codes_menu(i18n_bundle, data),
            MenuLevel::HotkeyMenu => build_hotkey_menu(i18n_bundle, data),
            MenuLevel::SpectatorsMenu => spectators_menu::build_spectators_menu(i18n_bundle, data),
        };
        new_menu_state.nav_index = nav_index;
        self.menu_state = Some(new_menu_state);
    }

    pub fn update_menu(&mut self) {
        self.nav_menu(self.get_menu_level());
    }

    fn get_menu_level(&self) -> MenuLevel {
        *self.menu_level.last().unwrap()
    }

    fn render_menu(&self, f: &mut Frame) {
        if self.menu_state.is_none() {
            return;
        }
        let state = self.menu_state.as_ref().unwrap();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(f.size());

        f.render_widget(block_title(state.title.to_owned()), chunks[0]);
        f.render_widget(
            render_selected_list(&state.items, state.nav_index, state.scroll_top),
            chunks[1],
        );
    }
}

pub struct MenuBuilder<'a> {
    title: Cow<'a, str>,
    list_items: Vec<ListItem<'a>>,
    handlers: HashMap<usize, Arc<dyn CallbackTrait>>,
    input_handlers: HashMap<usize, (String, Box<fn(String) -> Option<String>>)>,
    num_ids: HashMap<usize, usize>,
    head_id: usize,
}

impl<'a> Debug for MenuBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MenuBuilder")
            .field("title", &self.title)
            .field("list_items", &self.list_items)
            .field("handlers", &self.handlers.keys())
            .field("input_handlers", &self.input_handlers)
            .field("num_ids", &self.num_ids)
            .field("head_id", &self.head_id)
            .finish()
    }
}

impl<'a> MenuBuilder<'a> {
    pub(super) fn new() -> MenuBuilder<'a> {
        MenuBuilder {
            title: std::borrow::Cow::Borrowed(""),
            list_items: Vec::new(),
            handlers: HashMap::new(),
            input_handlers: HashMap::new(),
            num_ids: HashMap::new(),
            head_id: 0,
        }
    }

    pub(super) fn title<T>(mut self, value: T) -> MenuBuilder<'a>
    where
        T: Into<String>,
    {
        self.title = value.into().into();
        self
    }

    pub(super) fn add_item<F, T>(mut self, item: ListItem<'a>, handler: F, state: T) -> MenuBuilder
    where
        F: FnOnce(&mut TerminalMenu, T) -> Option<String> + Clone + 'static,
        T: Clone + 'static,
    {
        let num = self.next_id();
        self.add_numbered_item(num, item, handler, state)
    }

    pub(super) fn add_input_item(
        mut self,
        item: ListItem<'a>,
        prompt_text: &str,
        input_handler: fn(String) -> Option<String>,
    ) -> MenuBuilder<'a> {
        let num = self.next_id();
        self.add_numbered_input_item(num, item, prompt_text, input_handler)
    }

    pub(super) fn next_id(&mut self) -> usize {
        loop {
            self.head_id += 1;
            if !self.num_ids.contains_key(&self.head_id) {
                break;
            }
        }
        self.head_id
    }

    pub(super) fn skip_id(mut self) -> MenuBuilder<'a> {
        self.next_id();
        self
    }

    pub(super) fn no_id(mut self) -> MenuBuilder<'a> {
        self.num_ids.remove_entry(&self.head_id);
        self.head_id -= 1;
        self
    }

    pub(super) fn add_numbered_item<F, T>(
        mut self,
        num: usize,
        item: ListItem<'a>,
        handler: F,
        state: T,
    ) -> MenuBuilder
    where
        F: FnOnce(&mut TerminalMenu, T) -> Option<String> + Clone + 'static,
        T: Clone + 'static,
    {
        self.list_items.push(item);
        self.handlers.insert(
            self.list_items.len() - 1,
            Arc::new(CallbackItem {
                callback: handler,
                state,
            }),
        );
        self.num_ids.insert(num, self.list_items.len() - 1);
        self
    }

    pub(super) fn add_numbered_input_item(
        mut self,
        num: usize,
        item: ListItem<'a>,
        prompt_text: &str,
        input_handler: fn(String) -> Option<String>,
    ) -> MenuBuilder<'a> {
        self.list_items.push(item);
        self.input_handlers.insert(
            self.list_items.len() - 1,
            (String::from(prompt_text), Box::new(input_handler)),
        );
        self.num_ids.insert(num, self.list_items.len() - 1);
        self
    }

    pub(super) fn add_text_item<T>(mut self, label: T) -> MenuBuilder<'a>
    where
        T: Into<String>,
    {
        self.list_items.push(item_text(label));
        self
    }

    pub(super) fn add_dummy_item(mut self) -> MenuBuilder<'a> {
        self.list_items.push(item_dummy());
        self
    }
}

#[macro_export]
macro_rules! menu_add_toggle_item {
    ( $builder:ident, $i18n_bundle:expr, $label:expr, $value:expr, $x:ident ) => {{
        MenuBuilder::add_item(
            $builder,
            item_enabled($i18n_bundle, $label, $value),
            |_handle: &mut TerminalMenu, _| {
                let settings = &mut lock_config!().settings;
                settings.$x = !settings.$x;
                None
            },
            (),
        )
    }};
}

pub(super) enum LootLevel {
    White,
    Blue,
    Purple,
    Gold,
    Red,
}

#[macro_export]
macro_rules! menu_add_pick_item {
    ( $builder:ident, $i18n_bundle:expr, $label_prefix:expr, $label_id:ident, $value:expr, $x:ident ) => {{
        use ratatui::style::Color;
        let label = i18n_msg!($i18n_bundle, $label_id);
        let (pick_color, pick_mark) = if $value {
            (Color::Green, "[x]")
        } else {
            (Color::Red, "[ ]")
        };
        MenuBuilder::add_item(
            $builder,
            ListItem::new(Line::from(vec![
                Span::from($label_prefix),
                Span::styled(format!("{} ", label), Style::default().fg(pick_color)),
                Span::from(pick_mark),
            ])),
            |_handle: &mut TerminalMenu, _| {
                let settings = &mut lock_config!().settings;
                settings.loot.$x = !settings.loot.$x;
                None
            },
            (),
        )
    }};
}

#[macro_export]
macro_rules! menu_add_colored_loot_item {
    ( $builder:ident, $i18n_bundle:expr, $label_prefix:expr, $label_id:ident, $loot_level:expr, $value:expr, $x:ident ) => {{
        use ratatui::style::Color;
        let label = i18n_msg!($i18n_bundle, $label_id);
        let (color_label, color) = match $loot_level {
            LootLevel::White => (i18n_msg!($i18n_bundle, LootLevel1Name), Color::White),
            LootLevel::Blue => (i18n_msg!($i18n_bundle, LootLevel2Name), Color::Blue),
            LootLevel::Purple => (i18n_msg!($i18n_bundle, LootLevel3Name), Color::Magenta),
            LootLevel::Gold => (i18n_msg!($i18n_bundle, LootLevel4Name), Color::Yellow),
            LootLevel::Red => (i18n_msg!($i18n_bundle, LootLevel5Name), Color::Red),
        };
        let (pick_color, pick_mark) = if $value {
            (Color::Green, "[x]")
        } else {
            (Color::Red, "[‌​‌‌​​​‌‌‌‍‌​‌‌​‌​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌​‌‌‌‌‌‍‌​‌‌‌‌​​‌‌‍‌​‌‌​​​​‌‌‍‌​‌‌‌​​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌‌​​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​‌‌‍‌​‌‌​‌​​‌‌‍‌​‌‌​‌‌​‌‌‍‌​‌‌​​‌​‌‌‍‌​‌‌​‌‌‌​‌‍‌​‌‌‌​‌​‌‌ ]")
        };
        MenuBuilder::add_item(
            $builder,
            ListItem::new(Line::from(vec![
                Span::from($label_prefix),
                Span::styled(format!("{}: ", label), Style::default().fg(pick_color)),
                Span::styled(format!("{} ", color_label), Style::default().fg(color)),
                Span::from(pick_mark),
            ])),
            |_handle: &mut TerminalMenu, _| {
                let settings = &mut lock_config!().settings;
                settings.loot.$x = !settings.loot.$x;
                None
            },
            ()
        )
    }};
}

impl<'a> Into<MenuState<'a>> for MenuBuilder<'a> {
    fn into(self) -> MenuState<'a> {
        MenuState {
            title: self.title,
            items: self.list_items,
            handler: self.handlers,
            input_handlers: self.input_handlers,
            num_ids: self.num_ids,
            nav_index: 0,
            scroll_top: 0,
        }
    }
}

fn build_aimbot_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    settings: config::Settings,
) -> MenuState<'static> {
    let menu = MenuBuilder::new().title(i18n_msg!(i18n_bundle, AimbotMenuTitle));
    menu.add_item(
        item_enabled(
            &i18n_bundle,
            format!(" 1 - {}", i18n_msg!(i18n_bundle, MenuItemKeyboard)),
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
            format!(" 2 - {}", i18n_msg!(i18n_bundle, MenuItemGamepad)),
            settings.aimbot_settings.gamepad,
        ),
        |_, _| {
            let settings = &mut lock_config!().settings;
            settings.aimbot_settings.gamepad = !settings.aimbot_settings.gamepad;
            None
        },
        (),
    )
    .add_input_item(
        format_item(
            &i18n_bundle,
            format!(" 3 - {}", i18n_msg!(i18n_bundle, MenuItemAimbotMode)),
            match settings.aimbot_settings.aim_mode {
                0 => Span::from(i18n_msg!(i18n_bundle, MenuValueAimbotOff).to_string()),
                1 => Span::styled(
                    i18n_msg!(i18n_bundle, MenuValueAimbotNoVisCheck).to_string(),
                    Style::default().fg(Color::Red),
                ),
                2 => Span::styled(
                    i18n_msg!(i18n_bundle, MenuValueAimbotOn).to_string(),
                    Style::default().fg(Color::Green),
                ),
                _ => Span::styled(
                    std::borrow::Cow::Borrowed("!").to_string(),
                    Style::default().fg(Color::Red),
                ),
            },
        ),
        &i18n_msg!(i18n_bundle, InputPromptAimbotMode),
        |val| {
            let i18n_bundle = get_fluent_bundle();
            let val = val.trim();
            if let Some(new_val) = val.parse::<u8>().ok() {
                if vec![0, 1, 2].contains(&new_val) {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.aim_mode = new_val.into();
                    return None;
                }
                return Some(i18n_msg!(i18n_bundle, InfoInvalidValue).to_string());
            }
            Some(i18n_msg!(i18n_bundle, InfoInvalidValue).to_string())
        },
    )
    .add_input_item(
        format_item(
            &i18n_bundle,
            format!(" 4 - {}", i18n_msg!(i18n_bundle, MenuItemChangeAdsFov)),
            Span::from(format!("{}", settings.aimbot_settings.ads_fov)),
        ),
        &i18n_msg!(i18n_bundle, InputPromptAdsFov),
        |val| {
            if let Some(new_val) = val.parse::<f32>().ok() {
                if new_val >= 1.0 && new_val <= 50.0 {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.ads_fov = new_val;
                    return None;
                }
            }
            let i18n_bundle = get_fluent_bundle();
            Some(i18n_msg!(i18n_bundle, InfoInvalidAdsFov).to_string())
        },
    )
    .add_input_item(
        format_item(
            &i18n_bundle,
            format!(" 5 - {}", i18n_msg!(i18n_bundle, MenuItemChangeNonAdsFov)),
            Span::from(format!("{}", settings.aimbot_settings.non_ads_fov)),
        ),
        &i18n_msg!(i18n_bundle, InputPromptNonAdsFov),
        |val| {
            if let Some(new_val) = val.parse::<f32>().ok() {
                if new_val >= 1.0 && new_val <= 50.0 {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.non_ads_fov = new_val;
                    return None;
                }
            }
            let i18n_bundle = get_fluent_bundle();
            Some(i18n_msg!(i18n_bundle, InfoInvalidNonAdsFov).to_string())
        },
    )
    .add_item(
        format_item(
            &i18n_bundle,
            format!(" 6 - {}", i18n_msg!(i18n_bundle, MenuItemToggleNadeAim)),
            Span::from(
                if !settings.aimbot_settings.auto_nade_aim {
                    i18n_msg!(i18n_bundle, MenuValueNoNadeAim)
                } else {
                    i18n_msg!(i18n_bundle, MenuValueNadeAimOn)
                }
                .to_string(),
            ),
        ),
        |_, _| {
            let settings = &mut lock_config!().settings;
            settings.aimbot_settings.auto_nade_aim = !settings.aimbot_settings.auto_nade_aim;
            None
        },
        (),
    )
    .add_item(
        item_enabled(
            &i18n_bundle,
            format!(" 7 - {}", i18n_msg!(i18n_bundle, MenuItemToggleNoRecoil)),
            settings.aimbot_settings.no_recoil,
        ),
        |_handle: &mut TerminalMenu, _| {
            let settings = &mut lock_config!().settings;
            settings.aimbot_settings.no_recoil = !settings.aimbot_settings.no_recoil;
            None
        },
        (),
    )
    .add_input_item(
        format_item(
            &i18n_bundle,
            format!(" 8 - {}", i18n_msg!(i18n_bundle, MenuItemChangeBoneAim)),
            Span::from(
                if settings.aimbot_settings.bone_nearest {
                    i18n_msg!(i18n_bundle, MenuValueBoneNearest)
                } else if settings.aimbot_settings.bone_auto {
                    i18n_msg!(i18n_bundle, MenuValueBoneAuto)
                } else {
                    match settings.aimbot_settings.bone {
                        0 => i18n_msg!(i18n_bundle, MenuValueBoneHead),
                        1 => i18n_msg!(i18n_bundle, MenuValueBoneNeck),
                        2 => i18n_msg!(i18n_bundle, MenuValueBoneChest),
                        3 => i18n_msg!(i18n_bundle, MenuValueBoneGutShut),
                        _ => i18n_msg!(i18n_bundle, MenuValueBoneUnknown),
                    }
                }
                .to_string(),
            ),
        ),
        &i18n_msg!(i18n_bundle, InputPromptBoneValue),
        |val| {
            let i18n_bundle = get_fluent_bundle();
            let val = val.trim();
            if val == "x" {
                let settings = &mut lock_config!().settings;
                settings.aimbot_settings.bone_auto = true;
                settings.aimbot_settings.bone_nearest = false;
                return None;
            } else if val == "h" {
                let settings = &mut lock_config!().settings;
                settings.aimbot_settings.bone_nearest = true;
                settings.aimbot_settings.bone_auto = false;
                return None;
            } else if let Some(new_val) = val.parse::<u8>().ok() {
                if vec![0, 1, 2, 3].contains(&new_val) {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.bone = new_val.into();
                    settings.aimbot_settings.bone_auto = false;
                    return None;
                }
                return Some(i18n_msg!(i18n_bundle, InfoInvalidBoneValue).to_string());
            }
            Some(i18n_msg!(i18n_bundle, InfoInvalidValue).to_string())
        },
    )
    .add_input_item(
        format_item(
            &i18n_bundle,
            format!(" 9 - {}", i18n_msg!(i18n_bundle, MenuItemAimDist)),
            Span::from(format!("{}m", settings.aimbot_settings.aim_dist / 39.62)),
        ),
        &i18n_msg!(i18n_bundle, InputPromptAimDist),
        |val| {
            if let Some(new_val) = val.parse::<f32>().ok() {
                if new_val >= 10.0 && new_val <= 1600.0 {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.aim_dist = new_val * 39.62;
                    return None;
                }
            }
            None
        },
    )
    .add_input_item(
        format_item(
            &i18n_bundle,
            format!("10 - {}", i18n_msg!(i18n_bundle, MenuItemHeadshotDist)),
            Span::from(format!(
                "{}m",
                settings.aimbot_settings.headshot_dist / 39.62
            )),
        ),
        &i18n_msg!(i18n_bundle, InputPromptHeadshotDist),
        |val| {
            if let Some(new_val) = val.parse::<f32>().ok() {
                if new_val >= 0.0 && new_val <= 1600.0 {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.headshot_dist = new_val * 39.62;
                    return None;
                }
            }
            None
        },
    )
    .add_input_item(
        format_item(
            &i18n_bundle,
            format!("11 - {}", i18n_msg!(i18n_bundle, MenuItemSmoothValue)),
            if settings.aimbot_settings.smooth < 120.0 {
                Span::styled(
                    format!("{}", settings.aimbot_settings.smooth),
                    Style::default().fg(Color::Red),
                )
            } else if settings.aimbot_settings.smooth >= 160.0 {
                Span::styled(
                    format!("{}", settings.aimbot_settings.smooth),
                    Style::default().fg(Color::Green),
                )
            } else {
                Span::from(format!("{}", settings.aimbot_settings.smooth))
            },
        ),
        &i18n_msg!(i18n_bundle, InputPromptSmoothValue),
        |val| {
            if let Some(new_val) = val.parse::<u16>().ok() {
                if new_val >= 50 && new_val <= 500 {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.smooth = new_val.into();
                    // settings.aimbot_settings.skynade_smooth =
                    //     settings.aimbot_settings.smooth * 0.6667;
                    return None;
                }
            }
            let i18n_bundle = get_fluent_bundle();
            Some(i18n_msg!(i18n_bundle, InfoInvalidSmoothValue).to_string())
        },
    )
    .add_input_item(
        format_item(
            &i18n_bundle,
            format!("12 - {}", i18n_msg!(i18n_bundle, MenuItemSkynadeSmooth)),
            if settings.aimbot_settings.skynade_smooth < 90.0 * 0.6667 {
                Span::styled(
                    format!("{}", settings.aimbot_settings.skynade_smooth),
                    Style::default().fg(Color::Red),
                )
            } else if settings.aimbot_settings.skynade_smooth > 120.0 * 0.6667 {
                Span::styled(
                    format!("{}", settings.aimbot_settings.skynade_smooth),
                    Style::default().fg(Color::Green),
                )
            } else {
                Span::from(format!("{}", settings.aimbot_settings.skynade_smooth))
            },
        ),
        &i18n_msg!(i18n_bundle, InputPromptSmoothValue),
        |val| {
            if let Some(new_val) = val.parse::<u16>().ok() {
                if new_val >= 50 && new_val <= 500 {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.skynade_smooth = new_val.into();
                    return None;
                }
            }
            let i18n_bundle = get_fluent_bundle();
            Some(i18n_msg!(i18n_bundle, InfoInvalidSmoothValue).to_string())
        },
    )
    .add_input_item(
        format_item(
            &i18n_bundle,
            format!("13 - {}", i18n_msg!(i18n_bundle, MenuItemRecoilXValue)),
            if settings.aimbot_settings.recoil_smooth_x > 70.0 {
                Span::styled(
                    format!("{}%", settings.aimbot_settings.recoil_smooth_x),
                    Style::default().fg(Color::Red),
                )
            } else if settings.aimbot_settings.recoil_smooth_x <= 70.0 {
                Span::styled(
                    format!("{}%", settings.aimbot_settings.recoil_smooth_x),
                    Style::default().fg(Color::Green),
                )
            } else {
                Span::from(format!("{}%", settings.aimbot_settings.recoil_smooth_x))
            },
        ),
        &i18n_msg!(i18n_bundle, InputPromptRecoilValue),
        |val| {
            if let Some(new_val) = val.parse::<f32>().ok() {
                if new_val >= 0.0 && new_val <= 100.0 {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.recoil_smooth_x = new_val.into();
                    return None;
                }
            }
            let i18n_bundle = get_fluent_bundle();
            Some(i18n_msg!(i18n_bundle, InfoInvalidRecoilValue).to_string())
        },
    )
    .add_input_item(
        format_item(
            &i18n_bundle,
            format!("14 - {}", i18n_msg!(i18n_bundle, MenuItemRecoilYValue)),
            if settings.aimbot_settings.recoil_smooth_y > 70.0 {
                Span::styled(
                    format!("{}%", settings.aimbot_settings.recoil_smooth_y),
                    Style::default().fg(Color::Red),
                )
            } else if settings.aimbot_settings.recoil_smooth_y <= 70.0 {
                Span::styled(
                    format!("{}%", settings.aimbot_settings.recoil_smooth_y),
                    Style::default().fg(Color::Green),
                )
            } else {
                Span::from(format!("{}%", settings.aimbot_settings.recoil_smooth_y))
            },
        ),
        &i18n_msg!(i18n_bundle, InputPromptRecoilValue),
        |val| {
            if let Some(new_val) = val.parse::<f32>().ok() {
                if new_val >= 0.0 && new_val <= 100.0 {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.recoil_smooth_y = new_val.into();
                    return None;
                }
            }
            let i18n_bundle = get_fluent_bundle();
            Some(i18n_msg!(i18n_bundle, InfoInvalidRecoilValue).to_string())
        },
    )
    .into()
}

fn build_glow_color_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    settings: config::Settings,
) -> MenuState<'static> {
    fn parse_rgb(val: &String) -> Result<(f32, f32, f32), String> {
        let i18n_bundle = get_fluent_bundle();
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

    MenuBuilder::new()
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
                    let i18n_bundle = get_fluent_bundle();
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
                    let i18n_bundle = get_fluent_bundle();
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
                    let i18n_bundle = get_fluent_bundle();
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

fn build_hotkey_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    settings: config::Settings,
) -> MenuState<'static> {
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

    MenuBuilder::new()
        .title(i18n_msg!(i18n_bundle, HotkeyMenuTitle))
        .add_input_item(
            menu_item_keycode(
                format!("1 - {}", i18n_msg!(i18n_bundle, HotkeyItemAimbot1)),
                settings.aimbot_hot_key_1,
            ),
            &prompt_text_keycode!(i18n_bundle, HotkeyItemAimbot1),
            |val| {
                if let Some(keycode) = val.parse::<u8>().ok() {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_hot_key_1 = keycode as i32;
                    return None;
                }
                let i18n_bundle = get_fluent_bundle();
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
                if let Some(keycode) = val.parse::<u8>().ok() {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_hot_key_2 = keycode as i32;
                    return None;
                }
                let i18n_bundle = get_fluent_bundle();
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
                if let Some(keycode) = val.parse::<u8>().ok() {
                    let settings = &mut lock_config!().settings;
                    settings.trigger_bot_hot_key = keycode as i32;
                    return None;
                }
                let i18n_bundle = get_fluent_bundle();
                Some(text_invalid_keycode!(i18n_bundle, HotkeyItemTriggerBot))
            },
        )
        .add_dummy_item()
        .add_item(
            item_text(format!("4 - {}", i18n_msg!(i18n_bundle, MenuItemKeyCodes))),
            |handler: &mut TerminalMenu, _| {
                handler.nav_menu(MenuLevel::KeyCodesMenu);
                None
            },
            (),
        )
        .add_dummy_item()
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

fn build_key_codes_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    _settings: config::Settings,
) -> MenuState<'static> {
    MenuBuilder::new()
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

fn render_selected_list<'a>(
    list_items: &'a Vec<ListItem<'a>>,
    selected_index: usize,
    scroll_top: usize,
) -> List<'a> {
    let now = chrono::Local::now();
    List::new(
        list_items
            .iter()
            .skip(scroll_top)
            .enumerate()
            .map(|(index, item)| {
                if index == selected_index - scroll_top {
                    if (now.month() == 12 && now.day() == 25)
                        || chinese_lunisolar_calendar::LunisolarDate::from_date(now)
                            .unwrap()
                            .the_n_day_in_this_year()
                            < 16
                    {
                        item.clone().white().bold().on_red()
                    } else {
                        item.clone().black().bold().on_light_yellow()
                    }
                } else {
                    item.clone()
                }
            })
            .collect::<Vec<ListItem>>(),
    )
}

fn format_label<T>(label: T) -> Span<'static>
where
    T: Into<String>,
{
    Span::from({
        //format!("{: <40}", label.into())
        const LABEL_SIZE: usize = 40;
        let mut labal_text: String = label.into();
        let label_width = UnicodeWidthStr::width(labal_text.as_str());
        if label_width < LABEL_SIZE {
            let space_count = LABEL_SIZE - label_width;
            labal_text += &(" ".repeat(space_count));
        }
        labal_text
    })
}
pub(super) fn format_item<'a, T>(
    i18n_bundle: &FluentBundle<FluentResource>,
    label: T,
    value: Span<'a>,
) -> ListItem<'a>
where
    T: Into<String>,
{
    ListItem::new(Line::from(vec![
        format_label(label.into()),
        Span::styled(
            i18n_msg!(i18n_bundle, MenuValuePrefix).to_string(),
            Style::default().fg(Color::DarkGray),
        ),
        value,
        Span::styled(
            i18n_msg!(i18n_bundle, MenuValueSuffix).to_string(),
            Style::default().fg(Color::DarkGray),
        ),
    ]))
}
fn span_enabled(i18n_bundle: &FluentBundle<FluentResource>, v: bool) -> Span<'static> {
    if v {
        Span::styled(
            i18n_msg!(i18n_bundle, MenuValueEnabled).to_string(),
            Style::default().fg(Color::Green),
        )
    } else {
        Span::from(i18n_msg!(i18n_bundle, MenuValueDisabled).to_string())
    }
}
pub(super) fn item_enabled<T>(
    i18n_bundle: &FluentBundle<FluentResource>,
    label: T,
    v: bool,
) -> ListItem<'static>
where
    T: Into<String>,
{
    format_item(i18n_bundle, label, span_enabled(i18n_bundle, v))
}
pub(super) fn item_text<T>(label: T) -> ListItem<'static>
where
    T: Into<String>,
{
    ListItem::new(Line::from(format_label(label)))
}
pub(super) fn item_dummy() -> ListItem<'static> {
    ListItem::new(Line::from("‌​‌‌​​​‌‌‌‍‌​‌‌​‌​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌​‌‌‌‌‌‍‌​‌‌‌‌​​‌‌‍‌​‌‌​​​​‌‌‍‌​‌‌‌​​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌‌​​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​‌‌‍‌​‌‌​‌​​‌‌‍‌​‌‌​‌‌​‌‌‍‌​‌‌​​‌​‌‌‍‌​‌‌​‌‌‌​‌‍‌​‌‌‌​‌​‌‌"))
}
fn block_title<T>(title: T) -> Paragraph<'static>
where
    T: Into<String>,
{
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        title.into(),
        Style::default().fg(Color::Green),
    ))
    .block(title_block);
    title
}
