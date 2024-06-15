use actix_web::{web, HttpResponse};

pub mod ios;
pub mod meal;

pub fn ios_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/activity/{token}")
            .route(web::get().to(ios::get_ios_activity))
            .route(web::post().to(ios::post_ios_acitvity))
            .route(web::delete().to(ios::delete_ios_activity))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

pub fn meal_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(meal::get_meal_today)));
    cfg.service(web::resource("/week").route(web::get().to(meal::get_meal_week_today)));
    cfg.service(web::resource("/{date}").route(web::get().to(meal::get_meal_date)));
    cfg.service(web::resource("/week/{date}").route(web::get().to(meal::get_meal_week_date)));
}
