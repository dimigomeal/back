use actix_web::{web, HttpResponse};
use serde::Serialize;

pub mod ios;

#[derive(Serialize)]
pub struct IosActivityKey {
    pub last_date: String,
    pub activity_key: String,
}

pub fn ios_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/activity/{token}")
            .route(web::get().to(ios::get_ios_activity))
            .route(web::post().to(ios::post_ios_acitvity))
            .route(web::delete().to(ios::delete_ios_activity))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}
