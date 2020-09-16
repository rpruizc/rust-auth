#[macro_use]
extern crate validator_derive;

mod config;
mod db;
mod handlers;
mod models;

use color_eyre::Result;
use crate::config::Config;
use handlers::app_config;
use actix_web::{App, HttpServer, middleware::Logger};
use tracing::info;

#[actix_rt::main]
async fn main() -> Result<()> {
    let config = Config::from_env()
        .expect("Server configuration");

    let pool = config.db_pool().await
        .expect("Database configuration");

    let crypto_service = config.crypto_service();

    info!("Starting server at http://{}:{}/", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .data(crypto_service.clone())
            .configure(app_config)
    })
        .bind(format!{"{}:{}", config.host, config.port})?
        .run()
        .await?;
    Ok(())
}
