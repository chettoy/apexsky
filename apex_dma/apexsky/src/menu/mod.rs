mod apex_menu;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::*, widgets::*};
use std::time::{Duration, SystemTime};
use unicode_width::UnicodeWidthChar;

use crate::global_state::G_STATE;

use self::apex_menu::TerminalMenu;

// ANCHOR: model
#[derive(Debug)]
pub struct Model {
    counter: i32,
    running_state: RunningState,
    key_input: String,
    input_callback: Option<fn(String) -> Option<String>>,
    input_buf: String,
    dialog_text: String,
    jump_buf: String,
    jump_time: u128,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Editing,
    Dialog,
    Done,
}
// ANCHOR_END: model

// ANCHOR: message
#[derive(PartialEq)]
enum Message {
    Up,
    Down,
    Back,
    Enter,
    Jump(u32),
    Reset,
    Quit,
    Input(char),
    Delete,
}
// ANCHOR_END: message

// ANCHOR: main
pub(crate) fn main() -> anyhow::Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut apex_menu = apex_menu::TerminalMenu::new(Model {
        counter: 0,
        running_state: RunningState::default(),
        key_input: String::new(),
        input_callback: None,
        input_buf: String::new(),
        dialog_text: String::new(),
        jump_buf: String::new(),
        jump_time: 0,
    });

    G_STATE.lock().unwrap().terminal_t = true;
    while apex_menu.app_model().running_state != RunningState::Done {
        let (tui_t, forceupdate) = {
            let g_state = G_STATE.lock().unwrap();
            (g_state.terminal_t, g_state.tui_forceupdate)
        };
        if !tui_t {
            apex_menu.app_model_mut().running_state = RunningState::Done;
            break;
        }
        if forceupdate {
            apex_menu.update_menu();
            G_STATE.lock().unwrap().tui_forceupdate = false;
        }

        // Render the current view
        terminal.draw(|f| view(&mut apex_menu, f))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(apex_menu.app_model())?;

        // Process updates as long as they return a non-‌​‌‌​​​‌‌‌‍‌​‌‌​‌​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌​‌‌‌‌‌‍‌​‌‌‌‌​​‌‌‍‌​‌‌​​​​‌‌‍‌​‌‌‌​​​​‌‍‌​‌‌​​‌​‌‌‍‌​‌‌‌‌​​​‌‍‌​‌‌‌​‌​​‌‍‌​‌‌‌​‌​‌‌‍‌​‌‌​‌​​‌‌‍‌​‌‌​‌‌​‌‌‍‌​‌‌​​‌​‌‌‍‌​‌‌​‌‌‌​‌‍‌​‌‌‌​‌​‌‌None message
        while current_msg != None {
            current_msg = update(&mut apex_menu, current_msg.unwrap());
        }
    }

    tui::restore_terminal()?;
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
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                if model.running_state == RunningState::Editing {
                    return Ok(handle_editing(key));
                } else {
                    return Ok(handle_key(key));
                }
            }
        }
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Up => Some(Message::Up),
        KeyCode::Down => Some(Message::Down),
        KeyCode::Tab => Some(Message::Down),
        KeyCode::Enter => Some(Message::Enter),
        KeyCode::Esc => Some(Message::Back),
        KeyCode::Char('h') => Some(Message::Back),
        KeyCode::Char('j') => Some(Message::Down),
        KeyCode::Char('k') => Some(Message::Up),
        KeyCode::Char('l') => Some(Message::Enter),
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Char(ch) => ch.to_digit(10).and_then(|c| Some(Message::Jump(c))),
        _ => None,
    }
}

fn handle_editing(key: event::KeyEvent) -> Option<Message> {
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
fn update(apex_menu: &mut TerminalMenu, msg: Message) -> Option<Message> {
    let model = apex_menu.app_model_mut();
    match msg {
        Message::Reset => model.counter = 0,
        Message::Quit => {
            // You can handle cleanup and exit here
            model.running_state = RunningState::Done;
        }
        Message::Up => apex_menu.nav_up(),
        Message::Down => apex_menu.nav_down(),
        Message::Back => match model.running_state {
            RunningState::Editing => {
                model.running_state = RunningState::Running;
                model.input_buf.clear();
                model.input_callback = None;
            }
            RunningState::Dialog => {
                model.running_state = RunningState::Running;
                model.dialog_text.clear();
            }
            RunningState::Running => apex_menu.nav_back(),
            RunningState::Done => (),
        },
        Message::Enter => match model.running_state {
            RunningState::Editing => {
                model.running_state = RunningState::Running;
                if let Some(callback) = model.input_callback {
                    let result = callback(model.input_buf.clone());
                    if let Some(text) = result {
                        alert(model, text);
                    }
                }
                model.input_buf.clear();
                model.input_callback = None;
                apex_menu.update_menu();
            }
            RunningState::Dialog => {
                model.running_state = RunningState::Running;
                model.dialog_text.clear();
            }
            RunningState::Running => apex_menu.nav_enter(),
            RunningState::Done => (),
        },
        Message::Jump(i) => {
            let now = time();
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
            apex_menu.nav_jump(num);
        }
        Message::Input(c) => {
            model.input_buf.insert(model.input_buf.len(), c);
        }
        Message::Delete => {
            if model.input_buf.len() > 0 {
                model.input_buf.remove(model.input_buf.len() - 1);
            }
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

pub fn prompt(model: &mut Model, key_input: String, callback: fn(String) -> Option<String>) {
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

fn editing_render(f: &mut Frame, key_input: &String, value_input: &String) {
    let popup_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let frame_size = f.size();
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

    let key_text = Paragraph::new(key_input.clone()).block(key_block);
    f.render_widget(key_text, popup_chunks[0]);

    let value_text = Paragraph::new(value_input.clone()).block(value_block);
    f.render_widget(value_text, popup_chunks[1]);
}

fn dialog_render(f: &mut Frame, dialog_text: &String) {
    let popup_block = Block::default()
        .title("Dialog")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::White));

    let frame_size = f.size();
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

fn time() -> u128 {
    SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

mod tui {
    use crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    };
    use ratatui::prelude::*;
    use std::{io::stdout, panic};

    pub fn init_terminal() -> anyhow::Result<Terminal<impl Backend>> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(terminal)
    }

    pub fn restore_terminal() -> anyhow::Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn install_panic_hook() {
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            stdout().execute(LeaveAlternateScreen).unwrap();
            disable_raw_mode().unwrap();
            original_hook(panic_info);
        }));
    }
}
