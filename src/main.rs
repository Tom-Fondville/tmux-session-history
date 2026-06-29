use std::{env, process::exit};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    crossterm, init,
    layout::{Constraint, Layout},
    restore,
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
            let history = match history {
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

            let mut terminal = init();
            let state = &mut ListState::default();
            state.select(Some(history.last_sessions.len()));

            loop {
                _ = terminal.draw(|frame| {
                    frame.render_widget("toto", frame.area());
                    let mut sessions_items = Vec::<ListItem>::new();
                    for last_session in history.last_sessions.clone() {
                        sessions_items.push(ListItem::new(last_session));
                    }
                    if let Some(current_session) = history.current_session.clone() {
                        sessions_items.push(ListItem::new(current_session));
                    }
                    for next_session in history.next_sessions.clone() {
                        sessions_items.push(ListItem::new(next_session));
                    }

                    let list = List::new(sessions_items)
                        .block(
                            Block::default()
                                .title("Select session")
                                .borders(Borders::ALL),
                        )
                        .highlight_style(
                            Style::default()
                                .bg(Color::Blue)
                                .fg(Color::White)
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
                            // let index = state.selected().unwrap_or_default();
                            // print!(selected);
                            print!("toto");
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

            restore();
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
            history.open_new_session(second_arg.to_string());
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
