use crate::func;
use crate::ios::activity;
use std::{io, process};

pub async fn debug_command() -> io::Result<()> {
    let target = func::get_arg(2);
    match target.as_str() {
        "push-notify" => {
            println!("Push notification debug...");

            let auth_token = func::get_arg(3);
            let target = func::get_arg(4);
            let meal_type = func::get_arg(5);
            let menu = func::get_arg(6);
            let date = func::get_arg(7);

            if auth_token == "" || target == "" || meal_type == "" || menu == "" || date == "" {
                println!("All arguments are required for push notification debug");
                process::exit(1);
            }

            let result =
                activity::send_custom_notification(&auth_token, &target, &meal_type, &menu, &date)
                    .await
                    .unwrap();

            println!("Push notification sent: {:?}", result);
            Ok(())
        }
        "--help" => {
            println!("Usage: dimigomeal-back debug [COMMAND]");
            println!("");
            println!("Commands:");
            println!(
                "  push-notify <auth_token> <target> <type> <menu> <date>    Force push notification to target"
            );
            println!(
                "  --help                                               Display this help message"
            );
            Ok(())
        }
        _ => {
            println!("Invalid debug command: {}", target);
            println!("Try 'dimigomeal-back debug --help' for more information.");
            process::exit(1);
        }
    }
}
