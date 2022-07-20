use clap::Parser; 

/// Discord Rich Presence integration for the C* Music Player
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Don't send notifications when playing new songs
    #[clap(short = 'n', long, value_parser)]
    pub no_notifications: bool,

    #[clap(short, long, value_parser)]
    pub debug: bool,

    /// discord rpc client id
    #[clap(short, long, value_parser, default_value = "999057193057919036")]
    pub client_id: u64,

    #[clap(short = 'l', long, value_parser, default_value = "icon")]
    pub client_large_image: String
}