mod shell;
mod cmus;

use notify_rust::Notification;
use clap::Parser; 
use std::{thread, time::Duration};

/// Cmus rpc
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Don't send notifications when playing new songs
    #[clap(short = 'n', long, value_parser)]
    pub no_notification: bool,

    #[clap(short, long, value_parser)]
    pub debug: bool,
}

fn main() {
    let args = Args::parse();
    let mut current_song = String::new();

    loop {
        let cmus = cmus::CmusQuery::new();

        if cmus.remote.is_empty() {
            break;
        }

        if cmus.title != current_song {
            // send notifications here!
            if args.no_notification {
                continue;
            }

            Notification::new()
                .summary("Now playing!")
                .body(&format!("{} - {}", cmus.title, cmus.artist))
                .urgency(notify_rust::Urgency::Low)
                .show()
                .expect("Failed to send notification");
        }

        current_song = cmus.title.to_string();

        println!("{} - {}", cmus.title, cmus.artist);

        thread::sleep(Duration::from_secs(1));
    }
}
