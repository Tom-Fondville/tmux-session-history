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

    let history = History::get();
    let Ok(mut history) = history else { exit(1) };

    let first_arg = args.first().cloned().unwrap_or("".to_string());
    match first_arg.as_str() {
        "--new" => {
            let Some(second_arg) = args.get(1) else {
                println!("session name must be provided when using --new");
                exit(1)
            };

            history.open_new_session(second_arg.to_string());
        }
        "--next" => history.open_next_session(),
        "--last" => history.open_last_session(),
        "--help" => {
            println!("Usage:");
            println!("  --new <string>");
            println!("  --next");
            println!("  --last");
        }
        _ => (),
    }
    _ = history.save();

    // let history = History::get();
    // let Ok(mut history) = history else { exit(1) };
    // println!("{:?}", history);
    //
    // history.open_new_session("ma nouvelle".to_string());
    // println!("{:?}", history);
    //
    // history.open_last_session();
    // println!("{:?}", history);
    //
    // history.open_last_session();
    // println!("{:?}", history);
    //
    // history.open_next_session();
    // println!("{:?}", history);
}
