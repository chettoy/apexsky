use super::{alert, prompt};
use crate::{
    config, global_state::G_CONTEXT, i18n::get_fluent_bundle, i18n_msg, i18n_msg_format,
    lock_config,
};
use chrono::Datelike;
use fluent::{FluentArgs, FluentBundle, FluentResource};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::{borrow::Cow, collections::HashMap, fmt::Debug};
use unicode_width::UnicodeWidthStr;

pub struct TerminalMenu<'a> {
    app_model: super::Model,
    menu_level: Vec<MenuLevel>,
    menu_state: Option<MenuState<'a>>,
    scroll_height: usize,
}

#[derive(Clone)]
struct MenuState<'a> {
    title: Cow<'a, str>,
    items: Vec<ListItem<'a>>,
    handler: HashMap<usize, Box<fn(&mut TerminalMenu<'_>) -> Option<String>>>,
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
enum MenuLevel {
    #[default]
    MainMenu,
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
    ScopesMenu,
    KeyCodesMenu,
    HotkeyMenu,
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
            let result = f.to_owned()(self);
            self.update_menu();
            if let Some(text) = result {
                alert(self.app_model_mut(), text);
            }
        } else if let Some((prompt_text, f)) = state.input_handlers.get(&state.nav_index) {
            prompt(self.app_model_mut(), prompt_text.to_owned(), **f);
        }
    }

    fn nav_menu(&mut self, menu_level: MenuLevel) {
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
            MenuLevel::MainMenu => build_main_menu(i18n_bundle, data),
            MenuLevel::GlowColorMenu => build_glow_color_menu(i18n_bundle, data),
            MenuLevel::ItemFilterMenu => build_item_filter_menu(i18n_bundle, data),
            MenuLevel::LightWeaponsMenu => build_light_weapons_menu(i18n_bundle, data),
            MenuLevel::HeavyWeaponsMenu => build_heavy_weapons_menu(i18n_bundle, data),
            MenuLevel::EnergyWeaponsMenu => build_energy_weapons_menu(i18n_bundle, data),
            MenuLevel::SniperWeaponsMenu => build_sniper_weapons_menu(i18n_bundle, data),
            MenuLevel::ArmorsMenu => build_armors_menu(i18n_bundle, data),
            MenuLevel::HealingMenu => build_healing_menu(i18n_bundle, data),
            MenuLevel::NadesMenu => build_nades_menu(i18n_bundle, data),
            MenuLevel::BackpacksMenu => build_backpacks_menu(i18n_bundle, data),
            MenuLevel::ScopesMenu => build_scopes_menu(i18n_bundle, data),
            MenuLevel::KeyCodesMenu => build_key_codes_menu(i18n_bundle, data),
            MenuLevel::HotkeyMenu => build_hotkey_menu(i18n_bundle, data),
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

#[derive(Debug)]
pub struct MenuBuilder<'a> {
    title: Cow<'a, str>,
    list_items: Vec<ListItem<'a>>,
    handlers: HashMap<usize, Box<fn(&mut TerminalMenu<'_>) -> Option<String>>>,
    input_handlers: HashMap<usize, (String, Box<fn(String) -> Option<String>>)>,
    num_ids: HashMap<usize, usize>,
    head_id: usize,
}

impl<'a> MenuBuilder<'a> {
    fn new() -> MenuBuilder<'a> {
        MenuBuilder {
            title: std::borrow::Cow::Borrowed(""),
            list_items: Vec::new(),
            handlers: HashMap::new(),
            input_handlers: HashMap::new(),
            num_ids: HashMap::new(),
            head_id: 0,
        }
    }

    fn title<T>(mut self, value: T) -> MenuBuilder<'a>
    where
        T: Into<String>,
    {
        self.title = value.into().into();
        self
    }

    fn add_item(
        mut self,
        item: ListItem<'a>,
        handler: fn(&mut TerminalMenu) -> Option<String>,
    ) -> MenuBuilder {
        let num = self.next_id();
        self.add_numbered_item(num, item, handler)
    }

    fn add_input_item(
        mut self,
        item: ListItem<'a>,
        prompt_text: &str,
        input_handler: fn(String) -> Option<String>,
    ) -> MenuBuilder<'a> {
        let num = self.next_id();
        self.add_numbered_input_item(num, item, prompt_text, input_handler)
    }

    fn next_id(&mut self) -> usize {
        loop {
            self.head_id += 1;
            if !self.num_ids.contains_key(&self.head_id) {
                break;
            }
        }
        self.head_id
    }

    fn no_id(mut self) -> MenuBuilder<'a> {
        self.num_ids.remove_entry(&self.head_id);
        self.head_id -= 1;
        self
    }

    fn add_numbered_item(
        mut self,
        num: usize,
        item: ListItem<'a>,
        handler: fn(&mut TerminalMenu) -> Option<String>,
    ) -> MenuBuilder {
        self.list_items.push(item);
        self.handlers
            .insert(self.list_items.len() - 1, Box::new(handler));
        self.num_ids.insert(num, self.list_items.len() - 1);
        self
    }

    fn add_numbered_input_item(
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

    fn add_text_item<T>(mut self, label: T) -> MenuBuilder<'a>
    where
        T: Into<String>,
    {
        self.list_items.push(item_text(label));
        self
    }

    fn add_dummy_item(mut self) -> MenuBuilder<'a> {
        self.list_items.push(item_dummy());
        self
    }
}

