mod callback;
pub mod general_menu;
pub mod state;

use std::{
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, SystemTime},
};

use callback::IInputCallback;
use crossterm::event::{self, Event, KeyCode, MouseEvent};
use ratatui::{prelude::*, widgets::*};
use unicode_width::UnicodeWidthChar;

pub use self::state::{MenuState, TerminalMenu};
pub use ratatui;
pub use unicode_width;

// ANCHOR: model
#[derive(Debug)]
pub struct Model {
    running_state: RunningState,
    pause: Arc<(Mutex<bool>, Condvar)>,
    key_input: String,
    input_callback: Option<Arc<dyn IInputCallback>>,
    input_buf: String,
    dialog_text: String,
    jump_buf: String,
    jump_time: u128,
}

pub struct PauseMenu {
    pause: Arc<(Mutex<bool>, Condvar)>,
}

impl PauseMenu {
    /// Set menu status to pause and return a MenuResumer
    pub fn new_pause(ctx: &mut TerminalMenu) -> Self {
        ctx.app_model_mut().running_state = RunningState::Pause;
        Self {
            pause: Arc::clone(&ctx.app_model().pause),
        }
    }

    /// Wait for the menu to pause
    ///
    /// Don't call it in the menu thread or it will cause a deadlock!
    pub fn wait_pause(&self) {
        let (lock, cvar) = &*self.pause;
        let mut paused = lock.lock().unwrap();
        while !*paused {
            paused = cvar.wait(paused).unwrap();
        }
    }

    /// Resume from pause and continue rendering
    pub fn resume(&self) {
        let (lock, cvar) = &*self.pause;
        *lock.lock().unwrap() = false;
        cvar.notify_all();
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Editing,
    Dialog,
    Pause,
    Done,
}
// ANCHOR_END: model

// ANCHOR: message
#[derive(PartialEq)]
enum Message {
    Back,
    Enter,
    Mouse(MouseEvent),
    Press(KeyCode),
    Input(char),
    Delete,
}
// ANCHOR_END: message

// ANCHOR: main
pub fn main(
    default_memu: Box<dyn MenuState>,
    quit: &AtomicBool,
    force_update: &AtomicBool,
) -> anyhow::Result<()> {
    tui::install_panic_hook();

    let mut apex_menu = TerminalMenu::new(
        Model {
            running_state: RunningState::default(),
            pause: Arc::new((Mutex::new(false), Condvar::new())),
            key_input: String::new(),
            input_callback: None,
            input_buf: String::new(),
            dialog_text: String::new(),
            jump_buf: String::new(),
            jump_time: 0,
        },
        default_memu,
    );

    while apex_menu.app_model().running_state != RunningState::Done {
        let mut terminal = tui::init_terminal()?;

        while !matches!(
            apex_menu.app_model().running_state,
            RunningState::Pause | RunningState::Done
        ) {
            if quit.load(Ordering::Relaxed) {
                apex_menu.app_model_mut().running_state = RunningState::Done;
                break;
            }
            if force_update.load(Ordering::SeqCst) {
                apex_menu.update_menu();
                force_update.store(false, Ordering::Release);
            }

            // Render the current view
            terminal.draw(|f| view(&mut apex_menu, f))?;

            // Handle events and map to a Message
            let mut current_msg = handle_event(apex_menu.app_model())?;

            // Process updates as long as they return a non-‌​‌‌​​​‌‌‌‍‌​‌‌​‌​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌​‌‌‌‌‌‍‌​‌‌‌‌​​‌‌‍‌​‌‌​​​​‌‌‍‌​‌‌‌​​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌‌​​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​‌‌‍‌​‌‌​‌​​‌‌‍‌​‌‌​‌‌​‌‌‍‌​‌‌​​‌​‌‌‍‌​‌‌​‌‌‌​‌‍‌​‌‌‌​‌​‌‌None message
            while current_msg.is_some() {
                current_msg = update(&mut apex_menu, current_msg.unwrap());
            }
        }
        tui::restore_terminal(&mut terminal)?;

        let should_pause = apex_menu.app_model().running_state == RunningState::Pause;

        if should_pause {
            {
                let (lock, cvar) = &*apex_menu.app_model().pause;
                let mut paused = lock.lock().unwrap();

                // Notify of pause
                *paused = true;
                cvar.notify_all();

                // Wait for resume
                while *paused {
                    paused = cvar.wait(paused).unwrap();
                }
            }
            // Resume running state
            apex_menu.app_model_mut().running_state = RunningState::Running;
        }
    }

    Ok(())
}
// ANCHOR_END: main

// ANCHOR: view
fn view(apex_menu: &mut TerminalMenu, f: &mut Frame) {
    let model = apex_menu.app_model();
    match model.running_state {
        RunningState::Editing => editing_render(f, &model.key_input, &model.input_buf),
        RunningState::Dialog => dialog_render(f, &model.dialog_text),
        RunningState::Running => {
            apex_menu.resize(f);
            apex_menu.render(f);
        }
        RunningState::Pause => (),
        RunningState::Done => (),
    }

    // f.render_widget(
    //     Paragraph::new(format!("Counter: {}", model.counter)),
    //     f.size(),
    // );
}
// ANCHOR_END: view

/// Convert Event to Message
///
/// We don't need to pass in a `model` to this function in this example
/// but you might need it as your project evolves
fn handle_event(model: &Model) -> anyhow::Result<Option<Message>> {
    if event::poll(Duration::from_millis(20))? {
        let ev = event::read()?;
        if let Event::Key(key) = ev {
            if key.kind == event::KeyEventKind::Press {
                if model.running_state == RunningState::Running {
                    return Ok(Some(Message::Press(key.code)));
                } else {
                    return Ok(handle_dialog_or_edit(key));
                }
            }
        } else if let Event::Mouse(mouse) = ev {
            return Ok(Some(Message::Mouse(mouse)));
        }
    }
    Ok(None)
}

fn handle_dialog_or_edit(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Enter => Some(Message::Enter),
        KeyCode::Esc => Some(Message::Back),
        KeyCode::Char(c) => Some(Message::Input(c)),
        KeyCode::Backspace => Some(Message::Delete),
        KeyCode::Delete => Some(Message::Delete),
        _ => None,
    }
}

