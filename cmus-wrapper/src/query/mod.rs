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

pub type QueryMap = HashMap<Query, String>;

/// query status
/// return: false if cmus is not running.
pub fn load(status: &mut QueryMap) -> bool {
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

            status.insert(Query::Status, status_string);
            status.insert(Query::StatusSymbol, status_symbol);
        }

        if line.starts_with("file ") {
            status.insert(Query::File, line.replace("file ", ""));
        }

        if line.starts_with("duration ") {
            status
                .insert(Query::Duration, line.replace("duration ", ""));
        }

        if line.starts_with("position ") {
            status
                .insert(Query::Position, line.replace("position ", ""));
        }

        if line.starts_with("tag artist ") {
            status
                .insert(Query::Artist, line.replace("tag artist ", ""));
        }

        if line.starts_with("tag album ") {
            status
                .insert(Query::Album, line.replace("tag album ", ""));
        }

        if line.starts_with("tag title ") {
            status
                .insert(Query::Title, line.replace("tag title ", ""));
        }

        if line.starts_with("tag date ") {
            status
                .insert(Query::Date, line.replace("tag date ", ""));
        }

        if line.starts_with("set aaa_mode ") {
            status
                .insert(Query::AaaMode, line.replace("set aaa_mode ", ""));
        }

        if line.starts_with("set continue_ ") {
            status
                .insert(Query::Continue_, line.replace("set continue_ ", ""));
        }

        if line.starts_with("set play_library ") {
            status
                .insert(Query::PlayLibrary, line.replace("set play_library ", ""));
        }

        if line.starts_with("set play_sorted ") {
            status
                .insert(Query::PlaySorted, line.replace("set play_sorted ", ""));
        }

        if line.starts_with("set replaygain ") {
            status
                .insert(Query::Replaygain, line.replace("set replaygain ", ""));
        }

        if line.starts_with("set replaygain_limit ") {
            status.insert(
                Query::ReplaygainLimit,
                line.replace("set replaygain_limit ", ""),
            );
        }

        if line.starts_with("set replaygain_preamp ") {
            status.insert(
                Query::ReplaygainPreamp,
                line.replace("set replaygain_preamp ", ""),
            );
        }

        if line.starts_with("set repeat ") {
            status
                .insert(Query::Repeat, line.replace("set repeat ", ""));
        }

        if line.starts_with("set repeat_current ") {
            status.insert(
                Query::RepeatCurrent,
                line.replace("set repeat_current ", ""),
            );
        }

        if line.starts_with("set shuffle ") {
            status
                .insert(Query::Shuffle, line.replace("set shuffle ", ""));
        }

        if line.starts_with("set softvol ") {
            status
                .insert(Query::Softvol, line.replace("set softvol ", ""));
        }

        if line.starts_with("set vol_left ") {
            status
                .insert(Query::VolLeft, line.replace("set vol_left ", ""));
        }

        if line.starts_with("set vol_right ") {
            status
                .insert(Query::VolRight, line.replace("set vol_right ", ""));
        }
    }

    if status.get(&Query::Duration).is_some() && status.get(&Query::Position).is_some() {
        let duration: u64 = status.get(&Query::Duration).unwrap().parse().unwrap();
        let position: u64 = status.get(&Query::Position).unwrap().parse().unwrap();

        status
            .insert(Query::FormattedDuration, format_time(duration, true));
        status
            .insert(Query::FormattedPosition, format_time(position, true));

        status
            .insert(Query::TimeLeft, format_time(duration - position, true));
    }

    true
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
