use crate::args::Args;
use cmus_wrapper::query;
use discord_rpc_client::Client;
use notify_rust::Notification;
use query::Query;
use std::path;
use std::process::Command;
use std::{collections::HashMap, thread, time::Duration};

pub fn app(args: Args, mut rpc: Client) -> () {
    let mut current_song = String::new();
    let mut query_map: query::QueryMap = HashMap::new();

    loop {
        if query::load(&mut query_map) == false {
            println!("cmus is not running!");
            if !args.debug {
                rpc.clear_activity().expect("Failed to clear activity");
            }
            thread::sleep(Duration::from_secs(3));
            continue;
        }

        let song_status: String = query_map
            .get(&Query::Status)
            .unwrap_or(&String::new())
            .to_owned();

        if song_status == "playing" {
            let file: &String = query_map.get(&Query::File).unwrap();
            let title: String = query_map
                .get(&Query::Title)
                .unwrap_or(&String::from("Unknown title"))
                .to_owned();
            let artist: String = query_map
                .get(&Query::Artist)
                .unwrap_or(&String::from("Unknown artist"))
                .to_owned();
            let time_left: String = query_map
                .get(&Query::TimeLeft)
                .unwrap_or(&String::new())
                .to_owned();

            let song_cover = get_song_cover(file);

            if title != current_song {
                if !args.no_notifications {
                    let mut notify = Notification::new();
                    notify.summary("Now playing!");
                    notify.body(&format!("{} - {}", title, artist));
                    notify.urgency(notify_rust::Urgency::Low);
                    if let Some(cover_path) = song_cover {
                        notify.icon(&cover_path);
                    }

                    notify.show().expect("Failed to send notification");
                }
            }

            current_song = title.clone();

            println!("{} - {} (-{})", title, artist, time_left);

            if !args.debug {
                rpc.set_activity(|activity| {
                    activity
                        .details(format!("{}", title))
                        .state(format!("{} (-{})", artist, time_left))
                        .assets(|asset| asset.large_image(args.client_large_image.as_str()))
                })
                .expect("Failed to set activity");
            }
        } else {
            if !args.debug {
                rpc.clear_activity().expect("Failed to clear activity");
            }
        }

        thread::sleep(Duration::from_millis(args.interval));
    }
}

pub fn get_song_cover(file_path: &String) -> Option<String> {
    let file = path::Path::new(file_path);

    let file_name = file.file_name().unwrap();
    let tmp_song_name = path::Path::new(file_name)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    let song_cover = tmp_song_name.replace(" ", "_");
    let song_cover_file = format!("{}.png", song_cover);
    let tmp_dir = path::Path::new("/tmp/cmus_rpc_song_cover_cache");

    let song_cover_file_path = tmp_dir.join(path::Path::new(&song_cover_file));

    if !tmp_dir.exists() {
        std::fs::create_dir(tmp_dir).expect("Failed to create tmp_dir");
    }

    if !song_cover_file_path.exists() {
        let mut cmd = Command::new("sh");
        cmd.arg("-c");

        cmd.arg(format!(
            "ffmpeg -i '{}' -y -an -c:v copy '/tmp/cmus_rpc_song_cover_cache/tmp.png' -vf scale='-1:200' '{}'",
            file.to_str().unwrap(),
            song_cover_file_path.to_str().unwrap()
        ));

        let output = cmd.output().expect("Failed to run ffmpeg");

        if !output.status.success() {
            return None;
        }
    }

    Some(song_cover_file_path.to_str().unwrap().to_owned())
}
