use std::{any::Any, collections::HashMap, fmt::Debug};

use crossterm::event::{KeyCode, MouseButton, MouseEvent, MouseEventKind};
use ratatui::Frame;

use crate::PauseMenu;

use super::RunningState;

pub struct TerminalMenu {
    app_model: super::Model,
    menu_stack: Vec<Box<dyn MenuState>>,
    current: Option<Box<dyn MenuState>>,
    saved_state: HashMap<u64, Box<dyn Any>>,
    mouse_down: Option<(u16, u16)>, // (column, row)
    width: usize,
    height: usize,
}

pub trait MenuState: Debug {
    /// Unique menu level id
    fn menu_id(&self) -> u64;

    /// Save state here
    fn save_state(&self) -> Box<dyn Any> {
        Box::new(())
    }

    /// handler: create
    ///
    /// saved_state: state saved by `save_state()` function
    ///
    /// This function is called when the menu is created.
    ///
    /// e.g. when the menu is navigated to.
    fn on_create(&mut self, _saved_state: Option<Box<dyn Any>>) {}

    /// handler: resize
    ///
    /// width: width of the frame area, in characters
    /// height: height of the frame area, in lines
    ///
    /// This function is called when the frame is resized or after the menu is created.
    ///
    /// e.g. when the terminal is resized or after the menu is navigated to.
    fn on_resize(&mut self, _width: usize, _height: usize) {}

    /// handler: navigate up
    fn on_nav_up(&mut self) {}

    /// handler: navigate down
    fn on_nav_down(&mut self) {}

    /// handler: navigate jump to
    fn on_nav_jump(&mut self, _num: usize) {}

    /// handler: mouse event
    ///
    /// return false to prevent default action
    fn on_mouse_event(&mut self, _mouse_event: MouseEvent) -> bool {
        true
    }

    /// handler: click
    fn on_click(&mut self, _ctx: &mut TerminalMenu, _col: u16, _row: u16) {}

    /// handler: keydown
    ///
    /// return false to prevent default action
    fn on_key_press(&mut self, _ctx: &mut TerminalMenu, _key: KeyCode) -> bool {
        true
    }

    /// handler: navigate enter
    fn on_nav_enter(&mut self, _ctx: &mut TerminalMenu) {}

    /// handler: navigate back
    fn on_nav_back(&mut self) -> bool {
        true
    }

    /// handler: redraw
    fn update_menu(&mut self, _ctx: &mut TerminalMenu) {}

    /// render function
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
            width: 0,
            height: 0,
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

    /// Resize content to frame size
    pub fn resize(&mut self, f: &mut Frame) {
        self.width = f.area().width.into();
        self.height = f.area().height.into();
        if let Some(state) = &mut self.current {
            state.on_resize(self.width, self.height);
        }
    }

    /// Render a frame
    pub fn render(&mut self, f: &mut Frame) {
        self.render_menu(f);
    }

    #[tracing::instrument(skip_all)]
    pub fn nav_up(&mut self) {
        if let Some(state) = &mut self.current {
            state.on_nav_up();
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn nav_down(&mut self) {
        if let Some(state) = &mut self.current {
            state.on_nav_down();
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn nav_jump(&mut self, num: usize) {
        if let Some(state) = &mut self.current {
            state.on_nav_jump(num);
        }
    }

    pub fn nav_mouse(&mut self, mouse_event: MouseEvent) {
        if self
            .current
            .as_mut()
            .is_some_and(|state| !state.on_mouse_event(mouse_event))
        {
            return;
        }

        if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
            self.mouse_down = Some((mouse_event.column, mouse_event.row));
        } else if mouse_event.kind == MouseEventKind::Up(MouseButton::Left) {
            if let Some((col, row)) = self.mouse_down {
                if mouse_event.column == col && mouse_event.row == row {
                    let Some(mut menu_state) = self.take_current() else {
                        return;
                    };
                    menu_state.on_click(self, col, row);
                    self.revert_current(menu_state);
                    return;
                }
            }
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn nav_back(&mut self) {
        if self.current.as_mut().is_some_and(|m| !m.on_nav_back()) {
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

        let default_action = menu_state.on_key_press(self, key);

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
                    let now = super::time_ms();
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

        menu_state.on_nav_enter(self);

        self.revert_current(menu_state);
    }

    /// Navigating to a new menu level
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
        new_menu_state.on_resize(self.width, self.height);

        // Make sure there are no duplicate menus on the stack
        self.menu_stack.retain(|x| x.menu_id() != menu_id);

        // Update menu state
        self.current = Some(new_menu_state);
    }

    /// Redraw menu content
    #[tracing::instrument(skip_all)]
    pub fn update_menu(&mut self) {
        let Some(mut menu_state) = self.take_current() else {
            return;
        };

        menu_state.update_menu(self);

        self.revert_current(menu_state);
    }

    /// Set menu status to pause.
    ///
    /// After this round of rendering and event processing the menu will
    /// stop rendering and restore the terminal, then wait for resume.
    ///
    /// ```no_run
    /// // Set the current menu to pause
    /// let pause = ctx.pause_menu();
    ///
    /// // Do not block the handler from returning
    /// std::thread::spawn(move || {
    ///     pause.wait_pause();
    ///     println!("menu paused");
    ///     std::thread::sleep(std::time::Duration::from_secs(2));
    ///     pause.resume();
    /// });
    /// ```
    pub fn pause_menu(&mut self) -> PauseMenu {
        PauseMenu::new_pause(self)
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub(super) fn get_menu_level(&self) -> Option<u64> {
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