// ANCHOR: update
fn update(ctx: &mut TerminalMenu, msg: Message) -> Option<Message> {
    match msg {
        Message::Back => match ctx.app_model_mut().running_state {
            RunningState::Editing => {
                ctx.app_model_mut().running_state = RunningState::Running;
                ctx.app_model_mut().input_buf.clear();
                ctx.app_model_mut().input_callback = None;
            }
            RunningState::Dialog => {
                ctx.app_model_mut().running_state = RunningState::Running;
                ctx.app_model_mut().dialog_text.clear();
            }
            _ => (),
        },
        Message::Enter => match ctx.app_model_mut().running_state {
            RunningState::Editing => {
                ctx.app_model_mut().running_state = RunningState::Running;
                if let Some(callback) = ctx.app_model_mut().input_callback.clone() {
                    let input = ctx.app_model_mut().input_buf.clone();
                    let result = callback.call(ctx, input);
                    if let Some(text) = result {
                        alert(ctx.app_model_mut(), text);
                    }
                }
                ctx.app_model_mut().input_buf.clear();
                ctx.app_model_mut().input_callback = None;
                ctx.update_menu();
            }
            RunningState::Dialog => {
                ctx.app_model_mut().running_state = RunningState::Running;
                ctx.app_model_mut().dialog_text.clear();
            }
            _ => (),
        },
        Message::Mouse(mouse) => {
            if ctx.app_model_mut().running_state == RunningState::Running {
                ctx.nav_mouse(mouse);
            }
        }
        Message::Press(keycode) => {
            ctx.nav_press(keycode);
        }
        Message::Input(ch) => {
            ctx.app_model_mut().input_buf.push(ch);
        }
        Message::Delete => {
            ctx.app_model_mut().input_buf.pop();
        }
    };
    None
}
// ANCHOR_END: update

