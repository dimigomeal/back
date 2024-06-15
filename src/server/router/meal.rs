use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use regex::Regex;
use serde_json::json;
use std::sync::Arc;

use crate::meal::get_meal;
use crate::meal::get_meal_week;

pub fn check_date_format(date: &str) -> bool {
    let date_regex =
        Arc::new(Regex::new(r"^\d{4}\-(0[1-9]|1[012])\-(0[1-9]|[12][0-9]|3[01])$").unwrap());

    date_regex.is_match(date)
}

pub async fn get_meal_date(path: web::Path<String>) -> impl Responder {
    let date = path.into_inner();

    if !check_date_format(&date) {
        return HttpResponse::BadRequest().body("Invalid date format");
    }

    let result = get_meal(&date).await;

    match result {
        Ok(meal) => {
            let data = json!({
                "date": meal.date,
                "breakfast": meal.breakfast,
                "lunch": meal.lunch,
                "dinner": meal.dinner,
            });

            HttpResponse::Ok()
                .content_type("application/json; charset=utf-8")
                .json(data)
        }
        Err(_) => HttpResponse::NotFound().body("Meal not found"),
    }
}

pub async fn get_meal_today() -> impl Responder {
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();

    get_meal_date(web::Path::from(date)).await
}

pub async fn get_meal_week_date(path: web::Path<String>) -> impl Responder {
    let date = path.into_inner();

    if !check_date_format(&date) {
        return HttpResponse::BadRequest().body("Invalid date format");
    }

    let meals = get_meal_week(&date).await.unwrap();

    let mut data = Vec::new();
    for meal in meals {
        let meal_data = json!({
            "date": meal.date,
            "breakfast": meal.breakfast,
            "lunch": meal.lunch,
            "dinner": meal.dinner,
        });
        data.push(meal_data);
    }

    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(data)
}

pub async fn get_meal_week_today() -> impl Responder {
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();

    get_meal_week_date(web::Path::from(date)).await
}
