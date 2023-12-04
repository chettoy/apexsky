use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::config;

#[derive(Debug)]
pub struct TerminalMenu<'a> {
    menu_level: MenuLevel,
    menu_state: MenuState<'a>,
    data: &'a crate::config::Config,
}

#[derive(Debug, Clone)]
struct MenuState<'a> {
    title: &'a str,
    items: Vec<ListItem<'a>>,
    nav_index: usize,
    nav_index_max: usize,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum MenuLevel {
    #[default]
    MainMenu,
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
}

impl<'a> TerminalMenu<'a> {
    pub fn from(data: &'a crate::config::Config) -> Self {
        Self {
            menu_level: Default::default(),
            menu_state: build_main_menu(data),
            data,
        }
    }

    pub fn render(&self, f: &mut Frame) {
        self.render_menu(f);
    }

    pub fn nav_up(&mut self) {
        if self.menu_state.nav_index > 0 {
            self.menu_state.nav_index -= 1;
        }
    }

    pub fn nav_down(&mut self) {
        if self.menu_state.nav_index < self.menu_state.nav_index_max {
            self.menu_state.nav_index += 1;
        }
    }

    pub fn nav_back(&mut self) {
        if self.menu_level != MenuLevel::MainMenu {
            self.menu_level = MenuLevel::MainMenu;
        }
    }

    pub fn nav_enter(&mut self) {}

    fn nav_menu(&mut self, menu_level: MenuLevel) {
        self.menu_level = menu_level;
        self.menu_state = match self.menu_level {
            MenuLevel::MainMenu => build_main_menu(&self.data),
            MenuLevel::ItemFilterMenu => build_item_filter_menu(&self.data),
            MenuLevel::LightWeaponsMenu => todo!(),
            MenuLevel::HeavyWeaponsMenu => todo!(),
            MenuLevel::EnergyWeaponsMenu => todo!(),
            MenuLevel::SniperWeaponsMenu => todo!(),
            MenuLevel::ArmorsMenu => todo!(),
            MenuLevel::HealingMenu => todo!(),
            MenuLevel::NadesMenu => todo!(),
            MenuLevel::BackpacksMenu => todo!(),
            MenuLevel::ScopesMenu => todo!(),
        };
    }

    fn render_menu(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(2),
            ])
            .split(f.size());

        f.render_widget(block_title(self.menu_state.title), chunks[0]);
        f.render_widget(
            format_selected_list(&self.menu_state.items, self.menu_state.nav_index),
            chunks[1],
        );
    }
}

