use crate::func;
use crate::server;
use std::{io, process};

pub mod cronjob;

pub async fn run_command() -> io::Result<()> {
    let first_arg = func::get_arg(1);

    // switch first argument
    match first_arg.as_str() {
        "run" => {
            println!("Running server...");
            server::run_server().await.expect("Internal server error");
            Ok(())
        }
        "cron-job" => cronjob::cron_job_command().await,
        "--help" => {
            println!("Usage: dimigomeal-back [OPTION]");
            println!("");
            println!("Options:");
            println!("  run                   Run backend server");
            println!("  cron-job <target>     Run cron job for target");
            println!("  --help                Display this help message");
            Ok(())
        }
        _ => {
            println!("Invalid argument: {}", first_arg);
            println!("Try 'dimigomeal-back --help' for more information.");
            process::exit(1);
        }
    }
}
