use crate::args::Args;
use crate::parser::CmusQuery;
use discord_rpc_client::Client;
use std::{thread, time::Duration};
use notify_rust::Notification;

pub fn app(args: Args, mut rpc: Client) -> () {
    let mut current_song = String::new();

    loop {
        let cmus = CmusQuery::new();

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
        
        thread::sleep(Duration::from_millis(args.interval));
    }
}