// ANCHOR: centered_rect
/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
// ANCHOR_END: centered_rect

pub fn prompt(model: &mut Model, key_input: String, callback: Arc<dyn IInputCallback>) {
    if model.running_state == RunningState::Editing {
        return;
    }
    model.key_input = key_input;
    model.input_callback = Some(callback);
    model.running_state = RunningState::Editing;
}

pub fn alert(model: &mut Model, dialog_text: String) {
    if model.running_state == RunningState::Dialog {
        return;
    }
    model.dialog_text = dialog_text;
    model.running_state = RunningState::Dialog;
}

fn editing_render(f: &mut Frame, key_input: &str, value_input: &str) {
    let popup_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let frame_size = f.area();
    let graph_width = frame_size.width as f32 / 2.0;
    let graph_height = frame_size.height as f32;
    let ratio = if graph_width > graph_height {
        graph_height / graph_width
    } else {
        graph_width / graph_height
    };
    let area = centered_rect(
        (100.0 * ratio).round() as u16,
        (100.0 * ratio / 1.618 * graph_width / graph_height).round() as u16,
        frame_size,
    );
    f.render_widget(popup_block, area);
    // ANCHOR_END: editing_popup

    // ANCHOR: popup_layout
    let popup_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    // ANCHOR_END: popup_layout

    // ANCHOR: key_value_blocks
    let key_block = Block::default().title("Key").borders(Borders::ALL);
    let value_block = Block::default().title("Value").borders(Borders::ALL);

    let key_text = Paragraph::new(key_input).block(key_block);
    f.render_widget(key_text, popup_chunks[0]);

    let value_text = Paragraph::new(value_input).block(value_block);
    f.render_widget(value_text, popup_chunks[1]);
}

fn dialog_render(f: &mut Frame, dialog_text: &str) {
    let popup_block = Block::default()
        .title("Dialog")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::White));

    let frame_size = f.area();
    let graph_width = frame_size.width as f32 / 2.0;
    let graph_height = frame_size.height as f32;
    let ratio = if graph_width > graph_height {
        graph_height / graph_width
    } else {
        graph_width / graph_height
    };
    let area = centered_rect(
        (80.0 * ratio).round() as u16,
        (80.0 * ratio / 1.618 * graph_width / graph_height).round() as u16,
        frame_size,
    );
    f.render_widget(popup_block, area);
    let text_block = Block::default().borders(Borders::ALL);

    let mut text_buf = String::new();
    let mut count_in_line: usize = 0;
    let chars: Vec<char> = dialog_text.chars().collect();
    for ch in chars {
        text_buf.insert(text_buf.len(), ch);
        if ch != '\n' {
            count_in_line += UnicodeWidthChar::width(ch).unwrap_or(1);
        } else {
            count_in_line = 0;
        }
        if count_in_line > (area.width as usize) - 3 {
            text_buf.insert(text_buf.len(), '\n');
            count_in_line = 0;
        }
    }
    let text = Paragraph::new(text_buf).fg(Color::Black).block(text_block);
    f.render_widget(text, area);
}

fn time_ms() -> u128 {
    SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub mod tui {
    use crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    };
    use ratatui::prelude::*;
    use std::{
        io::{Stdout, stdout},
        panic,
    };

    pub fn init_terminal() -> anyhow::Result<Terminal<CrosstermBackend<Stdout>>> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(terminal)
    }

    pub fn restore_terminal(
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> anyhow::Result<()> {
        disable_raw_mode()?;
        crossterm::execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        Ok(())
    }

    pub fn install_panic_hook() {
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            disable_raw_mode().unwrap();
            crossterm::execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
            original_hook(panic_info);
        }));
    }
}
