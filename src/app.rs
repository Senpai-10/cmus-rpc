use crate::args::Args;
use cmus_wrapper::query;
use discord_rpc_client::Client;
use notify_rust::Notification;
use query::Query;
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

            if title != current_song {
                if !args.no_notifications {
                    Notification::new()
                        .summary("Now playing!")
                        .body(&format!("{} - {}", title, artist))
                        .urgency(notify_rust::Urgency::Low)
                        .show()
                        .expect("Failed to send notification");
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
