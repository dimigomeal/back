mod command;
mod func;
mod ios;
mod server;

use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    command::run_command().await
}
