use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Error, Seek, SeekFrom, Write},
};

const FILE_NAME: &str = "tmux-session-history";

pub fn parse_history_from_file(file: File) -> Result<History, String> {
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let Ok(_) = reader.read_line(&mut line) else {
        return Result::Err(String::from("Could not parse the history"));
    };

    let last_sessions: Vec<String> = line
        .split(',')
        .filter_map(|s| {
            let s = s.to_string().trim_end().to_string();
            if s.is_empty() {
                return None;
            }

            Some(s)
        })
        .collect();

    line.clear();
    let Ok(_) = reader.read_line(&mut line) else {
        return Result::Err(String::from("Could not parse the history"));
    };
    let current_session = line.trim_end().to_string().clone();

    line.clear();
    let Ok(_) = reader.read_line(&mut line) else {
        return Result::Err(String::from("Could not parse the history"));
    };

    let next_sessions: Vec<String> = line
        .split(',')
        .filter_map(|s| {
            let s = s.to_string().trim_end().to_string();
            if s.is_empty() {
                return None;
            }

            Some(s)
        })
        .collect();

    Ok(History {
        last_sessions,
        current_session: Some(current_session),
        next_sessions,
    })
}

fn get_history_file() -> Result<File, Error> {
    let config_dir = dirs::home_dir()
        .expect("cannot find home directory")
        .join(".config");
    std::fs::create_dir_all(&config_dir)?;
    let path = config_dir.join(FILE_NAME);

    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
}

#[derive(Debug)]
pub struct History {
    pub last_sessions: Vec<String>,
    pub current_session: Option<String>,
    pub next_sessions: Vec<String>,
}

impl History {
    pub fn get() -> Result<Self, String> {
        let config_file = match get_history_file() {
            Ok(file) => file,
            Err(e) => {
                return Result::Err(e.to_string());
            }
        };

        let history = match parse_history_from_file(config_file) {
            Ok(history) => history,
            Err(e) => {
                return Result::Err(e);
            }
        };

        Ok(history)
    }

    pub fn save(self) -> ::std::io::Result<()> {
        let mut file = match get_history_file() {
            Ok(file) => file,
            Err(e) => {
                return Result::Err(e);
            }
        };

        let mut content: String = String::new();
        content.push_str(self.last_sessions.join(",").as_str());
        content.push('\n');
        if let Some(current) = &self.current_session {
            content.push_str(current);
        }
        content.push('\n');
        content.push_str(self.next_sessions.join(",").as_str());
        content.push('\n');

        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;
        file.write_all(content.as_bytes())?;
        file.flush()?;

        Ok(())
    }

    pub fn open_new_session(&mut self, session: String) {
        if let Some(current) = &self.current_session {
            self.last_sessions.push(current.clone());
        }
        self.current_session = Some(session);
    }

    pub fn open_last_session(&mut self) {
        let last_session = self.last_sessions.last();
        match last_session {
            None => (),
            Some(last) => {
                if let Some(current) = &self.current_session {
                    self.next_sessions.insert(0, current.clone());
                }
                self.current_session = Some(last.to_string());
                self.last_sessions.pop();
            }
        }
    }

    pub fn open_next_session(&mut self) {
        let next_session = self.next_sessions.first();
        match next_session {
            None => (),
            Some(next) => {
                if let Some(current) = &self.current_session {
                    self.last_sessions.push(current.clone());
                }

                self.current_session = Some(next.to_string());
                self.next_sessions.remove(0);
            }
        }
    }
}
