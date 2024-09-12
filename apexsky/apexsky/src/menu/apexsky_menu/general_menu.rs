use std::{
    any::Any,
    borrow::Cow,
    collections::HashMap,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
};

use chrono::Datelike;
use crossterm::event::{MouseEvent, MouseEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use super::{alert, prompt, MenuState, TerminalMenu};
use crate::i18n::{I18nBundle, MessageId};

pub trait GeneralMenuName: Debug + Clone + Hash {
    fn rebuild_state(self) -> Box<dyn MenuState>;
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
pub struct GeneralMenu<'a, L>
where
    L: GeneralMenuName,
{
    menu_level: L,
    title: Cow<'a, str>,
    items: Vec<ListItem<'a>>,
    handler: HashMap<usize, Arc<dyn CallbackTrait>>,
    input_handlers: HashMap<usize, (String, Box<fn(String) -> Option<String>>)>,
    num_ids: HashMap<usize, usize>, // id, index
    nav_index: usize,
    scroll_top: usize,
    scroll_height: usize,
}

impl<'a, L> Debug for GeneralMenu<'a, L>
where
    L: GeneralMenuName,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MenuState")
            .field("menu_level", &self.menu_level)
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

impl<'a, L> MenuState for GeneralMenu<'a, L>
where
    L: GeneralMenuName + 'static,
{
    fn menu_id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.menu_level.type_id().hash(&mut hasher);
        self.menu_level.hash(&mut hasher);
        hasher.finish()
    }

    fn save_state(&self) -> Box<dyn Any> {
        Box::new(self.nav_index)
    }

    #[tracing::instrument(skip_all)]
    fn on_create(&mut self, saved_state: Option<Box<dyn Any>>) {
        // println!("menu {} on_create {:?}", self.menu_id(), saved_state);
        if let Some(saved_state) = saved_state {
            if let Ok(data) = saved_state.downcast::<usize>() {
                self.nav_index = *data;
            }
        }
    }

    #[tracing::instrument(skip_all)]
    fn resize(&mut self, scroll_height: usize) {
        self.scroll_height = scroll_height - 4;
        if !(self.nav_index < self.scroll_top + self.scroll_height) {
            self.scroll_top = self.nav_index - self.scroll_height + 1;
        } else if !(self.nav_index >= self.scroll_top) {
            self.scroll_top = self.nav_index;
        }
    }

    #[tracing::instrument(skip_all)]
    fn nav_up(&mut self) {
        if self.nav_index > 0 {
            self.nav_index -= 1;
        }
    }

    #[tracing::instrument(skip_all)]
    fn nav_down(&mut self) {
        if self.nav_index < self.items.len() - 1 {
            self.nav_index += 1;
        }
    }

    #[tracing::instrument(skip_all)]
    fn nav_jump(&mut self, num: usize) {
        if let Some(&index) = self.num_ids.get(&num) {
            if index < self.items.len() {
                self.nav_index = index;
            }
        }
    }

    fn nav_mouse(&mut self, mouse: MouseEvent) {
        match mouse.kind {
            MouseEventKind::ScrollDown => {
                let items_row = self.items.len();
                let scroll_max = if items_row > self.scroll_height {
                    items_row - self.scroll_height
                } else {
                    0
                };
                if self.scroll_top < scroll_max && self.nav_index < items_row - 1 {
                    self.scroll_top += 1;
                    self.nav_index += 1;
                }
            }
            MouseEventKind::ScrollUp => {
                if self.scroll_top > 0 && self.nav_index > 0 {
                    self.scroll_top -= 1;
                    self.nav_index -= 1;
                }
            }
            MouseEventKind::Moved => {
                let max_index = self.items.len() - 1;
                let hover_con_row = mouse.row as usize;
                let hover_index = if hover_con_row < 3 {
                    None
                } else if hover_con_row + self.scroll_top >= 3
                    && hover_con_row - 3 + self.scroll_top <= max_index
                    && hover_con_row < 3 + self.scroll_height
                {
                    Some(hover_con_row - 3 + self.scroll_top)
                } else {
                    None
                };
                if let Some(nav_index) = hover_index {
                    self.nav_index = nav_index;
                }
            }
            _ => (),
        }
    }

    #[tracing::instrument(skip_all)]
    fn nav_click(&mut self, ctx: &mut TerminalMenu, _col: u16, row: u16) {
        let max_index = self.items.len() - 1;
        let hover_con_row = row as usize;
        let hover_index = if hover_con_row < 3 {
            None
        } else if hover_con_row + self.scroll_top >= 3
            && hover_con_row - 3 + self.scroll_top <= max_index
            && hover_con_row < 3 + self.scroll_height
        {
            Some(hover_con_row - 3 + self.scroll_top)
        } else {
            None
        };
        if hover_index.is_some() {
            self.nav_enter(ctx);
        }
    }

    #[tracing::instrument(skip_all)]
    fn nav_enter(&mut self, ctx: &mut TerminalMenu) {
        if let Some(f) = self.handler.get(&self.nav_index) {
            let ret = f.call(ctx);
            self.update_menu(ctx);
            if let Some(text) = ret {
                alert(ctx.app_model_mut(), text);
            }
        } else if let Some((prompt_text, f)) = self.input_handlers.get(&self.nav_index) {
            prompt(ctx.app_model_mut(), prompt_text.to_owned(), **f);
        }
    }

    #[tracing::instrument(skip_all)]
    fn update_menu(&mut self, ctx: &mut TerminalMenu) {
        if ctx.get_menu_level().is_some_and(|id| id != self.menu_id()) {
            return;
        }
        ctx.nav_menu(self.menu_level.clone().rebuild_state());
    }

    fn render(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(f.area());

        f.render_widget(block_title(self.title.clone()), chunks[0]);
        f.render_widget(
            render_selected_list(&self.items, self.nav_index, self.scroll_top),
            chunks[1],
        );
    }
}

pub struct MenuBuilder<'a, L>
where
    L: GeneralMenuName,
{
    menu_level: L,
    title: Cow<'a, str>,
    list_items: Vec<ListItem<'a>>,
    handlers: HashMap<usize, Arc<dyn CallbackTrait>>,
    input_handlers: HashMap<usize, (String, Box<fn(String) -> Option<String>>)>,
    num_ids: HashMap<usize, usize>,
    head_id: usize,
}

impl<'a, L> Debug for MenuBuilder<'a, L>
where
    L: GeneralMenuName,
{
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

impl<'a, L> MenuBuilder<'a, L>
where
    L: GeneralMenuName,
{
    pub fn new(menu_level: L) -> MenuBuilder<'a, L> {
        MenuBuilder {
            menu_level,
            title: std::borrow::Cow::Borrowed(""),
            list_items: Vec::new(),
            handlers: HashMap::new(),
            input_handlers: HashMap::new(),
            num_ids: HashMap::new(),
            head_id: 0,
        }
    }

    pub fn title<T>(mut self, value: T) -> MenuBuilder<'a, L>
    where
        T: Into<String>,
    {
        self.title = value.into().into();
        self
    }

    pub fn add_item<F, T>(mut self, item: ListItem<'a>, handler: F, state: T) -> MenuBuilder<'a, L>
    where
        F: FnOnce(&mut TerminalMenu, T) -> Option<String> + Clone + 'static,
        T: Clone + 'static,
    {
        let num = self.next_id();
        self.add_numbered_item(num, item, handler, state)
    }

    pub fn add_input_item(
        mut self,
        item: ListItem<'a>,
        prompt_text: &str,
        input_handler: fn(String) -> Option<String>,
    ) -> MenuBuilder<'a, L> {
        let num = self.next_id();
        self.add_numbered_input_item(num, item, prompt_text, input_handler)
    }

    pub fn next_id(&mut self) -> usize {
        loop {
            self.head_id += 1;
            if !self.num_ids.contains_key(&self.head_id) {
                break;
            }
        }
        self.head_id
    }

    pub fn skip_id(mut self) -> MenuBuilder<'a, L> {
        self.next_id();
        self
    }

    pub fn no_id(mut self) -> MenuBuilder<'a, L> {
        self.num_ids.remove_entry(&self.head_id);
        self.head_id -= 1;
        self
    }

    pub fn add_numbered_item<F, T>(
        mut self,
        num: usize,
        item: ListItem<'a>,
        handler: F,
        state: T,
    ) -> MenuBuilder<'a, L>
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

    pub fn add_numbered_input_item(
        mut self,
        num: usize,
        item: ListItem<'a>,
        prompt_text: &str,
        input_handler: fn(String) -> Option<String>,
    ) -> MenuBuilder<'a, L> {
        self.list_items.push(item);
        self.input_handlers.insert(
            self.list_items.len() - 1,
            (String::from(prompt_text), Box::new(input_handler)),
        );
        self.num_ids.insert(num, self.list_items.len() - 1);
        self
    }

    pub fn add_text_item<T>(mut self, label: T) -> MenuBuilder<'a, L>
    where
        T: Into<String>,
    {
        self.list_items.push(item_text(label));
        self
    }

    pub fn add_dummy_item(mut self) -> MenuBuilder<'a, L> {
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

