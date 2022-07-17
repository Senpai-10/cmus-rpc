mod shell;
mod cmus;

extern crate discord_rpc_client;

use discord_rpc_client::Client;
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
    let mut rpc = Client::new(718109162923360327);

    if !args.debug {
        rpc.start();
    }

    loop {
        let cmus = cmus::CmusQuery::new();

        if cmus.remote.is_empty() { 
            println!("cmus is not running!");
            rpc.clear_activity().expect("Failed to clear activity");
            thread::sleep(Duration::from_secs(3));
            continue; 
        }

        if cmus.status == "playing" {
            if cmus.title != current_song {

                if !args.no_notification {
                    Notification::new()
                        .summary("Now playing!")
                        .body(&format!("{} - {}", cmus.title, cmus.artist))
                        .urgency(notify_rust::Urgency::Low)
                        .show()
                        .expect("Failed to send notification");
                }
                
            }
    
            current_song = cmus.title.to_string();
    
            println!("{} - {}", cmus.title, cmus.artist);
    
            rpc
                .set_activity(|activity| {
                    activity
                        .details(format!("{}", cmus.title))
                        .state(format!("{} (-{})", cmus.artist, cmus.time_left))
                        .assets(|asset| asset.large_image("icon"))
                })
                .expect("Failed to set activity");
        } else {
            rpc.clear_activity().expect("Failed to clear activity");
        }

        thread::sleep(Duration::from_secs(1));
    }
}
