mod app;
mod args;

use std::process::exit;
use app::app;
use args::Args;
use clap::Parser;
use discord_rpc_client::Client;
use std::env;
use std::ffi::OsStr;
use std::path::Path;

fn main() {
    let prog_name = match get_prog_name() {
        Some(v) => v,
        None=> return
    };

    if already_running(&prog_name) {
        println!("'{}' is already running!", prog_name);
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
fn already_running(prog_name: &String) -> bool {
    let p = std::process::Command::new("pgrep")
        .arg("--count")
        .arg(prog_name)
        .output()
        .expect("Failed to run pgrep");

    let mut p_string = String::from_utf8(p.stdout).unwrap();

    p_string.pop();

    let processes_count: u32 = p_string.parse().unwrap();

    processes_count >= 2
}

fn get_prog_name() -> Option<String> {
    env::current_exe()
        .ok()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from)
}
