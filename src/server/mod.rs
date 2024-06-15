use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::{io, result};

pub mod router;

pub fn index_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index_handler))
        .service(web::scope("/ios").configure(router::ios_config))
        .service(web::scope("/meal").configure(router::meal_config));
}

pub async fn index_handler() -> impl Responder {
    HttpResponse::Ok().body("dimigomeal server")
}

pub async fn run_server() -> result::Result<(), io::Error> {
    println!("Starting Actix web server...");

    HttpServer::new(move || App::new().configure(index_config))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
