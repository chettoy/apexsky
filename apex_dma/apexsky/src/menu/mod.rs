mod apex_menu;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::*, widgets::*};
use std::time::Duration;

// ANCHOR: model
#[derive(Debug)]
struct Model<'a> {
    counter: i32,
    running_state: RunningState,
    apex_menu: apex_menu::TerminalMenu<'a>,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Done,
}
// ANCHOR_END: model

// ANCHOR: message
#[derive(PartialEq)]
enum Message {
    Increment,
    Decrement,
    Up,
    Down,
    Back,
    Enter,
    Reset,
    Quit,
}
// ANCHOR_END: message

// ANCHOR: main
fn main() -> anyhow::Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let test_data = crate::config::Config::default();
    let mut model = Model{ counter: 0, running_state: RunningState::default(), apex_menu: apex_menu::TerminalMenu::from(&test_data) };

    while model.running_state != RunningState::Done {
        // Render the current view
        terminal.draw(|f| view(&mut model, f))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(&model)?;

        // Process updates as long as they return a non-None message
        while current_msg != None {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    tui::restore_terminal()?;
    Ok(())
}
// ANCHOR_END: main

// ANCHOR: view
fn view(model: &mut Model, f: &mut Frame) {
    model.apex_menu.render(f);

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
fn handle_event(_: &Model) -> anyhow::Result<Option<Message>> {
    if event::poll(Duration::from_millis(20))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
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
        _ => None,
    }
}

// ANCHOR: update
fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Increment => {
            model.counter += 1;
            if model.counter > 50 {
                return Some(Message::Reset);
            }
        }
        Message::Decrement => {
            model.counter -= 1;
            if model.counter < -50 {
                return Some(Message::Reset);
            }
        }
        Message::Reset => model.counter = 0,
        Message::Quit => {
            // You can handle cleanup and exit here
            model.running_state = RunningState::Done;
        }
        Message::Up => model.apex_menu.nav_up(),
        Message::Down => model.apex_menu.nav_down(),
        Message::Back => model.apex_menu.nav_back(),
        Message::Enter => model.apex_menu.nav_enter(),
        _ => (),
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

fn editing(f: &mut Frame, key_input: &String, value_input: &String) {
    let popup_block = Block::default()
        .title("Enter a new value")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let area = centered_rect(60, 25, f.size());
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

    // let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

    // match editing {
    //     CurrentlyEditing::Key => key_block = key_block.style(active_style),
    //     CurrentlyEditing::Value => value_block = value_block.style(active_style),
    // };

    let key_text = Paragraph::new(key_input.clone()).block(key_block);
    f.render_widget(key_text, popup_chunks[0]);

    let value_text = Paragraph::new(value_input.clone()).block(value_block);
    f.render_widget(value_text, popup_chunks[1]);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tui() {
        main().unwrap();
    }
}
