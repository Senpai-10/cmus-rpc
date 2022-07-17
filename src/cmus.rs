use crate::shell;

pub struct CmusQuery {
    pub remote: String,
    pub status: Option<String>,
    pub status_symbol: Option<String>,
    pub file: Option<String>,
    pub duration: Option<u64>,
    pub position: Option<u64>,
    pub time_left: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub title: Option<String>,
    pub date: Option<String>,
    pub aaa_mode: Option<String>,
    pub continue_: Option<bool>,
    pub play_library: Option<bool>,
    pub play_sorted: Option<bool>,
    pub replaygain: Option<String>,
    pub replaygain_limit: Option<bool>,
    pub replaygain_preamp: Option<f64>,
    pub repeat: Option<bool>,
    pub repeat_current: Option<bool>,
    pub shuffle: Option<bool>,
    pub softvol: Option<bool>,
    pub vol_left: Option<u8>,
    pub vol_right: Option<u8>
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

            time_left = Some(String::from(format!("{hours}:{minutes}:{seconds}")));
        }

        Self {
            remote,
            status,
            status_symbol,
            file,
            duration,
            position,
            time_left,
            artist,
            album,
            title,
            date,
            aaa_mode,
            continue_,
            play_library,
            play_sorted,
            replaygain,
            replaygain_limit,
            replaygain_preamp,
            repeat,
            repeat_current,
            shuffle,
            softvol,
            vol_left,
            vol_right
        }
    }
}