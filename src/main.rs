#[macro_use]
extern crate validator_derive;

mod config;
mod db;
mod errors;
mod handlers;
mod models;

use crate::config::Config;
use actix_web::{App, HttpServer, middleware::Logger};
use color_eyre::Result;
use handlers::app_config;
use tracing::{info, instrument};

#[actix_rt::main]
#[instrument]
async fn main() -> Result<()> {
    let config = Config::from_env().expect("Server configuration");

    let pool = config.db_pool().await.expect("Database configuration");

    let crypto_service = config.hashing();

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
