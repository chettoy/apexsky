use super::{alert, prompt};
use crate::{config, global_state::G_STATE};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::{collections::HashMap, fmt::Debug};

pub struct TerminalMenu<'a> {
    app_model: super::Model,
    menu_level: Vec<MenuLevel>,
    menu_state: Option<MenuState<'a>>,
    scroll_height: usize,
}

#[derive(Clone)]
struct MenuState<'a> {
    title: &'a str,
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
            .field("nav_index", &self.nav_index)
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

    pub fn app_model(&self) -> &super::Model {
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

        let data = G_STATE.lock().unwrap().settings.to_owned();
        let mut new_menu_state = match self.get_menu_level() {
            MenuLevel::MainMenu => build_main_menu(&data),
            MenuLevel::GlowColorMenu => build_glow_color_menu(&data),
            MenuLevel::ItemFilterMenu => build_item_filter_menu(&data),
            MenuLevel::LightWeaponsMenu => build_light_weapons_menu(&data),
            MenuLevel::HeavyWeaponsMenu => build_heavy_weapons_menu(&data),
            MenuLevel::EnergyWeaponsMenu => build_energy_weapons_menu(&data),
            MenuLevel::SniperWeaponsMenu => build_sniper_weapons_menu(&data),
            MenuLevel::ArmorsMenu => build_armors_menu(&data),
            MenuLevel::HealingMenu => build_healing_menu(&data),
            MenuLevel::NadesMenu => build_nades_menu(&data),
            MenuLevel::BackpacksMenu => build_backpacks_menu(&data),
            MenuLevel::ScopesMenu => build_scopes_menu(&data),
            MenuLevel::KeyCodesMenu => build_key_codes_menu(&data),
            MenuLevel::HotkeyMenu => build_hotkey_menu(&data),
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

        f.render_widget(block_title(state.title), chunks[0]);
        f.render_widget(
            render_selected_list(&state.items, state.nav_index, state.scroll_top),
            chunks[1],
        );
    }
}

#[derive(Debug)]
pub struct MenuBuilder<'a> {
    title: &'a str,
    list_items: Vec<ListItem<'a>>,
    handlers: HashMap<usize, Box<fn(&mut TerminalMenu<'_>) -> Option<String>>>,
    input_handlers: HashMap<usize, (String, Box<fn(String) -> Option<String>>)>,
    num_ids: HashMap<usize, usize>,
    head_id: usize,
}

impl<'a> MenuBuilder<'a> {
    fn new() -> MenuBuilder<'a> {
        MenuBuilder {
            title: "",
            list_items: Vec::new(),
            handlers: HashMap::new(),
            input_handlers: HashMap::new(),
            num_ids: HashMap::new(),
            head_id: 0,
        }
    }

    fn title(mut self, value: &'a str) -> MenuBuilder {
        self.title = value;
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

    fn add_text_item(mut self, label: &'a str) -> MenuBuilder {
        self.list_items.push(item_text(label));
        self
    }

    fn add_dummy_item(mut self) -> MenuBuilder<'a> {
        self.list_items.push(item_dummy());
        self
    }
}

