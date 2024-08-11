use crossterm::event::{KeyCode, MouseButton, MouseEvent, MouseEventKind};
pub use ratatui;
use ratatui::Frame;
use std::{any::Any, collections::HashMap, fmt::Debug};
pub use unicode_width;

use super::{alert, prompt, RunningState};

pub mod general_menu;

pub struct TerminalMenu {
    app_model: super::Model,
    menu_stack: Vec<Box<dyn MenuState>>,
    current: Option<Box<dyn MenuState>>,
    saved_state: HashMap<u64, Box<dyn Any>>,
    mouse_down: Option<(u16, u16)>,
}

pub trait MenuState: Debug {
    fn menu_id(&self) -> u64;
    fn save_state(&self) -> Box<dyn Any> {
        Box::new(())
    }
    fn on_create(&mut self, _saved_state: Option<Box<dyn Any>>) {}
    fn resize(&mut self, _scroll_height: usize) {}
    fn nav_up(&mut self) {}
    fn nav_down(&mut self) {}
    fn nav_jump(&mut self, _num: usize) {}
    fn nav_mouse(&mut self, _mouse: MouseEvent) {}
    fn nav_click(&mut self, _ctx: &mut TerminalMenu, _col: u16, _row: u16) {}
    fn nav_press(&mut self, _ctx: &mut TerminalMenu, _key: KeyCode) -> bool {
        true
    }
    fn nav_enter(&mut self, _ctx: &mut TerminalMenu) {}
    fn nav_back(&mut self) -> bool {
        true
    }
    fn update_menu(&mut self, _ctx: &mut TerminalMenu) {}
    fn render(&mut self, f: &mut Frame);
}

impl TerminalMenu {
    pub fn new<M>(app_model: super::Model, default_menu: M) -> Self
    where
        M: Into<Box<dyn MenuState>>,
    {
        let mut instance = Self {
            app_model,
            menu_stack: Vec::new(),
            current: None,
            saved_state: HashMap::new(),
            mouse_down: None,
        };
        instance.nav_menu(default_menu);
        instance
    }

    pub(crate) fn app_model(&self) -> &super::Model {
        &self.app_model
    }

    pub fn app_model_mut(&mut self) -> &mut super::Model {
        &mut self.app_model
    }

    pub fn resize(&mut self, f: &mut Frame) {
        let scroll_height = f.area().height.into();
        if let Some(state) = &mut self.current {
            state.resize(scroll_height);
        }
    }

    pub fn render(&mut self, f: &mut Frame) {
        self.render_menu(f);
    }

    #[tracing::instrument(skip_all)]
    pub fn nav_up(&mut self) {
        if let Some(state) = &mut self.current {
            state.nav_up();
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn nav_down(&mut self) {
        if let Some(state) = &mut self.current {
            state.nav_down();
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn nav_jump(&mut self, num: usize) {
        if let Some(state) = &mut self.current {
            state.nav_jump(num);
        }
    }

    pub fn nav_mouse(&mut self, mouse: MouseEvent) {
        if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
            self.mouse_down = Some((mouse.column, mouse.row));
        } else if mouse.kind == MouseEventKind::Up(MouseButton::Left) {
            if let Some((col, row)) = self.mouse_down {
                if mouse.column == col && mouse.row == row {
                    let Some(mut menu_state) = self.take_current() else {
                        return;
                    };
                    menu_state.nav_click(self, col, row);
                    self.revert_current(menu_state);
                    return;
                }
            }
        }
        if let Some(state) = &mut self.current {
            state.nav_mouse(mouse);
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn nav_back(&mut self) {
        if self.current.as_mut().is_some_and(|m| !m.nav_back()) {
            return;
        }

        if self.menu_stack.is_empty() {
            return;
        }
        if let Some(current) = self.current.take() {
            self.saved_state.remove(&current.menu_id());
        }
        if let Some(menu) = self.menu_stack.pop() {
            self.nav_menu(menu);
        }
        self.update_menu();
    }

    #[tracing::instrument(skip_all)]
    pub fn nav_press(&mut self, key: KeyCode) {
        let Some(mut menu_state) = self.take_current() else {
            return;
        };

        let default_action = menu_state.nav_press(self, key);

        self.revert_current(menu_state);

        if !default_action {
            return;
        }
        match key {
            KeyCode::Up => self.nav_up(),
            KeyCode::Down => self.nav_down(),
            KeyCode::Tab => self.nav_down(),
            KeyCode::Enter => self.nav_enter(),
            KeyCode::Esc => self.nav_back(),
            KeyCode::Char('r') => self.update_menu(),
            KeyCode::Char('h') => self.nav_back(),
            KeyCode::Char('j') => self.nav_down(),
            KeyCode::Char('k') => self.nav_up(),
            KeyCode::Char('l') => self.nav_enter(),
            KeyCode::Char('q') => self.app_model.running_state = RunningState::Done,
            KeyCode::Char(ch) => {
                if let Some(i) = ch.to_digit(10) {
                    let model = &mut self.app_model;
                    let now = super::time();
                    if now - model.jump_time > 600 {
                        model.jump_buf.clear();
                    }
                    model.jump_time = now;
                    if model.jump_buf.len() + 1 > 2 {
                        model.jump_buf.clear();
                    }
                    model
                        .jump_buf
                        .insert(model.jump_buf.len(), char::from_digit(i, 10).unwrap());
                    let num = model.jump_buf.parse::<usize>().unwrap();
                    self.nav_jump(num);
                }
            }
            _ => (),
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn nav_enter(&mut self) {
        let Some(mut menu_state) = self.take_current() else {
            return;
        };

        menu_state.nav_enter(self);

        self.revert_current(menu_state);
    }

    #[tracing::instrument(skip_all)]
    pub fn nav_menu<M>(&mut self, menu: M)
    where
        M: Into<Box<dyn MenuState>>,
    {
        // Save state for current menu
        let _ = self.take_current();

        // Get menu id
        let menu = menu.into();
        let menu_id = menu.menu_id();

        // Restore state for new menu
        let mut new_menu_state = menu;
        let saved_state = self.saved_state.remove(&menu_id);
        new_menu_state.on_create(saved_state);

        // Make sure there are no duplicate menus on the stack
        self.menu_stack.retain(|x| x.menu_id() != menu_id);

        // Update menu state
        self.current = Some(new_menu_state);
    }

    #[tracing::instrument(skip_all)]
    pub fn update_menu(&mut self) {
        let Some(mut menu_state) = self.take_current() else {
            return;
        };

        menu_state.update_menu(self);

        self.revert_current(menu_state);
    }

    fn get_menu_level(&self) -> Option<u64> {
        Some(self.current.as_ref()?.menu_id())
    }

    fn render_menu(&mut self, f: &mut Frame) {
        let Some(menu_state) = self.current.as_mut() else {
            return;
        };
        menu_state.render(f);
    }

    /// Take out the current menu and save the state
    #[tracing::instrument(skip_all)]
    fn take_current(&mut self) -> Option<Box<dyn MenuState>> {
        let menu_state = self.current.take()?;
        self.saved_state
            .insert(menu_state.menu_id(), menu_state.save_state());
        Some(menu_state)
    }

    /// Put back the menu took out
    #[tracing::instrument(skip_all)]
    fn revert_current(&mut self, menu_state: Box<dyn MenuState>) {
        if self
            .current
            .as_ref()
            .is_some_and(|new| new.menu_id() != menu_state.menu_id())
        {
            self.menu_stack.push(menu_state);
        } else {
            let _ = self.current.get_or_insert(menu_state);
        }
    }
}
