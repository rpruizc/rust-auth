use actix_web::{HttpResponse, web, web::ServiceConfig};

pub fn app_config(config: &mut ServiceConfig) {
    let health_resource = web::resource("/")
        .route(web::get().to(health));

    config.service(health_resource);
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}