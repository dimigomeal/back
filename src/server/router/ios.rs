use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use serde::Serialize;

use crate::ios::activity::add_new_push_token;
use crate::ios::activity::delete_push_token;
use crate::ios::activity::get_push_token_data;

// Struct representing iOS activity push token
#[derive(Serialize)]
pub struct IosActivityPushToken {
    pub last_date: String,
    pub push_token: String,
}

// Checks if the token format is valid
pub fn check_token_format(key: &str) -> bool {
    if key.len() != 160 {
        return false;
    }

    for c in key.chars() {
        if !c.is_ascii_alphanumeric() {
            return false;
        }
    }

    true
}

// Handler for GET /ios/activity/<push_token>
pub async fn get_ios_activity(path: web::Path<String>) -> impl Responder {
    let token = path.into_inner();

    if !check_token_format(&token) {
        return HttpResponse::BadRequest().body("Invalid token format");
    }

    let result = get_push_token_data(token).await.unwrap();

    HttpResponse::Ok().body(serde_json::to_string(&result).unwrap())
}

// Handler for POST /ios/activity/<push_token>
// If push_token is already in the database return BadRequest, otherwise insert to database
pub async fn post_ios_acitvity(path: web::Path<String>) -> impl Responder {
    let token = path.into_inner();

    if !check_token_format(&token) {
        return HttpResponse::BadRequest().body("Invalid token format");
    }

    let result = add_new_push_token(token).await.unwrap();

    HttpResponse::Ok().body(serde_json::to_string(&result).unwrap())
}

// Handler for DELETE /ios/activity/<push_token>
pub async fn delete_ios_activity(path: web::Path<String>) -> impl Responder {
    let token = path.into_inner();

    if !check_token_format(&token) {
        return HttpResponse::BadRequest().body("Invalid token format");
    }

    let result = delete_push_token(token).await.unwrap();

    if result == false {
        return HttpResponse::BadRequest().body("Token not found");
    }

    HttpResponse::Ok().body("Token deleted")
}

// add FromRequest for ForcePushBody
#[derive(Serialize)]
pub struct ForcePushBody {
    pub authentication_token: &'static str,
    pub push_token: &'static str,
    pub meal_type: &'static str,
    pub meal_data: &'static str,
    pub date: &'static str,
}
