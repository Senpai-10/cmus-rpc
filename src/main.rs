mod app;
mod args;
mod handle_instances;

use app::app;
use args::Args;
use clap::Parser;
use discord_rpc_client::Client;
use handle_instances::handle_instances;

fn main() {
    let args = Args::parse();

    handle_instances(&args);

    let mut rpc = Client::new(args.client_id);

    if !args.debug {
        rpc.start();
    }

    app(args, rpc);
}