macro_rules! add_toggle_item {
    ( $builder:ident, $label:expr, $value:expr, $x:ident ) => {{
        MenuBuilder::add_item(
            $builder,
            item_enabled($label, $value),
            |_handle: &mut TerminalMenu| {
                let settings = &mut G_STATE.lock().unwrap().settings;
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
    ( $builder:ident, $label:expr, $value:expr, $x:ident ) => {{
        let (pick_color, pick_mark) = if $value {
            (Color::Green, "[x]")
        } else {
            (Color::Red, "[ ]")
        };
        MenuBuilder::add_item(
            $builder,
            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", $label), Style::new().fg(pick_color)),
                Span::styled(pick_mark, Style::new().white()),
            ])),
            |_handle: &mut TerminalMenu| {
                let settings = &mut G_STATE.lock().unwrap().settings;
                settings.loot.$x = !settings.loot.$x;
                None
            },
        )
    }};
}

macro_rules! add_colored_loot_item {
    ( $builder:ident, $label:expr, $loot_level:expr, $value:expr, $x:ident ) => {{
        let (color_label, color) = match $loot_level {
            LootLevel::White => ("White", Color::White),
            LootLevel::Blue => ("Blue", Color::Blue),
            LootLevel::Purple => ("Purple", Color::Magenta),
            LootLevel::Gold => ("Gold", Color::Yellow),
            LootLevel::Red => ("Red", Color::Red),
        };
        let (pick_color, pick_mark) = if $value {
            (Color::Green, "[x]")
        } else {
            (Color::Red, "[‌​‌‌​​​‌‌‌‍‌​‌‌​‌​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌​‌‌‌‌‌‍‌​‌‌‌‌​​‌‌‍‌​‌‌​​​​‌‌‍‌​‌‌‌​​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌‌​​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​‌‌‍‌​‌‌​‌​​‌‌‍‌​‌‌​‌‌​‌‌‍‌​‌‌​​‌​‌‌‍‌​‌‌​‌‌‌​‌‍‌​‌‌‌​‌​‌‌ ]")
        };
        MenuBuilder::add_item(
            $builder,
            ListItem::new(Line::from(vec![
                Span::styled(format!("{}: ", $label), Style::new().fg(pick_color)),
                Span::styled(format!("{} ", color_label), Style::new().fg(color)),
                Span::styled(pick_mark, Style::new().white()),
            ])),
            |_handle: &mut TerminalMenu| {
                let settings = &mut G_STATE.lock().unwrap().settings;
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

fn build_main_menu(settings: &config::Config) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title("Main Menu");
    menu = add_toggle_item!(
        menu,
        " 1 - Firing Range",
        settings.firing_range,
        firing_range
    );
    menu = add_toggle_item!(menu, " 2 - TDMToggle", settings.tdm_toggle, tdm_toggle);
    menu = menu
        .add_item(item_enabled(" 3 - Keyboard", settings.keyboard), |_| {
            let settings = &mut G_STATE.lock().unwrap().settings;
            settings.keyboard = !settings.keyboard;
            settings.gamepad = !settings.keyboard;
            None
        })
        .add_item(item_enabled(" 4 - Gamepad", settings.gamepad), |_| {
            let settings = &mut G_STATE.lock().unwrap().settings;
            settings.gamepad = !settings.gamepad;
            settings.keyboard = !settings.gamepad;
            None
        });
    menu = add_toggle_item!(menu, " 5 - Item Glow", settings.item_glow, item_glow);
    menu = add_toggle_item!(menu, " 6 - Player Glow", settings.player_glow, player_glow);
    menu = menu
        .add_input_item(
            format_item(
                " 7 - Smooth Value",
                Span::styled(
                    format!("{}", settings.smooth),
                    Style::default().fg(if settings.smooth < 90.0 {
                        Color::Red
                    } else if settings.smooth > 120.0 {
                        Color::Green
                    } else {
                        Color::White
                    }),
                ),
            ),
            "New value for 'smooth' (50 to 500): ",
            |val| {
                if let Some(new_val) = val.parse::<u16>().ok() {
                    if new_val >= 50 && new_val <= 500 {
                        let settings = &mut G_STATE.lock().unwrap().settings;
                        settings.smooth = new_val.into();
                        settings.skynade_smooth = settings.smooth * 0.6667;
                        return None;
                    }
                }
                Some("Invalid value. 'smooth' value must be between 70 and 500.".to_string())
            },
        )
        .add_input_item(
            format_item(
                " 8 - Change Bone Aim Value",
                Span::styled(
                    if settings.bone_auto {
                        "Auto"
                    } else if settings.bone_nearest {
                        "Nearest"
                    } else {
                        match settings.bone {
                            0 => "Head",
                            1 => "Neck",
                            2 => "Chest",
                            3 => "Gut Shut",
                            _ => "Unknown",
                        }
                    },
                    Style::new().white(),
                ),
            ),
            "New value for 'bone': 
            x => Auto
            0 => Head
            1 => Neck
            2 => Chest
            3 => Gut Shut
            ",
            |val| {
                if val.trim() == "x" {
                    let settings = &mut G_STATE.lock().unwrap().settings;
                    settings.bone_auto = true;
                    return None;
                } else if let Some(new_val) = val.parse::<u8>().ok() {
                    if vec![0, 1, 2, 3].contains(&new_val) {
                        let settings = &mut G_STATE.lock().unwrap().settings;
                        settings.bone = new_val.into();
                        settings.bone_auto = false;
                        return None;
                    }
                    return Some(
                        "Invalid value. 'bone' value must be between 0 and 3.".to_string(),
                    );
                }
                Some("Invalid value. ".to_string())
            },
        )
        .add_item(
            item_enabled(" 9 - Loot Glow Filled", settings.loot_filled_toggle),
            |_| {
                let settings = &mut G_STATE.lock().unwrap().settings;
                settings.loot_filled_toggle = !settings.loot_filled_toggle;
                settings.loot_filled = if settings.loot_filled_toggle { 14 } else { 0 };
                None
            },
        )
        .add_item(
            item_enabled("10 - Player Glow Filled", settings.player_filled_toggle),
            |_| {
                let settings = &mut G_STATE.lock().unwrap().settings;
                settings.player_filled_toggle = !settings.player_filled_toggle;
                settings.player_glow_inside_value = if settings.player_filled_toggle { 14 } else { 0 };
                None
            },
        )
        .add_input_item(
            item_text("11 - Player Outline Glow Setting Size"),
            "Player Outlines (0 to 255): ",
            |val| {
                if let Some(new_val) = val.parse::<u8>().ok() {
                    let settings = &mut G_STATE.lock().unwrap().settings;
                    settings.player_glow_outline_size = new_val; //[0, 255]
                    return Some(format!(
                        "Player Outline updated to: {}",
                        settings.player_glow_outline_size
                    ));
                }
                Some("Invalid value. 'outlinesize' value must be between 0 and 255.".to_string())
            },
        )
        .add_item(
            item_text("12 - Update Glow Colors"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::GlowColorMenu);
                None
            },
        )
        .add_input_item(
            format_item(
                "13 - Change ADS FOV",
                Span::styled(
                    format!("{}", settings.ads_fov),
                    Style::default().fg(Color::White),
                ),
            ),
            "New value for 'ADS FOV': 
            (1 to 50)",
            |val| {
                if let Some(new_val) = val.parse::<f32>().ok() {
                    if new_val >= 1.0 && new_val <= 50.0 {
                        let settings = &mut G_STATE.lock().unwrap().settings;
                        settings.ads_fov = new_val;
                        return None;
                    }
                }
                Some("Invalid value. 'ADS FOV' value must be between 1.0 and 50.".to_string())
            },
        )
        .add_input_item(
            format_item(
                "14 - Change Non-ADS FOV",
                Span::styled(
                    format!("{}", settings.non_ads_fov),
                    Style::default().fg(Color::White),
                ),
            ),
            "New value for 'Non-ADS FOV': 
            (1 to 50)",
            |val| {
                if let Some(new_val) = val.parse::<f32>().ok() {
                    if new_val >= 1.0 && new_val <= 50.0 {
                        let settings = &mut G_STATE.lock().unwrap().settings;
                        settings.non_ads_fov = new_val;
                        return None;
                    }
                }
                Some("Invalid value. 'Non-ADS FOV' value must be between 1.0 and 50.".to_string())
            },
        );
    menu = add_toggle_item!(
        menu,
        "15 - Super Glide",
        settings.super_key_toggle,
        super_key_toggle
    );
    menu = menu
        .add_item(
            item_text("16 - Item Filter Settings"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::ItemFilterMenu);
                None
            },
        )
        .add_item(
            item_text("17 - Hot Key Setting"),
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
                let settings = &mut G_STATE.lock().unwrap();
                settings.settings.load_settings = !settings.settings.load_settings;
                if settings.settings.load_settings {
                    None
                } else {
                    Some("Hello World".to_string())
                }
            },
        );
    menu.next_id();
    menu = add_toggle_item!(menu, "20 - Death Boxes", settings.deathbox, deathbox);
    menu = menu
        .add_dummy_item()
        .add_item(item_text("21 - Save Settings"), |_| {
            Some(
                if crate::save_settings() {
                    "Saved"
                } else {
                    "Failed"
                }
                .to_string(),
            )
        })
        .add_item(item_text("22 - Load Settings"), |_| {
            let mut result = "Loaded".to_string();
            let settings = crate::config::get_configuration().unwrap_or_else(|e| {
                result = format!("{}", e);
                result += "\nFallback to defalut configuration.";
                crate::config::Config::default()
            });
            G_STATE.lock().unwrap().settings = settings;
            Some(result)
        })
        .add_dummy_item()
        .add_item(
            format_item(
                "23 - Toggle NoNadeAim",
                Span::styled(
                    if settings.no_nade_aim {
                        "No Nade Aim"
                    } else {
                        "Throwing aimbot on"
                    },
                    Style::default().fg(Color::White),
                ),
            ),
            |_| {
                let settings = &mut G_STATE.lock().unwrap().settings;
                settings.no_nade_aim = !settings.no_nade_aim;
                None
            },
        );
    menu = add_toggle_item!(menu, "24 - Toggle 1v1", settings.onevone, onevone);
    menu = add_toggle_item!(
        menu,
        "25 - Toggle No Recoil",
        settings.aim_no_recoil,
        aim_no_recoil
    );
    menu = menu.add_input_item(
        format_item(
            "26 - Set Game FPS for Aim Prediction",
            Span::styled(
                if settings.calc_game_fps {
                    format!("calc game fps")
                } else {
                    format!("{:.1}", settings.game_fps)
                },
                Style::default().fg(Color::White),
            ),
        ),
        "New value for 'Game FPS for Aim Predict':
        (0 to 500) 
        0 => calc game fps",
        |val| {
            if let Some(new_val) = val.parse::<u16>().ok() {
                let settings = &mut G_STATE.lock().unwrap().settings;
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
        "27 - Toggle F8 Map Radar",
        settings.map_radar_testing,
        map_radar_testing
    );
    menu = add_toggle_item!(
        menu,
        "28 - Toggle Player Armor Glow Color",
        settings.player_glow_armor_color,
        player_glow_armor_color
    );
    menu.add_dummy_item()
        .add_item(
            format_item(
                "29 - Toggle Overlay",
                if settings.no_overlay {
                    Span::styled("no-overlay", Style::default().white())
                } else {
                    Span::styled("external-overlay", Style::default().green())
                },
            ),
            |_| {
                let settings = &mut G_STATE.lock().unwrap().settings;
                settings.no_overlay = !settings.no_overlay;
                None
            },
        )
        .into()
}

fn build_glow_color_menu(settings: &config::Config) -> MenuState<'static> {
    fn parse_rgb(val: &String) -> Result<(f32, f32, f32), String> {
        let val: Vec<&str> = val.split(" ").collect();
        if val.len() != 3 {
            return Err(format!("Expecting 3 values but getting {}!", val.len()));
        }
        let r = val[0].parse::<f32>().ok();
        let g = val[1].parse::<f32>().ok();
        let b = val[2].parse::<f32>().ok();
        if r.is_none() || g.is_none() || b.is_none() {
            return Err(format!("This input cannot be parsed as values!"));
        }
        let (r, g, b) = (r.unwrap(), g.unwrap(), b.unwrap());
        if r < 0.0 || r > 1.0 || g < 0.0 || g > 1.0 || b < 0.0 || b > 1.0 {
            return Err(format!("Values out of range!"));
        }
        Ok((r, g, b))
    }
    fn menu_item_rgb(label: &str, (r, g, b): (f32, f32, f32)) -> ListItem {
        ListItem::new(Line::from(vec![
            Span::styled(format!("{: <40}", label), Style::default().white()),
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
    fn prompt_text_rgb(label: &str) -> String {
        format!(
            "Enter RGB values for \"{}\":
    (0-1 for each channel)
    e.g. 
    1 0 0
    0.5 0.5 0.5",
            label
        )
    }

    MenuBuilder::new()
        .title("Glow Color Menu")
        .add_input_item(
            menu_item_rgb(
                "1 - Glow Colors (Not Visible Target)",
                (
                    settings.glow_r_not,
                    settings.glow_g_not,
                    settings.glow_b_not,
                ),
            ),
            &prompt_text_rgb("Not Visible Target"),
            |val| match parse_rgb(&val) {
                Ok((r, g, b)) => {
                    let settings = &mut G_STATE.lock().unwrap().settings;
                    (
                        settings.glow_r_not,
                        settings.glow_g_not,
                        settings.glow_b_not,
                    ) = (r, g, b);
                    Some(format!(
                        "Glow Colors \"Not Visible Target\" updated (R: {}, G: {}, B: {}).",
                        settings.glow_r_not, settings.glow_g_not, settings.glow_b_not
                    ))
                }
                Err(e) => Some(e),
            },
        )
        .add_input_item(
            menu_item_rgb(
                "2 - Glow Colors (Visible Target)",
                (
                    settings.glow_r_viz,
                    settings.glow_g_viz,
                    settings.glow_b_viz,
                ),
            ),
            &prompt_text_rgb("Visible Target"),
            |val| match parse_rgb(&val) {
                Ok((r, g, b)) => {
                    let settings = &mut G_STATE.lock().unwrap().settings;
                    (
                        settings.glow_r_viz,
                        settings.glow_g_viz,
                        settings.glow_b_viz,
                    ) = (r, g, b);
                    Some(format!(
                        "Glow Colors \"Visible Target\" updated (R: {}, G: {}, B: {}).",
                        settings.glow_r_viz, settings.glow_g_viz, settings.glow_b_viz
                    ))
                }
                Err(e) => Some(e),
            },
        )
        .add_input_item(
            menu_item_rgb(
                "3 - Glow Colors (Knocked Target)",
                (
                    settings.glow_r_knocked,
                    settings.glow_g_knocked,
                    settings.glow_b_knocked,
                ),
            ),
            &prompt_text_rgb("Knocked Target"),
            |val| match parse_rgb(&val) {
                Ok((r, g, b)) => {
                    let settings = &mut G_STATE.lock().unwrap().settings;
                    (
                        settings.glow_r_knocked,
                        settings.glow_g_knocked,
                        settings.glow_b_knocked,
                    ) = (r, g, b);
                    Some(format!(
                        "Glow Colors \"Knocked Target\" updated (R: {}, G: {}, B: {}).",
                        settings.glow_r_knocked, settings.glow_g_knocked, settings.glow_b_knocked
                    ))
                }
                Err(e) => Some(e),
            },
        )
        .add_dummy_item()
        .add_item(
            item_text("4 - Back to Main Menu"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_hotkey_menu(settings: &config::Config) -> MenuState<'static> {
    fn menu_item_keycode(label: &str, value: i32) -> ListItem {
        ListItem::new(Line::from(vec![
            Span::styled(format!("{: <40}", label), Style::default().white()),
            Span::styled(format!("{}", value), Style::default().white().underlined()),
        ]))
    }
    fn prompt_text_keycode(label: &str) -> String {
        format!(
            "Enter a new value for \"{}\":
     (e.g., 108 for Left mouse button)",
            label
        )
    }

    MenuBuilder::new()
        .title("Hotkey Menu")
        .add_input_item(
            menu_item_keycode("1 - Aimbot Hot Key 1", settings.aimbot_hot_key_1),
            &prompt_text_keycode("AimbotHotKey1"),
            |val| {
                if let Some(keycode) = val.parse::<u8>().ok() {
                    let settings = &mut G_STATE.lock().unwrap().settings;
                    settings.aimbot_hot_key_1 = keycode as i32;
                    return None;
                }
                Some("Invalid value. 'AimbotHotKey1' value must be between 0 and 255.".to_string())
            },
        )
        .add_input_item(
            menu_item_keycode("2 - Aimbot Hot Key 2", settings.aimbot_hot_key_2),
            &prompt_text_keycode("AimbotHotKey2"),
            |val| {
                if let Some(keycode) = val.parse::<u8>().ok() {
                    let settings = &mut G_STATE.lock().unwrap().settings;
                    settings.aimbot_hot_key_2 = keycode as i32;
                    return None;
                }
                Some("Invalid value. 'AimbotHotKey2' value must be between 0 and 255.".to_string())
            },
        )
        .add_input_item(
            menu_item_keycode("3 - Trigger Bot Hot Key", settings.trigger_bot_hot_key),
            &prompt_text_keycode("TriggerBotHotKey"),
            |val| {
                if let Some(keycode) = val.parse::<u8>().ok() {
                    let settings = &mut G_STATE.lock().unwrap().settings;
                    settings.trigger_bot_hot_key = keycode as i32;
                    return None;
                }
                Some(
                    "Invalid value. 'TriggerBotHotKey' value must be between 0 and 255."
                        .to_string(),
                )
            },
        )
        .add_dummy_item()
        .add_item(item_text("4 - Key Codes"), |handler: &mut TerminalMenu| {
            handler.nav_menu(MenuLevel::KeyCodesMenu);
            None
        })
        .add_dummy_item()
        .add_item(
            item_text("5 - Back to Main Menu"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_item_filter_menu(_settings: &config::Config) -> MenuState<'static> {
    MenuBuilder::new()
        .title("Item Filter Menu")
        .add_item(
            item_text("1 - Light weapons"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::LightWeaponsMenu);
                None
            },
        )
        .add_item(
            item_text("2 - Heavy Weapons"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::HeavyWeaponsMenu);
                None
            },
        )
        .add_item(
            item_text("3 - Energy Weapons"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::EnergyWeaponsMenu);
                None
            },
        )
        .add_item(
            item_text("4 - Sniper Weapons"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::SniperWeaponsMenu);
                None
            },
        )
        .add_item(item_text("5 - Armors"), |handle: &mut TerminalMenu| {
            handle.nav_menu(MenuLevel::ArmorsMenu);
            None
        })
        .add_item(item_text("6 - Healing"), |handle: &mut TerminalMenu| {
            handle.nav_menu(MenuLevel::HealingMenu);
            None
        })
        .add_item(item_text("7 - Nades"), |handle: &mut TerminalMenu| {
            handle.nav_menu(MenuLevel::NadesMenu);
            None
        })
        .add_item(item_text("8 - Backpacks"), |handle: &mut TerminalMenu| {
            handle.nav_menu(MenuLevel::BackpacksMenu);
            None
        })
        .add_item(item_text("9 - Scopes"), |handle: &mut TerminalMenu| {
            handle.nav_menu(MenuLevel::ScopesMenu);
            None
        })
        .add_item(
            item_text("10 - Back to Main Menu"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_light_weapons_menu(settings: &config::Config) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title("Light Weapons Menu");
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from("Red = Disable").red(),
                Span::from(" - ").dark_gray(),
                Span::from("Green = Enabled").green(),
            ])),
            |_| None,
        )
        .no_id()
        .add_dummy_item()
        .add_text_item("Light Weapons:")
        .add_dummy_item();
    menu = add_pick_item!(menu, "1 - P2020", settings.loot.weapon_p2020, weapon_p2020);
    menu = add_pick_item!(menu, "2 - RE-45", settings.loot.weapon_re45, weapon_re45);
    menu = add_pick_item!(
        menu,
        "3 - Alternator",
        settings.loot.weapon_alternator,
        weapon_alternator
    );
    menu = add_pick_item!(menu, "4 - R-99", settings.loot.weapon_r99, weapon_r99);
    menu = add_pick_item!(menu, "5 - R-301", settings.loot.weapon_r301, weapon_r301);
    menu = add_pick_item!(
        menu,
        "6 - M600",
        settings.loot.weapon_spitfire,
        weapon_spitfire
    );
    menu = add_pick_item!(
        menu,
        "7 - G7 Scout",
        settings.loot.weapon_g7_scout,
        weapon_g7_scout
    );
    menu = add_pick_item!(menu, "8 - Light Ammo", settings.loot.lightammo, lightammo);
    menu = menu
        .add_dummy_item()
        .add_text_item("Light Weapon Mags:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "9 - Light Weapon Mag",
        LootLevel::White,
        settings.loot.lightammomag1,
        lightammomag1
    );
    menu = add_colored_loot_item!(
        menu,
        "10 - Light Weapon Mag",
        LootLevel::Blue,
        settings.loot.lightammomag2,
        lightammomag2
    );
    menu = add_colored_loot_item!(
        menu,
        "11 - Light Weapon Mag",
        LootLevel::Purple,
        settings.loot.lightammomag3,
        lightammomag3
    );
    menu = add_colored_loot_item!(
        menu,
        "12 - Light Weapon Mag",
        LootLevel::Gold,
        settings.loot.lightammomag4,
        lightammomag4
    );

    menu = menu
        .add_dummy_item()
        .add_text_item("Weapon Stocks:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "13 - Weapon Stock",
        LootLevel::White,
        settings.loot.stockregular1,
        stockregular1
    );
    menu = add_colored_loot_item!(
        menu,
        "14 - Weapon Stock",
        LootLevel::Blue,
        settings.loot.stockregular2,
        stockregular2
    );
    menu = add_colored_loot_item!(
        menu,
        "15 - Weapon Stock",
        LootLevel::Purple,
        settings.loot.stockregular3,
        stockregular3
    );
    menu = menu
        .add_dummy_item()
        .add_text_item("Weapon Suppressors:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "16 - Weapon Suppressors",
        LootLevel::White,
        settings.loot.suppressor1,
        suppressor1
    );
    menu = add_colored_loot_item!(
        menu,
        "17 - Weapon Suppressors",
        LootLevel::Blue,
        settings.loot.suppressor2,
        suppressor2
    );
    menu = add_colored_loot_item!(
        menu,
        "18 - Weapon Suppressors",
        LootLevel::Purple,
        settings.loot.suppressor3,
        suppressor3
    );
    menu = menu
        .add_dummy_item()
        .add_text_item("Weapon Lasers:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "19 - Weapon Lasers",
        LootLevel::White,
        settings.loot.lasersight1,
        lasersight1
    );
    menu = add_colored_loot_item!(
        menu,
        "20 - Weapon Lasers",
        LootLevel::Blue,
        settings.loot.lasersight2,
        lasersight2
    );
    menu = add_colored_loot_item!(
        menu,
        "21 - Weapon Lasers",
        LootLevel::Purple,
        settings.loot.lasersight3,
        lasersight3
    );
    menu = add_colored_loot_item!(
        menu,
        "22 - Weapon Lasers",
        LootLevel::Gold,
        settings.loot.lasersight4,
        lasersight4
    );
    menu = menu
        .add_dummy_item()
        .add_text_item("Weapon Hop-Ups:")
        .add_dummy_item();
    menu = add_pick_item!(
        menu,
        "23 - Turbo Charger",
        settings.loot.turbo_charger,
        turbo_charger
    );
    menu = add_pick_item!(
        menu,
        "24 - Skull Piecer",
        settings.loot.skull_piecer,
        skull_piecer
    );
    menu = add_pick_item!(
        menu,
        "25 - Hammer Points",
        settings.loot.hammer_point,
        hammer_point
    );
    menu = add_pick_item!(
        menu,
        "26 - Disruptor Rounds",
        settings.loot.disruptor_rounds,
        disruptor_rounds
    );
    menu = add_pick_item!(
        menu,
        "27 - Boosted Loader",
        settings.loot.boosted_loader,
        boosted_loader
    );
    menu.add_dummy_item()
        .add_item(
            item_text("28 - Back to Settings Menu"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_heavy_weapons_menu(settings: &config::Config) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title("Heavy Weapons Menu");
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from("Red = Disable").red(),
                Span::from(" - ").dark_gray(),
                Span::from("Green = Enabled").green(),
            ])),
            |_| None,
        )
        .no_id()
        .add_dummy_item()
        .add_text_item("Heavy Weapons:")
        .add_dummy_item();
    menu = add_pick_item!(
        menu,
        "1 - Flatline",
        settings.loot.weapon_flatline,
        weapon_flatline
    );
    menu = add_pick_item!(
        menu,
        "2 - Hemlock",
        settings.loot.weapon_hemlock,
        weapon_hemlock
    );
    menu = add_pick_item!(
        menu,
        "3 - 30-30 Repeater",
        settings.loot.weapon_3030_repeater,
        weapon_3030_repeater
    );
    menu = add_pick_item!(
        menu,
        "4 - Rampage",
        settings.loot.weapon_rampage,
        weapon_rampage
    );
    menu = add_pick_item!(
        menu,
        "5 - Prowler",
        settings.loot.weapon_prowler,
        weapon_prowler
    );
    menu = add_pick_item!(
        menu,
        "6 - Car SMG",
        settings.loot.weapon_car_smg,
        weapon_car_smg
    );
    menu = add_pick_item!(menu, "7 - Heavy Ammo", settings.loot.heavyammo, heavyammo);
    menu = menu
        .add_dummy_item()
        .add_text_item("Heavy Weapon Mags:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "8 - Heavy Weapon Mag",
        LootLevel::White,
        settings.loot.heavyammomag1,
        heavyammomag1
    );
    menu = add_colored_loot_item!(
        menu,
        "9 - Heavy Weapon Mag",
        LootLevel::Blue,
        settings.loot.heavyammomag2,
        heavyammomag2
    );
    menu = add_colored_loot_item!(
        menu,
        "10 - Heavy Weapon Mag",
        LootLevel::Purple,
        settings.loot.heavyammomag3,
        heavyammomag3
    );
    menu = add_colored_loot_item!(
        menu,
        "11 - Heavy Weapon Mag",
        LootLevel::Gold,
        settings.loot.heavyammomag4,
        heavyammomag4
    );

    menu = menu
        .add_dummy_item()
        .add_text_item("Weapon Stocks:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "12 - Weapon Stock",
        LootLevel::White,
        settings.loot.stockregular1,
        stockregular1
    );
    menu = add_colored_loot_item!(
        menu,
        "13 - Weapon Stock",
        LootLevel::Blue,
        settings.loot.stockregular2,
        stockregular2
    );
    menu = add_colored_loot_item!(
        menu,
        "14 - Weapon Stock",
        LootLevel::Purple,
        settings.loot.stockregular3,
        stockregular3
    );
    menu = menu
        .add_dummy_item()
        .add_text_item("Weapon Suppressors:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "15 - Weapon Suppressors",
        LootLevel::White,
        settings.loot.suppressor1,
        suppressor1
    );
    menu = add_colored_loot_item!(
        menu,
        "16 - Weapon Suppressors",
        LootLevel::Blue,
        settings.loot.suppressor2,
        suppressor2
    );
    menu = add_colored_loot_item!(
        menu,
        "17 - Weapon Suppressors",
        LootLevel::Purple,
        settings.loot.suppressor3,
        suppressor3
    );
    menu = menu
        .add_dummy_item()
        .add_text_item("Weapon Lasers:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "18 - Weapon Lasers",
        LootLevel::White,
        settings.loot.lasersight1,
        lasersight1
    );
    menu = add_colored_loot_item!(
        menu,
        "19 - Weapon Lasers",
        LootLevel::Blue,
        settings.loot.lasersight2,
        lasersight2
    );
    menu = add_colored_loot_item!(
        menu,
        "20 - Weapon Lasers",
        LootLevel::Purple,
        settings.loot.lasersight3,
        lasersight3
    );
    menu = add_colored_loot_item!(
        menu,
        "21 - Weapon Lasers",
        LootLevel::Gold,
        settings.loot.lasersight4,
        lasersight4
    );
    menu = menu
        .add_dummy_item()
        .add_text_item("Weapon Hop-Ups:")
        .add_dummy_item();
    menu = add_pick_item!(
        menu,
        "22 - Turbo Charger",
        settings.loot.turbo_charger,
        turbo_charger
    );
    menu = add_pick_item!(
        menu,
        "23 - Skull Piecer",
        settings.loot.skull_piecer,
        skull_piecer
    );
    menu = add_pick_item!(
        menu,
        "24 - Hammer Points",
        settings.loot.hammer_point,
        hammer_point
    );
    menu = add_pick_item!(
        menu,
        "25 - Disruptor Rounds",
        settings.loot.disruptor_rounds,
        disruptor_rounds
    );
    menu = add_pick_item!(
        menu,
        "26 - Boosted Loader",
        settings.loot.boosted_loader,
        boosted_loader
    );
    menu.add_dummy_item()
        .add_item(
            item_text("27 - Back to Settings Menu"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_energy_weapons_menu(settings: &config::Config) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title("Energy Weapons Menu");
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from("Red = Disable").red(),
                Span::from(" - ").dark_gray(),
                Span::from("Green = Enabled").green(),
            ])),
            |_| None,
        )
        .no_id()
        .add_dummy_item()
        .add_text_item("Energy Weapons:")
        .add_dummy_item();
    menu = add_pick_item!(menu, "1 - LSTAR", settings.loot.weapon_lstar, weapon_lstar);
    menu = add_pick_item!(
        menu,
        "2 - Nemesis",
        settings.loot.weapon_nemesis,
        weapon_nemesis
    );
    menu = add_pick_item!(menu, "3 - Havoc", settings.loot.weapon_havoc, weapon_havoc);
    menu = add_pick_item!(
        menu,
        "4 - Deovtion",
        settings.loot.weapon_devotion,
        weapon_devotion
    );
    menu = add_pick_item!(
        menu,
        "5 - Tripple Take",
        settings.loot.weapon_triple_take,
        weapon_triple_take
    );
    menu = add_pick_item!(menu, "6 - Volt", settings.loot.weapon_volt, weapon_volt);
    menu = add_pick_item!(
        menu,
        "7 - Energy Ammo",
        settings.loot.energyammo,
        energyammo
    );
    menu = menu
        .add_dummy_item()
        .add_text_item("Energy Weapon Mags:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "8 - Energy Weapon Mag",
        LootLevel::White,
        settings.loot.energyammomag1,
        energyammomag1
    );
    menu = add_colored_loot_item!(
        menu,
        "9 - Energy Weapon Mag",
        LootLevel::Blue,
        settings.loot.energyammomag2,
        energyammomag2
    );
    menu = add_colored_loot_item!(
        menu,
        "10 - Energy Weapon Mag",
        LootLevel::Purple,
        settings.loot.energyammomag3,
        energyammomag3
    );
    menu = add_colored_loot_item!(
        menu,
        "11 - Energy Weapon Mag",
        LootLevel::Gold,
        settings.loot.energyammomag4,
        energyammomag4
    );

    menu = menu
        .add_dummy_item()
        .add_text_item("Weapon Stocks:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "12 - Weapon Stock",
        LootLevel::White,
        settings.loot.stockregular1,
        stockregular1
    );
    menu = add_colored_loot_item!(
        menu,
        "13 - Weapon Stock",
        LootLevel::Blue,
        settings.loot.stockregular2,
        stockregular2
    );
    menu = add_colored_loot_item!(
        menu,
        "14 - Weapon Stock",
        LootLevel::Purple,
        settings.loot.stockregular3,
        stockregular3
    );
    menu = menu
        .add_dummy_item()
        .add_text_item("Weapon Suppressors:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "15 - Weapon Suppressors",
        LootLevel::White,
        settings.loot.suppressor1,
        suppressor1
    );
    menu = add_colored_loot_item!(
        menu,
        "16 - Weapon Suppressors",
        LootLevel::Blue,
        settings.loot.suppressor2,
        suppressor2
    );
    menu = add_colored_loot_item!(
        menu,
        "17 - Weapon Suppressors",
        LootLevel::Purple,
        settings.loot.suppressor3,
        suppressor3
    );
    menu = menu
        .add_dummy_item()
        .add_text_item("Weapon Lasers:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "18 - Weapon Lasers",
        LootLevel::White,
        settings.loot.lasersight1,
        lasersight1
    );
    menu = add_colored_loot_item!(
        menu,
        "19 - Weapon Lasers",
        LootLevel::Blue,
        settings.loot.lasersight2,
        lasersight2
    );
    menu = add_colored_loot_item!(
        menu,
        "20 - Weapon Lasers",
        LootLevel::Purple,
        settings.loot.lasersight3,
        lasersight3
    );
    menu = add_colored_loot_item!(
        menu,
        "21 - Weapon Lasers",
        LootLevel::Gold,
        settings.loot.lasersight4,
        lasersight4
    );
    menu = menu
        .add_dummy_item()
        .add_text_item("Weapon Hop-Ups:")
        .add_dummy_item();
    menu = add_pick_item!(
        menu,
        "22 - Turbo Charger",
        settings.loot.turbo_charger,
        turbo_charger
    );
    menu = add_pick_item!(
        menu,
        "23 - Skull Piecer",
        settings.loot.skull_piecer,
        skull_piecer
    );
    menu = add_pick_item!(
        menu,
        "24 - Hammer Points",
        settings.loot.hammer_point,
        hammer_point
    );
    menu = add_pick_item!(
        menu,
        "25 - Disruptor Rounds",
        settings.loot.disruptor_rounds,
        disruptor_rounds
    );
    menu = add_pick_item!(
        menu,
        "26 - Boosted Loader",
        settings.loot.boosted_loader,
        boosted_loader
    );
    menu.add_dummy_item()
        .add_item(
            item_text("27 - Back to Settings Menu"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_sniper_weapons_menu(settings: &config::Config) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title("Sniper Weapons Menu");
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from("Red = Disable").red(),
                Span::from(" - ").dark_gray(),
                Span::from("Green = Enabled").green(),
            ])),
            |_| None,
        )
        .no_id()
        .add_dummy_item()
        .add_text_item("Sniper Weapons:")
        .add_dummy_item();
    menu = add_pick_item!(
        menu,
        "1 - Wingman",
        settings.loot.weapon_wingman,
        weapon_wingman
    );
    menu = add_pick_item!(
        menu,
        "2 - Longbow",
        settings.loot.weapon_longbow,
        weapon_longbow
    );
    menu = add_pick_item!(
        menu,
        "3 - Charge Rifle",
        settings.loot.weapon_charge_rifle,
        weapon_charge_rifle
    );
    menu = add_pick_item!(
        menu,
        "4 - Sentinel",
        settings.loot.weapon_sentinel,
        weapon_sentinel
    );
    menu = add_pick_item!(menu, "5 - Bow", settings.loot.weapon_bow, weapon_bow);
    menu = add_pick_item!(
        menu,
        "6 - Sniper Ammo",
        settings.loot.sniperammo,
        sniperammo
    );
    menu = menu
        .add_dummy_item()
        .add_text_item("Sniper Weapon Mags:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "7 - Sniper Weapon Mag",
        LootLevel::White,
        settings.loot.sniperammomag1,
        sniperammomag1
    );
    menu = add_colored_loot_item!(
        menu,
        "8 - Sniper Weapon Mag",
        LootLevel::Blue,
        settings.loot.sniperammomag2,
        sniperammomag2
    );
    menu = add_colored_loot_item!(
        menu,
        "9 - Sniper Weapon Mag",
        LootLevel::Purple,
        settings.loot.sniperammomag3,
        sniperammomag3
    );
    menu = add_colored_loot_item!(
        menu,
        "10 - Sniper Weapon Mag",
        LootLevel::Gold,
        settings.loot.sniperammomag4,
        sniperammomag4
    );

    menu = menu
        .add_dummy_item()
        .add_text_item("Weapon Stocks:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "11 - Weapon Stock",
        LootLevel::White,
        settings.loot.stockregular1,
        stockregular1
    );
    menu = add_colored_loot_item!(
        menu,
        "12 - Weapon Stock",
        LootLevel::Blue,
        settings.loot.stockregular2,
        stockregular2
    );
    menu = add_colored_loot_item!(
        menu,
        "13 - Weapon Stock",
        LootLevel::Purple,
        settings.loot.stockregular3,
        stockregular3
    );
    menu = menu
        .add_dummy_item()
        .add_text_item("Weapon Suppressors:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "14 - Weapon Suppressors",
        LootLevel::White,
        settings.loot.suppressor1,
        suppressor1
    );
    menu = add_colored_loot_item!(
        menu,
        "15 - Weapon Suppressors",
        LootLevel::Blue,
        settings.loot.suppressor2,
        suppressor2
    );
    menu = add_colored_loot_item!(
        menu,
        "16 - Weapon Suppressors",
        LootLevel::Purple,
        settings.loot.suppressor3,
        suppressor3
    );
    menu = menu
        .add_dummy_item()
        .add_text_item("Weapon Hop-Ups:")
        .add_dummy_item();
    menu = add_pick_item!(
        menu,
        "17 - Turbo Charger",
        settings.loot.turbo_charger,
        turbo_charger
    );
    menu = add_pick_item!(
        menu,
        "18 - Skull Piecer",
        settings.loot.skull_piecer,
        skull_piecer
    );
    menu = add_pick_item!(
        menu,
        "19 - Hammer Points",
        settings.loot.hammer_point,
        hammer_point
    );
    menu = add_pick_item!(
        menu,
        "20 - Disruptor Rounds",
        settings.loot.disruptor_rounds,
        disruptor_rounds
    );
    menu = add_pick_item!(
        menu,
        "21 - Boosted Loader",
        settings.loot.boosted_loader,
        boosted_loader
    );
    menu.add_dummy_item()
        .add_item(
            item_text("22 - Back to Settings Menu"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_armors_menu(settings: &config::Config) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title("Armors Menu");
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from("Red = Disable").red(),
                Span::from(" - ").dark_gray(),
                Span::from("Green = Enabled").green(),
            ])),
            |_| None,
        )
        .no_id()
        .add_dummy_item()
        .add_text_item("Armors:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "1 - Armor",
        LootLevel::White,
        settings.loot.shieldupgrade1,
        shieldupgrade1
    );
    menu = add_colored_loot_item!(
        menu,
        "2 - Armor",
        LootLevel::Blue,
        settings.loot.shieldupgrade2,
        shieldupgrade2
    );
    menu = add_colored_loot_item!(
        menu,
        "3 - Armor",
        LootLevel::Purple,
        settings.loot.shieldupgrade3,
        shieldupgrade3
    );
    menu = add_colored_loot_item!(
        menu,
        "4 - Armor",
        LootLevel::Gold,
        settings.loot.shieldupgrade4,
        shieldupgrade4
    );
    menu = add_colored_loot_item!(
        menu,
        "5 - Armor",
        LootLevel::Red,
        settings.loot.shieldupgrade5,
        shieldupgrade5
    );
    menu = menu
        .add_dummy_item()
        .add_text_item("Helmets:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "6 - Helmet",
        LootLevel::White,
        settings.loot.shieldupgradehead1,
        shieldupgradehead1
    );
    menu = add_colored_loot_item!(
        menu,
        "7 - Helmet",
        LootLevel::Blue,
        settings.loot.shieldupgradehead2,
        shieldupgradehead2
    );
    menu = add_colored_loot_item!(
        menu,
        "8 - Helmet",
        LootLevel::Purple,
        settings.loot.shieldupgradehead3,
        shieldupgradehead3
    );
    menu = add_colored_loot_item!(
        menu,
        "9 - Helmet",
        LootLevel::Gold,
        settings.loot.shieldupgradehead4,
        shieldupgradehead4
    );
    menu = menu
        .add_dummy_item()
        .add_text_item("Knockdown Shields:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "10 - Knockdown Shield",
        LootLevel::White,
        settings.loot.shielddown1,
        shielddown1
    );
    menu = add_colored_loot_item!(
        menu,
        "11 - Knockdown Shield",
        LootLevel::Blue,
        settings.loot.shielddown2,
        shielddown2
    );
    menu = add_colored_loot_item!(
        menu,
        "12 - Knockdown Shield",
        LootLevel::Purple,
        settings.loot.shielddown3,
        shielddown3
    );
    menu = add_colored_loot_item!(
        menu,
        "13 - Knockdown Shield",
        LootLevel::Gold,
        settings.loot.shielddown4,
        shielddown4
    );
    menu.add_dummy_item()
        .add_item(
            item_text("14 - Back to Settings Menu"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_healing_menu(settings: &config::Config) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title("Healing Items Menu");
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from("Red = Disable").red(),
                Span::from(" - ").dark_gray(),
                Span::from("Green = Enabled").green(),
            ])),
            |_| None,
        )
        .no_id()
        .add_dummy_item()
        .add_text_item("Healing Items:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "1 - Accelerant",
        LootLevel::Blue,
        settings.loot.accelerant,
        accelerant
    );
    menu = add_colored_loot_item!(
        menu,
        "2 - Phoenix",
        LootLevel::Purple,
        settings.loot.phoenix,
        phoenix
    );
    menu = add_colored_loot_item!(
        menu,
        "3 - Small Health",
        LootLevel::White,
        settings.loot.healthsmall,
        healthsmall
    );
    menu = add_colored_loot_item!(
        menu,
        "4 - Large Health",
        LootLevel::White,
        settings.loot.healthlarge,
        healthlarge
    );
    menu = add_colored_loot_item!(
        menu,
        "5 - Small Shield Batt",
        LootLevel::White,
        settings.loot.shieldbattsmall,
        shieldbattsmall
    );
    menu = add_colored_loot_item!(
        menu,
        "6 - Large Shield Batt",
        LootLevel::White,
        settings.loot.shieldbattlarge,
        shieldbattlarge
    );
    menu.add_dummy_item()
        .add_item(
            item_text("7 - Back to Settings Menu"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_nades_menu(settings: &config::Config) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title("Nade Items Menu");
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from("Red = Disable").red(),
                Span::from(" - ").dark_gray(),
                Span::from("Green = Enabled").green(),
            ])),
            |_| None,
        )
        .no_id()
        .add_dummy_item()
        .add_text_item("Nade Items:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "1 - Frag Grenade",
        LootLevel::Red,
        settings.loot.grenade_frag,
        grenade_frag
    );
    menu = add_colored_loot_item!(
        menu,
        "2 - Arc Star",
        LootLevel::Blue,
        settings.loot.grenade_arc_star,
        grenade_arc_star
    );
    menu = add_colored_loot_item!(
        menu,
        "3 - Thermite",
        LootLevel::Red,
        settings.loot.grenade_thermite,
        grenade_thermite
    );
    menu.add_dummy_item()
        .add_item(
            item_text("4 - Back to Settings Menu"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_backpacks_menu(settings: &config::Config) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title("Backpacks Menu");
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from("Red = Disable").red(),
                Span::from(" - ").dark_gray(),
                Span::from("Green = Enabled").green(),
            ])),
            |_| None,
        )
        .no_id()
        .add_dummy_item()
        .add_text_item("Backpacks:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "1 - Light Backpack",
        LootLevel::White,
        settings.loot.lightbackpack,
        lightbackpack
    );
    menu = add_colored_loot_item!(
        menu,
        "2 - Medium Backpack",
        LootLevel::Blue,
        settings.loot.medbackpack,
        medbackpack
    );
    menu = add_colored_loot_item!(
        menu,
        "3 - Heavy Backpack",
        LootLevel::Purple,
        settings.loot.heavybackpack,
        heavybackpack
    );
    menu = add_colored_loot_item!(
        menu,
        "4 - Gold Backpack",
        LootLevel::Gold,
        settings.loot.goldbackpack,
        goldbackpack
    );
    menu.add_dummy_item()
        .add_item(
            item_text("5 - Back to Settings Menu"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_scopes_menu(settings: &config::Config) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title("Scopes Menu");
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from("Red = Disable").red(),
                Span::from(" - ").dark_gray(),
                Span::from("Green = Enabled").green(),
            ])),
            |_| None,
        )
        .no_id()
        .add_dummy_item()
        .add_text_item("Scopes:")
        .add_dummy_item();
    menu = add_colored_loot_item!(
        menu,
        "1 - 1x HCOG",
        LootLevel::White,
        settings.loot.optic1xhcog,
        optic1xhcog
    );
    menu = add_colored_loot_item!(
        menu,
        "2 - 2x HCOG",
        LootLevel::Blue,
        settings.loot.optic2xhcog,
        optic2xhcog
    );
    menu = add_colored_loot_item!(
        menu,
        "3 - 1x HOLO",
        LootLevel::White,
        settings.loot.opticholo1x,
        opticholo1x
    );
    menu = add_colored_loot_item!(
        menu,
        "4 - 1x-2x HOLO",
        LootLevel::Blue,
        settings.loot.opticholo1x2x,
        opticholo1x2x
    );
    menu = add_colored_loot_item!(
        menu,
        "5 - Optic Threat",
        LootLevel::Gold,
        settings.loot.opticthreat,
        opticthreat
    );
    menu = add_colored_loot_item!(
        menu,
        "6 - 3x HCOG",
        LootLevel::Purple,
        settings.loot.optic3xhcog,
        optic3xhcog
    );
    menu = add_colored_loot_item!(
        menu,
        "7 - 2x-4x HCOG",
        LootLevel::Purple,
        settings.loot.optic2x4x,
        optic2x4x
    );
    menu = add_colored_loot_item!(
        menu,
        "8 - 6x Sniper Optic",
        LootLevel::Blue,
        settings.loot.opticsniper6x,
        opticsniper6x
    );
    menu = add_colored_loot_item!(
        menu,
        "9 - 4x-8x Sniper Optic",
        LootLevel::Purple,
        settings.loot.opticsniper4x8x,
        opticsniper4x8x
    );
    menu = add_colored_loot_item!(
        menu,
        "10 - Sniper Threat",
        LootLevel::Gold,
        settings.loot.opticsniperthreat,
        opticsniperthreat
    );
    menu.add_dummy_item()
        .add_item(
            item_text("11 - Back to Settings Menu"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
        )
        .into()
}

