use crate::shell;

pub struct CmusQuery {
    pub remote: String,
    pub status: String,
    pub status_symbol: String,
    pub file: String,
    pub duration: u64,
    pub position: u64,
    pub time_left: String,
    pub artist: String,
    pub album: String,
    pub title: String,
    pub date: String,
    pub aaa_mode: String,
    pub continue_: bool,
    pub play_library: bool,
    pub play_sorted: bool,
    pub replaygain: String,
    pub replaygain_limit: bool,
    pub replaygain_preamp: f64,
    pub repeat: bool,
    pub repeat_current: bool,
    pub shuffle: bool,
    pub softvol: bool,
    pub vol_left: u8,
    pub vol_right: u8
}

impl CmusQuery {
    pub fn new() -> Self {
        let remote = shell::get_stdout("cmus-remote", "-Q").unwrap_or(String::new());

        let mut status: Option<String> = None;
        let mut status_symbol: Option<String> = None;
        let mut file: Option<String> = None;
        let mut duration: Option<u64> = None;
        let mut position: Option<u64> = None;
        let mut time_left: Option<String> = None;
        let mut artist: Option<String> = None;
        let mut album: Option<String> = None;
        let mut title: Option<String> = None;
        let mut date: Option<String> = None;
        let mut aaa_mode: Option<String> = None;
        let mut continue_: Option<bool> = None;
        let mut play_library: Option<bool> = None;
        let mut play_sorted: Option<bool> = None;
        let mut replaygain: Option<String> = None;
        let mut replaygain_limit: Option<bool> = None;
        let mut replaygain_preamp: Option<f64> = None;
        let mut repeat: Option<bool> = None;
        let mut repeat_current: Option<bool> = None;
        let mut shuffle: Option<bool> = None;
        let mut softvol: Option<bool> = None;
        let mut vol_left: Option<u8> = None;
        let mut vol_right: Option<u8> = None;

        for line in remote.lines() {
            if line.starts_with("status ") { 
                let status_string = line.replace("status ", "");

                status_symbol = match status_string.as_str() {
                    "playing" => Some(String::from(">")),
                    "paused" => Some(String::from("|")),
                    "stopped" => Some(String::from(".")),
                    _ => Some(String::from("?")),
                };

                status = Some(status_string)
            }
            if line.starts_with("file ") { file = Some(line.replace("file ", "")) }
            if line.starts_with("duration ") { duration = Some(line.replace("duration ", "").parse().unwrap()) }
            if line.starts_with("position ") { position = Some(line.replace("position ", "").parse().unwrap()) }
        
            if line.starts_with("tag artist ") { artist = Some(line.replace("tag artist ", "")) }
            if line.starts_with("tag album ") { album = Some(line.replace("tag album ", "")) }
            if line.starts_with("tag title ") { title = Some(line.replace("tag title ", "")) }
            if line.starts_with("tag date ") { date = Some(line.replace("tag date ", "")) }
            if line.starts_with("set aaa_mode ") { aaa_mode = Some(line.replace("set aaa_mode ", "")) }
            if line.starts_with("set continue_ ") { continue_ = Some(line.replace("set continue_ ", "").parse().unwrap()) }
            if line.starts_with("set play_library ") { play_library = Some(line.replace("set play_library ", "").parse().unwrap()) }
            if line.starts_with("set play_sorted ") { play_sorted = Some(line.replace("set play_sorted ", "").parse().unwrap()) }
            if line.starts_with("set replaygain ") { replaygain = Some(line.replace("set replaygain ", "")) }
            if line.starts_with("set replaygain_limit ") { replaygain_limit = Some(line.replace("set replaygain_limit ", "").parse().unwrap()) }
            if line.starts_with("set replaygain_preamp ") { replaygain_preamp = Some(line.replace("set replaygain_preamp ", "").parse().unwrap()) }
            if line.starts_with("set repeat ") { repeat = Some(line.replace("set repeat ", "").parse().unwrap()) }
            if line.starts_with("set repeat_current ") { repeat_current = Some(line.replace("set repeat_current ", "").parse().unwrap()) }
            if line.starts_with("set shuffle ") { shuffle = Some(line.replace("set shuffle ", "").parse().unwrap()) }
            if line.starts_with("set softvol ") { softvol = Some(line.replace("set softvol ", "").parse().unwrap()) }
            if line.starts_with("set vol_left ") { vol_left = Some(line.replace("set vol_left ", "").parse().unwrap()) }
            if line.starts_with("set vol_right ") { vol_right = Some(line.replace("set vol_right ", "").parse().unwrap()) }
        }

        if duration.is_some() && position.is_some() {
            let tmp_timeleft = duration.unwrap() - position.unwrap();
            
            let seconds = tmp_timeleft % 60;
            let minutes = (tmp_timeleft / 60) % 60;
            let hours = (tmp_timeleft / 60) / 60;

            let mut format = String::new();

            if hours != 0 {
                format.push_str(&format!("{}:", pad(hours)));
            }
            if minutes != 0 {
                format.push_str(&format!("{}:", pad(minutes)));
            }

            format.push_str(&format!("{}", pad(seconds)));

            time_left = Some(format);
        }

        Self {
            remote,
            status: status.unwrap_or_default(),
            status_symbol: status_symbol.unwrap_or_default(),
            file: file.unwrap_or_default(),
            duration: duration.unwrap_or_default(),
            position: position.unwrap_or_default(),
            time_left: time_left.unwrap_or_default(),
            artist: artist.unwrap_or_default(),
            album: album.unwrap_or_default(),
            title: title.unwrap_or_default(),
            date: date.unwrap_or_default(),
            aaa_mode: aaa_mode.unwrap_or_default(),
            continue_: continue_.unwrap_or_default(),
            play_library: play_library.unwrap_or_default(),
            play_sorted: play_sorted.unwrap_or_default(),
            replaygain: replaygain.unwrap_or_default(),
            replaygain_limit: replaygain_limit.unwrap_or_default(),
            replaygain_preamp: replaygain_preamp.unwrap_or_default(),
            repeat: repeat.unwrap_or_default(),
            repeat_current: repeat_current.unwrap_or_default(),
            shuffle: shuffle.unwrap_or_default(),
            softvol: softvol.unwrap_or_default(),
            vol_left: vol_left.unwrap_or_default(),
            vol_right: vol_right.unwrap_or_default()
        }
    }
}

fn pad(number: u64) -> String {
    if number < 10 && number != 0 {
        return format!("0{number}")
    }

    return number.to_string();
}