use crate::shell;
use regex::Regex;
use std::thread;
use std::time::Duration;

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

impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Query::Status => write!(f, "status"),
            Query::File => write!(f, "file"),
            Query::Duration => write!(f, "duration"),
            Query::Position => write!(f, "position"),
            Query::Artist => write!(f, "tag artist"),
            Query::Album => write!(f, "tag album"),
            Query::Title => write!(f, "tag title"),
            Query::Date => write!(f, "tag date"),
            Query::AaaMode => write!(f, "set aaa_mode"),
            Query::Continue_ => write!(f, "set continue_"),
            Query::PlayLibrary => write!(f, "set play_library"),
            Query::PlaySorted => write!(f, "set play_sorted"),
            Query::Replaygain => write!(f, "set replaygain"),
            Query::ReplaygainLimit => write!(f, "set replaygain_limit"),
            Query::ReplaygainPreamp => write!(f, "set replaygain_preamp"),
            Query::Repeat => write!(f, "set repeat"),
            Query::RepeatCurrent => write!(f, "set repeat_current"),
            Query::Shuffle => write!(f, "set shuffle"),
            Query::Softvol => write!(f, "set softvol"),
            Query::VolLeft => write!(f, "set vol_left"),
            Query::VolRight => write!(f, "set vol_right"),

            Query::StatusSymbol => write!(f, ""),
            Query::FormattedDuration => write!(f, ""),
            Query::FormattedPosition => write!(f, ""),
            Query::TimeLeft => write!(f, ""),
        }
    }
}

pub struct Status {
    output: String,
}

impl Status {
    pub fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    pub fn query_status(&mut self) -> () {
        loop {
            if let Some(remote) = shell::get_stdout("cmus-remote", "-Q") {
                if !remote.is_empty() {
                    self.output = remote;
                    break;
                }
            }

            println!("cmus is not running!");
            thread::sleep(Duration::from_secs(15));
        }
    }

    pub fn get(&self, q: Query) -> Option<String> {
        if q == Query::StatusSymbol {
            return match self.get(Query::Status) {
                Some(status) => match status.as_str() {
                    "playing" => Some(String::from(">")),
                    "paused" => Some(String::from("||")),
                    "stopped" => Some(String::from(".")),
                    _ => Some(String::from("?")),
                },
                None => Some(String::from("?")),
            };
        } else if q == Query::FormattedDuration {
            let duration = self.get(Query::Duration);

            if duration.is_some() {
                let duration: u64 = duration.unwrap().parse().unwrap();

                return Some(format_time(duration, true));
            } else {
                return Some(String::from("0"));
            }
        } else if q == Query::FormattedPosition {
            let position = self.get(Query::Position);

            if position.is_some() {
                let position: u64 = position.unwrap().parse().unwrap();

                return Some(format_time(position, true));
            } else {
                return Some(String::from("0"));
            }
        } else if q == Query::TimeLeft {
            let duration = self.get(Query::Duration);
            let position = self.get(Query::Position);

            if duration.is_some() && position.is_some() {
                let duration: u64 = duration.unwrap().parse().unwrap();
                let position: u64 = position.unwrap().parse().unwrap();

                return Some(format_time(duration - position, true));
            }
        }

        let re = Regex::new(&format!("(?m)^{} (.+)$", q.to_string())).unwrap();

        Some(re.captures(&self.output)?.get(1)?.as_str().to_string())
    }
}

fn pad(number: u64) -> String {
    if number < 10 && number != 0 {
        return format!("0{number}");
    }

    return number.to_string();
}

fn format_time(time: u64, clean: bool) -> String {
    let seconds = time % 60;
    let minutes = (time / 60) % 60;
    let hours = (time / 60) / 60;

    let mut format = String::new();

    if clean {
        if hours != 0 {
            format.push_str(&format!("{}:", pad(hours)));
        }
        if minutes != 0 {
            format.push_str(&format!("{}:", pad(minutes)));
        }

        format.push_str(&format!("{}", pad(seconds)));

        return format;
    }

    format = format!("{}:{}:{}", pad(hours), pad(minutes), pad(seconds));

    return format;
}
