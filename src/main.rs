use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod ios;

use ios::ios_config;

fn index_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/ios").configure(ios_config))
        .route("/", web::get().to(index_handler));
}

async fn index_handler() -> impl Responder {
    HttpResponse::Ok().body("dimigomeal push notification server")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Actix web server...");

    HttpServer::new(move || App::new().configure(index_config))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
