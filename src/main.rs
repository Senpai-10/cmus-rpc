mod app;
mod args;

use app::app;
use args::Args;
use clap::Parser;
use discord_rpc_client::Client;

fn main() {
    let args = Args::parse();
    let mut rpc = Client::new(args.client_id);

    if !args.debug {
        rpc.start();
    }

    app(args, rpc);
}
