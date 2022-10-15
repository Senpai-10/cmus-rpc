mod shell;
mod parser;
mod args;
mod app;

extern crate discord_rpc_client;

use app::app;
use clap::Parser; 
use discord_rpc_client::Client;
use args::Args;

fn main() {
    let args = Args::parse();
    let mut rpc = Client::new(args.client_id);

    if !args.debug {
        rpc.start();
    }

    app(args, rpc);
}
