mod shell;
mod cmus;
mod args;

extern crate discord_rpc_client;

use clap::Parser; 
use discord_rpc_client::Client;
use notify_rust::Notification;
use std::{thread, time::Duration};
use args::Args;

fn main() {
    let args = Args::parse();
    let mut current_song = String::new();
    let mut rpc = Client::new(args.client_id);

    if !args.debug {
        rpc.start();
    }

    loop {
        let cmus = cmus::CmusQuery::new();

        if cmus.remote.is_empty() { 
            println!("cmus is not running!");
            if !args.debug {
                rpc.clear_activity().expect("Failed to clear activity");
            }
            thread::sleep(Duration::from_secs(3));
            continue; 
        }

        if cmus.status == "playing" {
            if cmus.title != current_song {

                if !args.no_notifications {
                    Notification::new()
                        .summary("Now playing!")
                        .body(&format!("{} - {}", cmus.title, cmus.artist))
                        .urgency(notify_rust::Urgency::Low)
                        .show()
                        .expect("Failed to send notification");
                }
                
            }
    
            current_song = cmus.title.to_string();
    
            println!("{} - {} (-{})", cmus.title, cmus.artist, cmus.time_left);
            
            if !args.debug {    
                rpc
                    .set_activity(|activity| {
                        activity
                            .details(format!("{}", cmus.title))
                            .state(format!("{} (-{})", cmus.artist, cmus.time_left))
                            .assets(|asset| asset.large_image(args.client_large_image.as_str()))
                    })
                    .expect("Failed to set activity");
            }
        } else {
            if !args.debug {    
                rpc.clear_activity().expect("Failed to clear activity");
            }
        }

        thread::sleep(Duration::from_secs(1));
    }
}
