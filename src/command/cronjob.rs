use crate::func;
use crate::ios::activity;
use std::{io, process};

pub async fn cron_job_command() -> io::Result<()> {
    let target = func::get_arg(2);
    match target.as_str() {
        "ios-activity" => {
            println!("Running cron job for ios activity...");
            let authentication_token = func::get_arg(3);
            if authentication_token == "" {
                println!("Authentication token is required for ios activity cron job");
                process::exit(1);
            }
            activity::activity_cron(&authentication_token)
                .await
                .unwrap();
            Ok(())
        }
        "--help" => {
            println!("Usage: dimigomeal-back cron-job [TARGET]");
            println!("");
            println!("Targets:");
            println!("  ios-activity <auth_token>    Run cron job for ios activity");
            println!("  --help                       Display this help message");
            Ok(())
        }
        _ => {
            println!("Invalid job target: {}", target);
            println!("Try 'dimigomeal-back cron-job --help' for more information.");
            process::exit(1);
        }
    }
}