impl<'a, L> From<MenuBuilder<'a, L>> for GeneralMenu<'a, L>
where
    L: GeneralMenuName,
{
    fn from(val: MenuBuilder<'a, L>) -> Self {
        GeneralMenu {
            menu_level: val.menu_level,
            title: val.title,
            items: val.list_items,
            handler: val.handlers,
            input_handlers: val.input_handlers,
            num_ids: val.num_ids,
            nav_index: 0,
            scroll_top: 0,
            scroll_height: 0,
        }
    }
}

fn render_selected_list<'a>(
    list_items: &'a [ListItem<'a>],
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

pub fn format_label<T>(label: T) -> Span<'static>
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
pub fn format_item<'a, T>(i18n_bundle: &I18nBundle, label: T, value: Span<'a>) -> ListItem<'a>
where
    T: Into<String>,
{
    ListItem::new(Line::from(vec![
        format_label(label.into()),
        Span::styled(
            i18n_bundle.msg(MessageId::MenuValuePrefix).to_string(),
            Style::default().fg(Color::DarkGray),
        ),
        value,
        Span::styled(
            i18n_bundle.msg(MessageId::MenuValueSuffix).to_string(),
            Style::default().fg(Color::DarkGray),
        ),
    ]))
}
fn span_enabled(i18n_bundle: &I18nBundle, v: bool) -> Span<'static> {
    if v {
        Span::styled(
            i18n_bundle.msg(MessageId::MenuValueEnabled).to_string(),
            Style::default().fg(Color::Green),
        )
    } else {
        Span::from(i18n_bundle.msg(MessageId::MenuValueDisabled).to_string())
    }
}
pub fn item_enabled<T>(i18n_bundle: &I18nBundle, label: T, v: bool) -> ListItem<'static>
where
    T: Into<String>,
{
    format_item(i18n_bundle, label, span_enabled(i18n_bundle, v))
}
pub fn item_text<T>(label: T) -> ListItem<'static>
where
    T: Into<String>,
{
    ListItem::new(Line::from(format_label(label)))
}
pub fn item_dummy() -> ListItem<'static> {
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
