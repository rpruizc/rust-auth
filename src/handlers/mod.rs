mod auth;
mod user;

use crate::errors::AppError;
use actix_web::{HttpResponse, web};
use auth::auth;
use user::{create_user, me, update_profile};

type AppResult<T> = Result<T, AppError>;
type AppResponse = AppResult<HttpResponse>;

pub fn app_config(config: &mut web::ServiceConfig) {
    let signup = web::resource("/signup").route(web::post().to(create_user));

    let auth = web::resource("/auth").route(web::post().to(auth));

    let me = web::resource("/me")
        .route(web::get().to(me))
        .route(web::post().to(update_profile));

    let health_resource = web::resource("/")
        .route(web::get().to(health));

    config
        .service(signup)
        .service(auth)
        .service(me)
        .service(health_resource);
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}