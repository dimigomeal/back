use crate::func;
use crate::ios::activity;
use crate::meal;
use std::{io, process};

pub async fn cron_command() -> io::Result<()> {
    let target = func::get_arg(2);
    match target.as_str() {
        "meal" => {
            println!("Start cron for meal");

            meal::meal_cron().await.unwrap();

            println!("End cron for meal");
            Ok(())
        }
        "ios-activity" => {
            println!("Start cron for ios activity");

            let private_key = func::get_arg(3);

            if private_key == "" {
                println!("Authentication token is required for ios activity cron");
                process::exit(1);
            }

            activity::activity_cron(&private_key).await.unwrap();

            println!("End cron for ios activity");
            Ok(())
        }
        "--help" => {
            println!("Usage: dimigomeal-back cron [TARGET]");
            println!("");
            println!("Targets:");
            println!("  meal                         Run cron for meal");
            println!("  ios-activity <auth_token>    Run cron for ios activity");
            println!("  --help                       Display this help message");
            Ok(())
        }
        _ => {
            println!("Invalid cron target: {}", target);
            println!("Try 'dimigomeal-back cron --help' for more information.");
            process::exit(1);
        }
    }
}
