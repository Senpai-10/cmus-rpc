mod app;
mod args;

use std::process::exit;

use app::app;
use args::Args;
use clap::Parser;
use discord_rpc_client::Client;

fn main() {
    if already_running() {
        println!("cmus-rpc is already running!");
        exit(1);
    }

    let args = Args::parse();
    let mut rpc = Client::new(args.client_id);

    if !args.debug {
        rpc.start();
    }

    app(args, rpc);
}

/// check if cmus-rpc is already running
fn already_running() -> bool {
    let p = std::process::Command::new("pgrep").arg("--count").arg("cmus-rpc").output().expect("Failed to run pgrep");
    let mut p_string = String::from_utf8(p.stdout).unwrap();

    p_string.pop();

    let processes_count: u32 = p_string.parse().unwrap();

    processes_count >= 2
}
