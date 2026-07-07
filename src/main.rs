use std::{env, io, process::exit};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Terminal, crossterm,
    crossterm::{
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::CrosstermBackend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget},
};

use crate::history::History;

pub mod history;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    //first arg is the binary name, I don't care about it so i remove it
    args.remove(0);
    if args.is_empty() {
        exit(0)
    }

    let first_arg = args.first().cloned().unwrap_or("".to_string());
    match first_arg.as_str() {
        "--ui" => {
            let history = History::get();
            let mut history = match history {
                Ok(history) => history,
                Err(e) => {
                    eprintln!("could not get history: {}", e);
                    exit(1)
                }
            };

            match color_eyre::install() {
                Ok(_) => (),
                Err(_) => exit(1),
            }

            if enable_raw_mode().is_err() || execute!(io::stderr(), EnterAlternateScreen).is_err() {
                exit(1)
            }
            let mut terminal = match Terminal::new(CrosstermBackend::new(io::stderr())) {
                Ok(terminal) => terminal,
                Err(_) => {
                    let _ = disable_raw_mode();
                    let _ = execute!(io::stderr(), LeaveAlternateScreen);
                    exit(1)
                }
            };

            let mut all_sessions: Vec<String> = history.last_sessions.clone();
            if let Some(current_session) = history.current_session.clone() {
                all_sessions.push(current_session);
            }
            all_sessions.extend(history.next_sessions.clone());

            let state = &mut ListState::default();
            state.select(Some(history.last_sessions.len()));
            let mut chosen_session: Option<String> = history.current_session.clone();

            loop {
                _ = terminal.draw(|frame| {
                    let sessions_items: Vec<ListItem> =
                        all_sessions.iter().cloned().map(ListItem::new).collect();

                    let list = List::new(sessions_items)
                        .block(Block::default().title("Select session"))
                        .highlight_style(
                            Style::default()
                                .bg(Color::Blue)
                                .fg(Color::Black)
                                .add_modifier(Modifier::BOLD),
                        )
                        .highlight_symbol("▶ ");

                    StatefulWidget::render(list, frame.area(), frame.buffer_mut(), state);
                });

                match event::read().unwrap() {
                    Event::FocusGained => (),
                    Event::FocusLost => (),
                    Event::Key(key) => match key.code {
                        KeyCode::Enter => {
                            chosen_session =
                                state.selected().and_then(|i| all_sessions.get(i).cloned());
                            break;
                        }
                        KeyCode::Char(char) => match char {
                            'q' => break,
                            'j' => state.select_next(),
                            'k' => state.select_previous(),
                            _ => (),
                        },
                        _ => (),
                    },
                    Event::Mouse(_) => (),
                    Event::Paste(_) => (),
                    Event::Resize(_, _) => (),
                };
            }

            let _ = disable_raw_mode();
            let _ = execute!(io::stderr(), LeaveAlternateScreen);

            if let Some(session) = chosen_session {
                print!("{}", session);
                history.open_session(session);
            }

            let result = history.save();
            match result {
                Ok(_) => exit(0),
                Err(e) => {
                    eprintln!("could not save history: {}", e);
                    exit(1)
                }
            }
        }
        "--get" => {
            let history = History::get();
            let history = match history {
                Ok(history) => history,
                Err(e) => {
                    eprintln!("could not get history: {}", e);
                    exit(1)
                }
            };

            print!("{}", history);
            exit(0)
        }
        "--new" => {
            let Some(second_arg) = args.get(1) else {
                println!("session name must be provided when using --new");
                exit(1)
            };

            let history = History::get();
            let mut history = match history {
                Ok(history) => history,
                Err(e) => {
                    eprintln!("could not get history: {}", e);
                    exit(1)
                }
            };
            history.open_session(second_arg.to_string());
            if let Some(current_session) = history.current_session.as_ref() {
                print!("{}", current_session);
            }
            let result = history.save();
            match result {
                Ok(_) => exit(0),
                Err(e) => {
                    eprintln!("could not save history: {}", e);
                    exit(1)
                }
            }
        }
        "--next" => {
            let history = History::get();
            let mut history = match history {
                Ok(history) => history,
                Err(e) => {
                    eprintln!("could not get history: {}", e);
                    exit(1)
                }
            };
            history.open_next_session();
            if let Some(current_session) = history.current_session.as_ref() {
                print!("{}", current_session);
            }
            _ = history.save();
        }
        "--last" => {
            let history = History::get();
            let mut history = match history {
                Ok(history) => history,
                Err(e) => {
                    eprintln!("could not get history: {}", e);
                    exit(1)
                }
            };
            history.open_last_session();
            if let Some(current_session) = history.current_session.as_ref() {
                print!("{}", current_session);
            }
            _ = history.save();
        }
        "--help" => {
            println!("Usage:");
            println!("  --new <string>");
            println!("  --next");
            println!("  --last");
        }
        _ => (),
    }
}
