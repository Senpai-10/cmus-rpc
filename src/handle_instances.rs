use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::process::exit;

use crate::args::Args;

pub fn handle_instances(args: &Args) {
    let prog_name = match get_prog_name() {
        Some(v) => v,
        None => return,
    };

    if !already_running(&prog_name) {
        return;
    }

    if args.kill_old {
        let pids = get_running_instances(&prog_name);

        for pid in pids {
            kill(pid).expect("Failed to kill process");
        }
    } else {
        println!("'{}' is already running!", prog_name);
        exit(1);
    }
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

fn get_running_instances(prog_name: &String) -> Vec<String> {
    let p = std::process::Command::new("pgrep")
        .arg(prog_name)
        .output()
        .expect("Failed to run pgrep");

    let mut stdout = String::from_utf8(p.stdout).unwrap();

    // remove \n at the end
    stdout.pop();

    let mut pids: Vec<String> = Vec::new();

    for process_id in stdout.lines() {
        pids.push(process_id.to_owned())
    }

    // remove the new process id to prevent self kill
    pids.pop();

    pids
}

fn kill(pid: String) -> Result<std::process::ExitStatus, std::io::Error> {
    std::process::Command::new("kill")
        .arg("-9")
        .arg(pid)
        .status()
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
