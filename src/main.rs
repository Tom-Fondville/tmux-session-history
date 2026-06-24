use std::{env, process::exit};

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
