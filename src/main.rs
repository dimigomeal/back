mod ios;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use ios::{activity, ios_config};
use std::{env, io, process, result};

fn index_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/ios").configure(ios_config))
        .route("/", web::get().to(index_handler));
}

async fn index_handler() -> impl Responder {
    HttpResponse::Ok().body("dimigomeal push notification server")
}

async fn run_server() -> result::Result<(), io::Error> {
    println!("Starting Actix web server...");

    HttpServer::new(move || App::new().configure(index_config))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let first_arg = env::args().nth(1).unwrap_or("".to_string());

    // if first argument is not empty
    if first_arg != "" {
        // switch first argument
        match first_arg.as_str() {
            "cron-job" => {
                let target = env::args().nth(2).unwrap_or("".to_string());
                match target.as_str() {
                    "ios-activity" => {
                        println!("Running cron job for ios activity...");
                        let authentication_token = env::args().nth(3).unwrap_or("".to_string());
                        if authentication_token == "" {
                            println!("Authentication token is required for ios activity cron job");
                            process::exit(1);
                        }
                        activity::activity_cron(&authentication_token);
                    }
                    "--help" => {
                        println!("Usage: dimigomeal-push cron-job [TARGET]");
                        println!("");
                        println!("Targets:");
                        println!("  ios-activity <auth_token>    Run cron job for ios activity");
                        println!("  --help                       Display this help message");
                    }
                    _ => {
                        println!("Invalid job target: {}", target);
                        println!("Try 'dimigomeal-push cron-job --help' for more information.");
                    }
                }
            }
            "--help" => {
                println!("Usage: dimigomeal-push [OPTION]");
                println!("");
                println!("Options:");
                println!("  cron-job <target>     Run cron job for target");
                println!("  --help                Display this help message");
            }
            _ => {
                println!("Invalid argument: {}", first_arg);
                println!("Try 'dimigomeal-push --help' for more information.");
            }
        }
        process::exit(0);
    }

    // ./dimigomeal-push
    run_server().await
}
