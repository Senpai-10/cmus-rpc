mod app;
mod args;
mod parser;

// use app::app;
use args::Args;
use clap::Parser;
use cmus_wrapper as cw;
use cw::status::Status;
use discord_rpc_client::Client;

fn main() {
    let args = Args::parse();
    let mut rpc = Client::new(args.client_id);

    if !args.debug {
        rpc.start();
    }

    let mut status = Status::new();

    let q = status.query();

    if q == false {
        println!("cmus is not running!");
    } else {
        println!("{:?}", status.get(cw::status::Query::Status));
        println!("{:?}", status.get(cw::status::Query::StatusSymbol));
    }
    // app(args, rpc);
}