macro_rules! add_toggle_item {
    ( $builder:ident, $i18n_bundle:expr, $label:expr, $value:expr, $x:ident ) => {{
        MenuBuilder::add_item(
            $builder,
            item_enabled($i18n_bundle, $label, $value),
            |_handle: &mut TerminalMenu| {
                let settings = &mut lock_config!().settings;
                settings.$x = !settings.$x;
                None
            },
        )
    }};
}

enum LootLevel {
    White,
    Blue,
    Purple,
    Gold,
    Red,
}

macro_rules! add_pick_item {
    ( $builder:ident, $i18n_bundle:expr, $label_prefix:expr, $label_id:ident, $value:expr, $x:ident ) => {{
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
            |_handle: &mut TerminalMenu| {
                let settings = &mut lock_config!().settings;
                settings.loot.$x = !settings.loot.$x;
                None
            },
        )
    }};
}

macro_rules! add_colored_loot_item {
    ( $builder:ident, $i18n_bundle:expr, $label_prefix:expr, $label_id:ident, $loot_level:expr, $value:expr, $x:ident ) => {{
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
            |_handle: &mut TerminalMenu| {
                let settings = &mut lock_config!().settings;
                settings.loot.$x = !settings.loot.$x;
                None
            },
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

fn build_main_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    settings: config::Settings,
) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title(i18n_msg!(i18n_bundle, MainMenuTitle));
    menu = add_toggle_item!(
        menu,
        &i18n_bundle,
        format!(" 1 - {}", i18n_msg!(i18n_bundle, MenuItemFiringRange)),
        settings.firing_range,
        firing_range
    );
    menu = add_toggle_item!(
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
                settings.keyboard,
            ),
            |_| {
                let settings = &mut lock_config!().settings;
                settings.keyboard = !settings.keyboard;
                settings.gamepad = !settings.keyboard;
                None
            },
        )
        .add_item(
            item_enabled(
                &i18n_bundle,
                format!(" 4 - {}", i18n_msg!(i18n_bundle, MenuItemGamepad)),
                settings.gamepad,
            ),
            |_| {
                let settings = &mut lock_config!().settings;
                settings.gamepad = !settings.gamepad;
                settings.keyboard = !settings.gamepad;
                None
            },
        );
    menu = add_toggle_item!(
        menu,
        &i18n_bundle,
        format!(" 5 - {}", i18n_msg!(i18n_bundle, MenuItemItemGlow)),
        settings.item_glow,
        item_glow
    );
    menu = add_toggle_item!(
        menu,
        &i18n_bundle,
        format!(" 6 - {}", i18n_msg!(i18n_bundle, MenuItemPlayerGlow)),
        settings.player_glow,
        player_glow
    );
    menu = menu
        .add_input_item(
            format_item(
                &i18n_bundle,
                format!(" 7 - {}", i18n_msg!(i18n_bundle, MenuItemSmoothValue)),
                if settings.smooth < 90.0 {
                    Span::styled(
                        format!("{}", settings.smooth),
                        Style::default().fg(Color::Red),
                    )
                } else if settings.smooth > 120.0 {
                    Span::styled(
                        format!("{}", settings.smooth),
                        Style::default().fg(Color::Green),
                    )
                } else {
                    Span::from(format!("{}", settings.smooth))
                },
            ),
            &i18n_msg!(i18n_bundle, InputPromptSmoothValue),
            |val| {
                if let Some(new_val) = val.parse::<u16>().ok() {
                    if new_val >= 50 && new_val <= 500 {
                        let settings = &mut lock_config!().settings;
                        settings.smooth = new_val.into();
                        settings.skynade_smooth = settings.smooth * 0.6667;
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
                format!(" 8 - {}", i18n_msg!(i18n_bundle, MenuItemChangeBoneAim)),
                Span::from(
                    if settings.bone_nearest {
                        i18n_msg!(i18n_bundle, MenuValueBoneNearest)
                    } else if settings.bone_auto {
                        i18n_msg!(i18n_bundle, MenuValueBoneAuto)
                    } else {
                        match settings.bone {
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
                    settings.bone_auto = true;
                    settings.bone_nearest = false;
                    return None;
                } else if val == "h" {
                    let settings = &mut lock_config!().settings;
                    settings.bone_nearest = true;
                    settings.bone_auto = false;
                    return None;
                } else if let Some(new_val) = val.parse::<u8>().ok() {
                    if vec![0, 1, 2, 3].contains(&new_val) {
                        let settings = &mut lock_config!().settings;
                        settings.bone = new_val.into();
                        settings.bone_auto = false;
                        return None;
                    }
                    return Some(i18n_msg!(i18n_bundle, InfoInvalidBoneValue).to_string());
                }
                Some(i18n_msg!(i18n_bundle, InfoInvalidValue).to_string())
            },
        )
        .add_item(
            item_enabled(
                &i18n_bundle,
                format!(" 9 - {}", i18n_msg!(i18n_bundle, MenuItemLootGlowFilled)),
                settings.loot_filled_toggle,
            ),
            |_| {
                let settings = &mut lock_config!().settings;
                settings.loot_filled_toggle = !settings.loot_filled_toggle;
                settings.loot_filled = if settings.loot_filled_toggle { 14 } else { 0 };
                None
            },
        )
        .add_item(
            item_enabled(
                &i18n_bundle,
                format!("10 - {}", i18n_msg!(i18n_bundle, MenuItemPlayerGlowFilled)),
                settings.player_filled_toggle,
            ),
            |_| {
                let settings = &mut lock_config!().settings;
                settings.player_filled_toggle = !settings.player_filled_toggle;
                settings.player_glow_inside_value =
                    if settings.player_filled_toggle { 14 } else { 0 };
                None
            },
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
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::GlowColorMenu);
                None
            },
        )
        .add_input_item(
            format_item(
                &i18n_bundle,
                format!("13 - {}", i18n_msg!(i18n_bundle, MenuItemChangeAdsFov)),
                Span::from(format!("{}", settings.ads_fov)),
            ),
            &i18n_msg!(i18n_bundle, InputPromptAdsFov),
            |val| {
                if let Some(new_val) = val.parse::<f32>().ok() {
                    if new_val >= 1.0 && new_val <= 50.0 {
                        let settings = &mut lock_config!().settings;
                        settings.ads_fov = new_val;
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
                format!("14 - {}", i18n_msg!(i18n_bundle, MenuItemChangeNonAdsFov)),
                Span::from(format!("{}", settings.non_ads_fov)),
            ),
            &i18n_msg!(i18n_bundle, InputPromptNonAdsFov),
            |val| {
                if let Some(new_val) = val.parse::<f32>().ok() {
                    if new_val >= 1.0 && new_val <= 50.0 {
                        let settings = &mut lock_config!().settings;
                        settings.non_ads_fov = new_val;
                        return None;
                    }
                }
                let i18n_bundle = get_fluent_bundle();
                Some(i18n_msg!(i18n_bundle, InfoInvalidNonAdsFov).to_string())
            },
        );
    menu = add_toggle_item!(
        menu,
        &i18n_bundle,
        format!("15 - {}", i18n_msg!(i18n_bundle, MenuItemSuperGlide)),
        settings.super_key_toggle,
        super_key_toggle
    );
    menu = menu
        .add_item(
            item_text(format!(
                "16 - {}",
                i18n_msg!(i18n_bundle, MenuItemItemFilterSettings)
            )),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::ItemFilterMenu);
                None
            },
        )
        .add_item(
            item_text(format!(
                "17 - {}",
                i18n_msg!(i18n_bundle, MenuItemHotkeySettings)
            )),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::HotkeyMenu);
                None
            },
        )
        .add_item(
            if settings.load_settings {
                item_dummy()
            } else {
                item_text("18.5 -‌​‌‌​​​‌‌‌‍‌​‌‌​‌​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌​‌‌‌‌‌‍‌​‌‌‌‌​​‌‌‍‌​‌‌​​​​‌‌‍‌​‌‌‌​​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌‌​​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​‌‌‍‌​‌‌​‌​​‌‌‍‌​‌‌​‌‌​‌‌‍‌​‌‌​​‌​‌‌‍‌​‌‌​‌‌‌​‌‍‌​‌‌‌​‌​‌‌ ")
            },
            |_| {
                let config = &mut lock_config!();
                config.settings.load_settings = !config.settings.load_settings;
                if config.settings.load_settings {
                    None
                } else {
                    let i18n_bundle = get_fluent_bundle();
                    Some(i18n_msg!(i18n_bundle, HelloWorld).to_string())
                }
            },
        );
    menu.next_id();
    menu = add_toggle_item!(
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
            |_| {
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
        )
        .add_item(
            item_text(format!(
                "22 - {}",
                i18n_msg!(i18n_bundle, MenuItemLoadSettings)
            )),
            |_| {
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
        )
        .add_dummy_item()
        .add_item(
            format_item(
                &i18n_bundle,
                format!("23 - {}", i18n_msg!(i18n_bundle, MenuItemToggleNadeAim)),
                Span::from(
                    if settings.no_nade_aim {
                        i18n_msg!(i18n_bundle, MenuValueNoNadeAim)
                    } else {
                        i18n_msg!(i18n_bundle, MenuValueNadeAimOn)
                    }
                    .to_string(),
                ),
            ),
            |_| {
                let settings = &mut lock_config!().settings;
                settings.no_nade_aim = !settings.no_nade_aim;
                None
            },
        );
    menu = add_toggle_item!(
        menu,
        &i18n_bundle,
        format!("24 - {}", i18n_msg!(i18n_bundle, MenuItemToggleOnevone)),
        settings.onevone,
        onevone
    );
    menu = add_toggle_item!(
        menu,
        &i18n_bundle,
        format!("25 - {}", i18n_msg!(i18n_bundle, MenuItemToggleNoRecoil)),
        settings.aim_no_recoil,
        aim_no_recoil
    );
    menu = menu.add_input_item(
        format_item(
            &i18n_bundle,
            format!("26 - {}", i18n_msg!(i18n_bundle, MenuItemSetFpsPredict)),
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
    );
    menu = add_toggle_item!(
        menu,
        &i18n_bundle,
        format!("27 - {}", i18n_msg!(i18n_bundle, MenuItemBigMapFeat)),
        settings.map_radar_testing,
        map_radar_testing
    );
    menu = add_toggle_item!(
        menu,
        &i18n_bundle,
        format!(
            "28 - {}",
            i18n_msg!(i18n_bundle, MenuItemPlayerArmorGlowColor)
        ),
        settings.player_glow_armor_color,
        player_glow_armor_color
    );
    menu = add_toggle_item!(
        menu,
        &i18n_bundle,
        format!(
            "29 - {}",
            i18n_msg!(i18n_bundle, MenuItemFavoritePlayerGlow)
        ),
        settings.player_glow_love_user,
        player_glow_love_user
    );
    menu = menu.add_item(
        item_enabled(
            &i18n_bundle,
            format!("30 - {}", i18n_msg!(i18n_bundle, MenuItemWeaponModelGlow)),
            settings.weapon_model_glow,
        ),
        |_handle: &mut TerminalMenu| {
            let settings = &mut lock_config!().settings;
            settings.weapon_model_glow = !settings.weapon_model_glow;
            if settings.weapon_model_glow {
                let i18n_bundle = get_fluent_bundle();
                Some(i18n_msg!(i18n_bundle, InfoWeaponModelGlow).to_string())
            } else {
                None
            }
        },
    );
    menu = menu.add_item(
        item_enabled(
            &i18n_bundle,
            format!("31 - {}", i18n_msg!(i18n_bundle, MenuItemKbdBacklightCtrl)),
            settings.kbd_backlight_control,
        ),
        |_handle: &mut TerminalMenu| {
            let settings = &mut lock_config!().settings;
            settings.kbd_backlight_control = !settings.kbd_backlight_control;
            if settings.kbd_backlight_control {
                if let Err(e) = G_CONTEXT.lock().unwrap().kbd_backlight_test() {
                    return Some(e.to_string());
                }
            }
            None
        },
    );
    menu.add_dummy_item()
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
            |_| {
                let settings = &mut lock_config!().settings;
                settings.no_overlay = !settings.no_overlay;
                None
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
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
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
            |handler: &mut TerminalMenu| {
                handler.nav_menu(MenuLevel::KeyCodesMenu);
                None
            },
        )
        .add_dummy_item()
        .add_item(
            item_text(format!(
                "5 - {}",
                i18n_msg!(i18n_bundle, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_item_filter_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    _settings: config::Settings,
) -> MenuState<'static> {
    MenuBuilder::new()
        .title(i18n_msg!(i18n_bundle, ItemFilterMenuTitle))
        .add_item(
            item_text(format!("1 - {}", i18n_msg!(i18n_bundle, ItemLightWeapons))),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::LightWeaponsMenu);
                None
            },
        )
        .add_item(
            item_text(format!("2 - {}", i18n_msg!(i18n_bundle, ItemHeavyWeapons))),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::HeavyWeaponsMenu);
                None
            },
        )
        .add_item(
            item_text(format!("3 - {}", i18n_msg!(i18n_bundle, ItemEnergyWeapons))),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::EnergyWeaponsMenu);
                None
            },
        )
        .add_item(
            item_text(format!("4 - {}", i18n_msg!(i18n_bundle, ItemSniperWeapons))),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::SniperWeaponsMenu);
                None
            },
        )
        .add_item(
            item_text(format!("5 - {}", i18n_msg!(i18n_bundle, ItemArmors))),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::ArmorsMenu);
                None
            },
        )
        .add_item(
            item_text(format!("6 - {}", i18n_msg!(i18n_bundle, ItemHealing))),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::HealingMenu);
                None
            },
        )
        .add_item(
            item_text(format!("7 - {}", i18n_msg!(i18n_bundle, ItemNades))),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::NadesMenu);
                None
            },
        )
        .add_item(
            item_text(format!("8 - {}", i18n_msg!(i18n_bundle, ItemBackpacks))),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::BackpacksMenu);
                None
            },
        )
        .add_item(
            item_text(format!("9 - {}", i18n_msg!(i18n_bundle, ItemScopes))),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::ScopesMenu);
                None
            },
        )
        .add_item(
            item_text(format!(
                "10 - {}",
                i18n_msg!(i18n_bundle, MenuItemBackToMainMenu)
            )),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_light_weapons_menu(
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
            |_| None,
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, LightWeaponsSection))
        .add_dummy_item();
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "1 - ",
        WeaponP2020,
        settings.loot.weapon_p2020,
        weapon_p2020
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "2 - ",
        WeaponRe45,
        settings.loot.weapon_re45,
        weapon_re45
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "3 - ",
        WeaponAlternator,
        settings.loot.weapon_alternator,
        weapon_alternator
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "4 - ",
        WeaponR99,
        settings.loot.weapon_r99,
        weapon_r99
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "5 - ",
        WeaponR301,
        settings.loot.weapon_r301,
        weapon_r301
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "6 - ",
        WeaponM600,
        settings.loot.weapon_spitfire,
        weapon_spitfire
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "7 - ",
        WeaponG7Scout,
        settings.loot.weapon_g7_scout,
        weapon_g7_scout
    );
    menu = add_pick_item!(
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
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "9 - ",
        LootLightWeaponMag,
        LootLevel::White,
        settings.loot.lightammomag1,
        lightammomag1
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "10 - ",
        LootLightWeaponMag,
        LootLevel::Blue,
        settings.loot.lightammomag2,
        lightammomag2
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "11 - ",
        LootLightWeaponMag,
        LootLevel::Purple,
        settings.loot.lightammomag3,
        lightammomag3
    );
    menu = add_colored_loot_item!(
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
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "13 - ",
        LootStandardStock,
        LootLevel::White,
        settings.loot.stockregular1,
        stockregular1
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "14 - ",
        LootStandardStock,
        LootLevel::Blue,
        settings.loot.stockregular2,
        stockregular2
    );
    menu = add_colored_loot_item!(
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
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "16 - ",
        LootWeaponSuppressors,
        LootLevel::White,
        settings.loot.suppressor1,
        suppressor1
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "17 - ",
        LootWeaponSuppressors,
        LootLevel::Blue,
        settings.loot.suppressor2,
        suppressor2
    );
    menu = add_colored_loot_item!(
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
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "19 - ",
        LootWeaponLasers,
        LootLevel::White,
        settings.loot.lasersight1,
        lasersight1
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "20 - ",
        LootWeaponLasers,
        LootLevel::Blue,
        settings.loot.lasersight2,
        lasersight2
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "21 - ",
        LootWeaponLasers,
        LootLevel::Purple,
        settings.loot.lasersight3,
        lasersight3
    );
    menu = add_colored_loot_item!(
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
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "23 - ",
        LootTurboCharger,
        settings.loot.turbo_charger,
        turbo_charger
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "24 - ",
        LootSkullPiecer,
        settings.loot.skull_piecer,
        skull_piecer
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "25 - ",
        LootHammerPoints,
        settings.loot.hammer_point,
        hammer_point
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "26 - ",
        LootDisruptorRounds,
        settings.loot.disruptor_rounds,
        disruptor_rounds
    );
    menu = add_pick_item!(
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
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_heavy_weapons_menu(
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
            |_| None,
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, HeavyWeaponsSection))
        .add_dummy_item();
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "1 - ",
        WeaponFlatline,
        settings.loot.weapon_flatline,
        weapon_flatline
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "2 - ",
        WeaponHemlock,
        settings.loot.weapon_hemlock,
        weapon_hemlock
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "3 - ",
        Weapon3030Repeater,
        settings.loot.weapon_3030_repeater,
        weapon_3030_repeater
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "4 - ",
        WeaponRampage,
        settings.loot.weapon_rampage,
        weapon_rampage
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "5 - ",
        WeaponProwler,
        settings.loot.weapon_prowler,
        weapon_prowler
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "6 - ",
        WeaponCarSmg,
        settings.loot.weapon_car_smg,
        weapon_car_smg
    );
    menu = add_pick_item!(
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
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "8 - ",
        LootHeavyWeaponMag,
        LootLevel::White,
        settings.loot.heavyammomag1,
        heavyammomag1
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "9 - ",
        LootHeavyWeaponMag,
        LootLevel::Blue,
        settings.loot.heavyammomag2,
        heavyammomag2
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "10 - ",
        LootHeavyWeaponMag,
        LootLevel::Purple,
        settings.loot.heavyammomag3,
        heavyammomag3
    );
    menu = add_colored_loot_item!(
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
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "12 - ",
        LootStandardStock,
        LootLevel::White,
        settings.loot.stockregular1,
        stockregular1
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "13 - ",
        LootStandardStock,
        LootLevel::Blue,
        settings.loot.stockregular2,
        stockregular2
    );
    menu = add_colored_loot_item!(
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
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "15 - ",
        LootWeaponSuppressors,
        LootLevel::White,
        settings.loot.suppressor1,
        suppressor1
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "16 - ",
        LootWeaponSuppressors,
        LootLevel::Blue,
        settings.loot.suppressor2,
        suppressor2
    );
    menu = add_colored_loot_item!(
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
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "18 - ",
        LootWeaponLasers,
        LootLevel::White,
        settings.loot.lasersight1,
        lasersight1
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "19 - ",
        LootWeaponLasers,
        LootLevel::Blue,
        settings.loot.lasersight2,
        lasersight2
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "20 - ",
        LootWeaponLasers,
        LootLevel::Purple,
        settings.loot.lasersight3,
        lasersight3
    );
    menu = add_colored_loot_item!(
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
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "22 - ",
        LootTurboCharger,
        settings.loot.turbo_charger,
        turbo_charger
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "23 - ",
        LootSkullPiecer,
        settings.loot.skull_piecer,
        skull_piecer
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "24 - ",
        LootHammerPoints,
        settings.loot.hammer_point,
        hammer_point
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "25 - ",
        LootDisruptorRounds,
        settings.loot.disruptor_rounds,
        disruptor_rounds
    );
    menu = add_pick_item!(
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
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_energy_weapons_menu(
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
            |_| None,
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, EnergyWeaponsSection))
        .add_dummy_item();
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "1 - ",
        WeaponLStar,
        settings.loot.weapon_lstar,
        weapon_lstar
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "2 - ",
        WeaponNemesis,
        settings.loot.weapon_nemesis,
        weapon_nemesis
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "3 - ",
        WeaponHavoc,
        settings.loot.weapon_havoc,
        weapon_havoc
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "4 - ",
        WeaponDeovtion,
        settings.loot.weapon_devotion,
        weapon_devotion
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "5 - ",
        WeaponTripleTake,
        settings.loot.weapon_triple_take,
        weapon_triple_take
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "6 - ",
        WeaponVolt,
        settings.loot.weapon_volt,
        weapon_volt
    );
    menu = add_pick_item!(
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
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "8 - ",
        LootEnergyWeaponMag,
        LootLevel::White,
        settings.loot.energyammomag1,
        energyammomag1
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "9 - ",
        LootEnergyWeaponMag,
        LootLevel::Blue,
        settings.loot.energyammomag2,
        energyammomag2
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "10 - ",
        LootEnergyWeaponMag,
        LootLevel::Purple,
        settings.loot.energyammomag3,
        energyammomag3
    );
    menu = add_colored_loot_item!(
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
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "12 - ",
        LootStandardStock,
        LootLevel::White,
        settings.loot.stockregular1,
        stockregular1
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "13 - ",
        LootStandardStock,
        LootLevel::Blue,
        settings.loot.stockregular2,
        stockregular2
    );
    menu = add_colored_loot_item!(
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
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "15 - ",
        LootWeaponSuppressors,
        LootLevel::White,
        settings.loot.suppressor1,
        suppressor1
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "16 - ",
        LootWeaponSuppressors,
        LootLevel::Blue,
        settings.loot.suppressor2,
        suppressor2
    );
    menu = add_colored_loot_item!(
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
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "18 - ",
        LootWeaponLasers,
        LootLevel::White,
        settings.loot.lasersight1,
        lasersight1
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "19 - ",
        LootWeaponLasers,
        LootLevel::Blue,
        settings.loot.lasersight2,
        lasersight2
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "20 - ",
        LootWeaponLasers,
        LootLevel::Purple,
        settings.loot.lasersight3,
        lasersight3
    );
    menu = add_colored_loot_item!(
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
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "22 - ",
        LootTurboCharger,
        settings.loot.turbo_charger,
        turbo_charger
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "23 - ",
        LootSkullPiecer,
        settings.loot.skull_piecer,
        skull_piecer
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "24 - ",
        LootHammerPoints,
        settings.loot.hammer_point,
        hammer_point
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "25 - ",
        LootDisruptorRounds,
        settings.loot.disruptor_rounds,
        disruptor_rounds
    );
    menu = add_pick_item!(
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
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_sniper_weapons_menu(
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
            |_| None,
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, SniperWeaponsSection))
        .add_dummy_item();
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "1 - ",
        WeaponWingman,
        settings.loot.weapon_wingman,
        weapon_wingman
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "2 - ",
        WeaponLongbow,
        settings.loot.weapon_longbow,
        weapon_longbow
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "3 - ",
        WeaponChargeRifle,
        settings.loot.weapon_charge_rifle,
        weapon_charge_rifle
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "4 - ",
        WeaponSentinel,
        settings.loot.weapon_sentinel,
        weapon_sentinel
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "5 - ",
        WeaponBow,
        settings.loot.weapon_bow,
        weapon_bow
    );
    menu = add_pick_item!(
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
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "7 - ",
        LootSniperWeaponMag,
        LootLevel::White,
        settings.loot.sniperammomag1,
        sniperammomag1
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "8 - ",
        LootSniperWeaponMag,
        LootLevel::Blue,
        settings.loot.sniperammomag2,
        sniperammomag2
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "9 - ",
        LootSniperWeaponMag,
        LootLevel::Purple,
        settings.loot.sniperammomag3,
        sniperammomag3
    );
    menu = add_colored_loot_item!(
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
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "11 - ",
        LootSniperStock,
        LootLevel::White,
        settings.loot.stocksniper1,
        stocksniper1
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "12 - ",
        LootSniperStock,
        LootLevel::Blue,
        settings.loot.stocksniper2,
        stocksniper2
    );
    menu = add_colored_loot_item!(
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
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "14 - ",
        LootWeaponSuppressors,
        LootLevel::White,
        settings.loot.suppressor1,
        suppressor1
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "15 - ",
        LootWeaponSuppressors,
        LootLevel::Blue,
        settings.loot.suppressor2,
        suppressor2
    );
    menu = add_colored_loot_item!(
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
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "17 - ",
        LootTurboCharger,
        settings.loot.turbo_charger,
        turbo_charger
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "18 - ",
        LootSkullPiecer,
        settings.loot.skull_piecer,
        skull_piecer
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "19 - ",
        LootHammerPoints,
        settings.loot.hammer_point,
        hammer_point
    );
    menu = add_pick_item!(
        menu,
        i18n_bundle,
        "20 - ",
        LootDisruptorRounds,
        settings.loot.disruptor_rounds,
        disruptor_rounds
    );
    menu = add_pick_item!(
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
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_armors_menu(
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
            |_| None,
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, ArmorsSection))
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "1 - ",
        LootEvoShield,
        LootLevel::White,
        settings.loot.shieldupgrade1,
        shieldupgrade1
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "2 - ",
        LootEvoShield,
        LootLevel::Blue,
        settings.loot.shieldupgrade2,
        shieldupgrade2
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "3 - ",
        LootEvoShield,
        LootLevel::Purple,
        settings.loot.shieldupgrade3,
        shieldupgrade3
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "4 - ",
        LootBodyShield,
        LootLevel::Gold,
        settings.loot.shieldupgrade4,
        shieldupgrade4
    );
    menu = add_colored_loot_item!(
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
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "6 - ",
        LootHelmet,
        LootLevel::White,
        settings.loot.shieldupgradehead1,
        shieldupgradehead1
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "7 - ",
        LootHelmet,
        LootLevel::Blue,
        settings.loot.shieldupgradehead2,
        shieldupgradehead2
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "8 - ",
        LootHelmet,
        LootLevel::Purple,
        settings.loot.shieldupgradehead3,
        shieldupgradehead3
    );
    menu = add_colored_loot_item!(
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
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "10 - ",
        LootKnockdownShield,
        LootLevel::White,
        settings.loot.shielddown1,
        shielddown1
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "11 - ",
        LootKnockdownShield,
        LootLevel::Blue,
        settings.loot.shielddown2,
        shielddown2
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "12 - ",
        LootKnockdownShield,
        LootLevel::Purple,
        settings.loot.shielddown3,
        shielddown3
    );
    menu = add_colored_loot_item!(
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
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_healing_menu(
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
            |_| None,
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, HealingItemsSection))
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "1 - ",
        LootAccelerant,
        LootLevel::Blue,
        settings.loot.accelerant,
        accelerant
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "2 - ",
        LootPhoenix,
        LootLevel::Purple,
        settings.loot.phoenix,
        phoenix
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "3 - ",
        LootSmallHealth,
        LootLevel::White,
        settings.loot.healthsmall,
        healthsmall
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "4 - ",
        LootLargeHealth,
        LootLevel::White,
        settings.loot.healthlarge,
        healthlarge
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "5 - ",
        LootSmallShieldBatt,
        LootLevel::White,
        settings.loot.shieldbattsmall,
        shieldbattsmall
    );
    menu = add_colored_loot_item!(
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
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_nades_menu(
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
            |_| None,
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, NadeItemsSection))
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "1 - ",
        LootFragGrenade,
        LootLevel::Red,
        settings.loot.grenade_frag,
        grenade_frag
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "2 - ",
        LootArcStar,
        LootLevel::Blue,
        settings.loot.grenade_arc_star,
        grenade_arc_star
    );
    menu = add_colored_loot_item!(
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
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_backpacks_menu(
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
            |_| None,
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, BackpacksSection))
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "1 - ",
        LootLightBackpack,
        LootLevel::White,
        settings.loot.lightbackpack,
        lightbackpack
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "2 - ",
        LootMediumBackpack,
        LootLevel::Blue,
        settings.loot.medbackpack,
        medbackpack
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "3 - ",
        LootHeavyBackpack,
        LootLevel::Purple,
        settings.loot.heavybackpack,
        heavybackpack
    );
    menu = add_colored_loot_item!(
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
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_scopes_menu(
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
            |_| None,
        )
        .no_id()
        .add_dummy_item()
        .add_text_item(i18n_msg!(i18n_bundle, ScopesSection))
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "1 - ",
        Loot1xHcog,
        LootLevel::White,
        settings.loot.optic1xhcog,
        optic1xhcog
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "2 - ",
        Loot2xHcog,
        LootLevel::Blue,
        settings.loot.optic2xhcog,
        optic2xhcog
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "3 - ",
        Loot1xHolo,
        LootLevel::White,
        settings.loot.opticholo1x,
        opticholo1x
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "4 - ",
        Loot1x2xHolo,
        LootLevel::Blue,
        settings.loot.opticholo1x2x,
        opticholo1x2x
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "5 - ",
        LootOpticThreat,
        LootLevel::Gold,
        settings.loot.opticthreat,
        opticthreat
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "6 - ",
        Loot3xHcog,
        LootLevel::Purple,
        settings.loot.optic3xhcog,
        optic3xhcog
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "7 - ",
        Loot2x4xAog,
        LootLevel::Purple,
        settings.loot.optic2x4x,
        optic2x4x
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "8 - ",
        Loot6xSniperOptic,
        LootLevel::Blue,
        settings.loot.opticsniper6x,
        opticsniper6x
    );
    menu = add_colored_loot_item!(
        menu,
        i18n_bundle,
        "9 - ",
        Loot4x8xSniperOptic,
        LootLevel::Purple,
        settings.loot.opticsniper4x8x,
        opticsniper4x8x
    );
    menu = add_colored_loot_item!(
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
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
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
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::HotkeyMenu);
                None
            },
        )
        .add_dummy_item()
        .add_item(
            item_text(i18n_msg!(i18n_bundle, MenuItemBackToMainMenu)),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
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
                    if now.month() == 12 && now.day() == 25 {
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
fn format_item<'a, T>(
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
fn item_enabled<T>(
    i18n_bundle: &FluentBundle<FluentResource>,
    label: T,
    v: bool,
) -> ListItem<'static>
where
    T: Into<String>,
{
    format_item(i18n_bundle, label, span_enabled(i18n_bundle, v))
}
fn item_text<T>(label: T) -> ListItem<'static>
where
    T: Into<String>,
{
    ListItem::new(Line::from(format_label(label)))
}
fn item_dummy() -> ListItem<'static> {
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_menu() {
        super::super::main().unwrap();
    }
}
