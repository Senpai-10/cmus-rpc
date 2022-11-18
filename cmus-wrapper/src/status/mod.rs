use crate::shell;

use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
pub enum Query {
    Status,
    StatusSymbol,
    File,
    Duration,
    Position,
    FormattedDuration,
    FormattedPosition,
    TimeLeft,
    Artist,
    Album,
    Title,
    Date,
    AaaMode,
    Continue_,
    PlayLibrary,
    PlaySorted,
    Replaygain,
    ReplaygainLimit,
    ReplaygainPreamp,
    Repeat,
    RepeatCurrent,
    Shuffle,
    Softvol,
    VolLeft,
    VolRight,
}

pub struct Status {
    status: HashMap<Query, String>,
}

impl Status {
    pub fn new() -> Self {
        Self {
            status: HashMap::new(),
        }
    }

    pub fn get(&self, query: Query) -> Option<String> {
        self.status.get(&query).cloned()
    }

    /// query status
    /// return: false if cmus is not running.
    pub fn query(&mut self) -> bool {
        let remote = match shell::get_stdout("cmus-remote", "-Q") {
            Some(v) => v,
            None => return false,
        };

        for line in remote.lines() {
            if line.starts_with("status ") {
                let status_string = line.replace("status ", "");

                let status_symbol: String = match status_string.as_str() {
                    "playing" => String::from(">"),
                    "paused" => String::from("||"),
                    "stopped" => String::from("."),
                    _ => String::from("?"),
                };

                self.status.insert(Query::Status, status_string);
                self.status.insert(Query::StatusSymbol, status_symbol);
            }
        }

        true
    }
}