fn build_key_codes_menu(_settings: &config::Config) -> MenuState<'static> {
    MenuBuilder::new()
        .title("Key Codes:")
        .add_text_item("108 Left mouse button (mouse1)")
        .add_text_item("109 Right mouse button (mouse2)")
        .add_text_item("110 Middle mouse button (mouse3)")
        .add_text_item("111 Side mouse button (mouse4)")
        .add_text_item("112 Side mouse button (mouse5)")
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
            item_text("Back to Hotkey Menu"),
            |handle: &mut TerminalMenu| {
                handle.nav_menu(MenuLevel::HotkeyMenu);
                None
            },
        )
        .add_dummy_item()
        .add_item(
            item_text("Back to Main Menu"),
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
    List::new(
        list_items
            .iter()
            .skip(scroll_top)
            .enumerate()
            .map(|(index, item)| {
                if index == selected_index - scroll_top {
                    item.clone().on_light_yellow()
                } else {
                    item.clone()
                }
            })
            .collect::<Vec<ListItem>>(),
    )
}

fn text_enabled(v: bool) -> &'static str {
    // if v {
    //     "on"
    // } else {
    //     "off"
    // }
    if v {
        "Enabled"
    } else {
        "Disabled"
    }
}
fn format_label<T>(label: T) -> Span<'static>
where
    T: Into<String>,
{
    Span::styled(
        format!("{: <40}", label.into()),
        Style::default().fg(Color::White),
    )
}
fn format_value(label: &str) -> Span {
    Span::styled(format!("{: <40}", label), Style::default().fg(Color::Green))
}
fn format_item<T>(label: T, value: Span) -> ListItem
where
    T: Into<String>,
{
    ListItem::new(Line::from(vec![
        format_label(label.into()),
        Span::styled(" (Current: ", Style::default().fg(Color::DarkGray)),
        value,
        Span::styled(")", Style::default().fg(Color::DarkGray)),
    ]))
}
fn span_enabled(v: bool) -> Span<'static> {
    Span::styled(
        text_enabled(v),
        Style::default().fg(if v { Color::Green } else { Color::White }),
    )
}
fn item_enabled(label: &str, v: bool) -> ListItem {
    format_item(label, span_enabled(v))
}
fn item_text(label: &str) -> ListItem {
    ListItem::new(Line::from(format_label(label)))
}
fn item_dummy() -> ListItem<'static> {
    ListItem::new(Line::from("‌​‌‌​​​‌‌‌‍‌​‌‌​‌​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌​‌‌‌‌‌‍‌​‌‌‌‌​​‌‌‍‌​‌‌​​​​‌‌‍‌​‌‌‌​​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌‌​​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​‌‌‍‌​‌‌​‌​​‌‌‍‌​‌‌​‌‌​‌‌‍‌​‌‌​​‌​‌‌‍‌​‌‌​‌‌‌​‌‍‌​‌‌‌​‌​‌‌"))
}
fn block_title(title: &str) -> Paragraph<'_> {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title =
        Paragraph::new(Text::styled(title, Style::default().fg(Color::Green))).block(title_block);
    title
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu() {
        super::super::main().unwrap();
    }
}
