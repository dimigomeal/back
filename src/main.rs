use actix_web::{get, http::StatusCode, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("dimigomeal push notification server")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Actix web server...");

    HttpServer::new(move || App::new().service(index))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
