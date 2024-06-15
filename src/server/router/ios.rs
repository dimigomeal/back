use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::ios::activity::add_device_token;
use crate::ios::activity::get_device_token;
use crate::ios::activity::remove_device_token;

pub fn check_device_token_format(key: &str) -> bool {
    key.chars().all(|c| c.is_digit(16))
}

pub async fn get_ios_activity(path: web::Path<String>) -> impl Responder {
    let token = path.into_inner();

    if !check_device_token_format(&token) {
        return HttpResponse::BadRequest().body("Invalid token format");
    }

    let result = get_device_token(token).await.unwrap();

    HttpResponse::Ok().body(serde_json::to_string(&result).unwrap())
}

pub async fn post_ios_acitvity(path: web::Path<String>) -> impl Responder {
    let token = path.into_inner();

    if !check_device_token_format(&token) {
        return HttpResponse::BadRequest().body("Invalid token format");
    }

    let result = add_device_token(token).await.unwrap();

    HttpResponse::Ok().body(serde_json::to_string(&result).unwrap())
}

pub async fn delete_ios_activity(path: web::Path<String>) -> impl Responder {
    let token = path.into_inner();

    if !check_device_token_format(&token) {
        return HttpResponse::BadRequest().body("Invalid token format");
    }

    let result = remove_device_token(token).await.unwrap();

    if result == false {
        return HttpResponse::BadRequest().body("Token not found");
    }

    HttpResponse::Ok().body("Token deleted")
}
