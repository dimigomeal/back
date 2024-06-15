use crate::func;
use crate::server;
use std::{io, process};

pub mod cron;
pub mod debug;

pub async fn run_command() -> io::Result<()> {
    let first_arg = func::get_arg(1);

    match first_arg.as_str() {
        "run" => {
            println!("Running server...");
            let environment = func::get_arg(2);
            if environment == "development" {
                std::env::set_var("RUST_LOG", "actix_web=debug");
                std::env::set_var("RUST_BACKTRACE", "1");
                std::env::set_var("ENVIROMENT", "development");
            } else {
                std::env::set_var("ENVIROMENT", "production");
            }
            server::run_server().await.expect("Internal server error");
            Ok(())
        }
        "cron" => cron::cron_command().await,
        "debug" => debug::debug_command().await,
        "--help" => {
            println!("Usage: dimigomeal-back [OPTION]");
            println!("");
            println!("Options:");
            println!("  run <environment>      Run server");
            println!("  cron <target>          Run cron for target");
            println!("  debug <target>         Run debug for target");
            println!("  --help                 Display this help message");
            Ok(())
        }
        _ => {
            println!("Invalid argument: {}", first_arg);
            println!("Try 'dimigomeal-back --help' for more information.");
            process::exit(1);
        }
    }
}