fn build_main_menu(settings: &config::Config) -> MenuState {
    let mut list_items = Vec::<ListItem>::new();

    list_items.push(item_enabled(" 1 - Firing Range", settings.firing_range));
    list_items.push(item_enabled(" 2 - TDMToggle", settings.tdm_toggle));
    list_items.push(item_enabled(" 3 - Keyboard", settings.keyboard));
    list_items.push(item_enabled(" 4 - Gamepad", settings.gamepad));
    list_items.push(item_enabled(" 5 - Item Glow", settings.item_glow));
    list_items.push(item_enabled(" 6 - Player Glow", settings.player_glow));
    list_items.push(format_item(
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
    ));
    list_items.push(format_item(
        " 8 - Change Bone Aim Value",
        Span::styled(
            match settings.bone {
                0 => "Head",
                1 => "Neck",
                2 => "Chest",
                3 => "Gut Shut",
                _ => "Unknown",
            },
            Style::new().white(),
        ),
    ));
    list_items.push(item_enabled(
        " 9 - Loot Glow Filled",
        settings.loot_filled_toggle,
    ));
    list_items.push(item_enabled(
        "10 - Player Glow Filled",
        settings.player_filled_toggle,
    ));
    list_items.push(item_text("11 - Player Outline Glow Setting Size"));
    list_items.push(item_text("12 - Update Glow Colors"));
    list_items.push(format_item(
        "13 - Change ADS FOV",
        Span::styled(
            format!("{}", settings.ads_fov),
            Style::default().fg(Color::White),
        ),
    ));
    list_items.push(format_item(
        "14 - Change Non-ADS FOV",
        Span::styled(
            format!("{}", settings.non_ads_fov),
            Style::default().fg(Color::White),
        ),
    ));
    list_items.push(item_enabled("15 - Super Glide", settings.super_key_toggle));
    list_items.push(item_text("16 - Item Filter Settings"));
    list_items.push(item_dummy());
    list_items.push(item_text("17 - Aiming Key One Setting"));
    list_items.push(item_text("18 - Aiming Key Two Setting"));
    list_items.push(item_text("19 - Triggerbot Key Setting"));
    list_items.push(item_dummy());
    list_items.push(item_enabled("20 - Death Boxes", settings.deathbox));
    list_items.push(item_dummy());
    list_items.push(item_text("21 - Save Settings"));
    list_items.push(item_text("22 - Load Settings"));
    list_items.push(item_dummy());
    list_items.push(format_item(
        "23 - Toggle NoNadeAim",
        Span::styled(
            if settings.no_nade_aim {
                "No Nade Aim"
            } else {
                "Throwing aimbot on"
            },
            Style::default().fg(Color::White),
        ),
    ));
    list_items.push(item_enabled("24 - Toggle 1v1", settings.onevone));
    list_items.push(item_enabled(
        "25 - Toggle No Recoil",
        settings.aim_no_recoil,
    ));
    list_items.push(format_item(
        "26 - Set Game FPS for Aim Prediction",
        Span::styled(
            if settings.calc_game_fps {
                format!("calc game fps")
            } else {
                format!("{:.1}", settings.game_fps)
            },
            Style::default().fg(Color::White),
        ),
    ));

    let max_index = list_items.len() - 1;

    MenuState {
        title: "Main Menu",
        items: list_items,
        nav_index: 0,
        nav_index_max: max_index,
    }
}

fn build_item_filter_menu(settings: &config::Config) -> MenuState {
    let mut list_items = Vec::<ListItem>::new();

    list_items.push(item_text("1 - Light weapons"));
    list_items.push(item_text("2 - Heavy Weapons"));
    list_items.push(item_text("3 - Energy Weapons"));
    list_items.push(item_text("4 - Sniper Weapons"));
    list_items.push(item_text("5 - Armors"));
    list_items.push(item_text("6 - Healing"));
    list_items.push(item_text("7 - Nades"));
    list_items.push(item_text("8 - Backpacks"));
    list_items.push(item_text("9 - Scopes"));
    list_items.push(item_text("10 - Back to Main Menu"));

    let max_index = list_items.len() - 1;

    MenuState {
        title: "Item Filter Menu",
        items: list_items,
        nav_index: 0,
        nav_index_max: max_index,
    }
}

fn format_selected_list<'a>(list_items: &'a Vec<ListItem<'a>>, selected_index: usize) -> List<'a> {
    List::new(
        list_items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                if index == selected_index {
                    item.clone().on_light_yellow()
                } else {
                    item.clone()
                }
            })
            .collect::<Vec<ListItem>>(),
    )
}

fn text_enabled(v: bool) -> &'static str {
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
        format!("{: <30}", label.into()),
        Style::default().fg(Color::White),
    )
}
fn format_value(label: &str) -> Span {
    Span::styled(format!("{: <30}", label), Style::default().fg(Color::Green))
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
fn item_enabled(label: &str, v: bool) -> ListItem {
    format_item(
        label,
        Span::styled(
            text_enabled(v),
            Style::default().fg(if v { Color::Green } else { Color::White }),
        ),
    )
}
fn item_text(label: &str) -> ListItem {
    ListItem::new(Line::from(format_label(label)))
}
fn item_dummy() -> ListItem<'static> {
    ListItem::new(Line::default())
}
fn block_title(title: &str) -> Paragraph<'_> {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title =
        Paragraph::new(Text::styled(title, Style::default().fg(Color::Green))).block(title_block);
    title
}
