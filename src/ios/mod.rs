use actix_web::{web, HttpResponse};
use serde::Serialize;

pub mod activity;

#[derive(Serialize)]
pub struct IosActivityKey {
    pub last_date: String,
    pub activity_key: String,
}

pub fn ios_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/activity")
            .route(web::get().to(activity::get_ios_activity))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    )
    .service(
        web::resource("/activity/{token}")
            .route(web::post().to(activity::post_ios_acitvity))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}
