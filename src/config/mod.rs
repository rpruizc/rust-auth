pub mod crypto;

use color_eyre::Result;
use crypto::CryptoService;
use eyre::WrapErr;
use serde::Deserialize;
use dotenv::dotenv;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database_url: String,
    pub secret_key: String,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();       // Loads the configuration to the environment
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading configuration");

        let mut c = config::Config::new();
        c.merge(config::Environment::default())?;

        c.try_into()
            .context("loading configuration from environment ")
    }

    pub async fn db_pool(&self) -> Result<PgPool> {
        info!("Creating database connection pool");

        PgPool::builder()
            .connect_timeout(Duration::from_secs(30))
            .build(&self.database_url)
            .await
            .context("creating database connection pool")
    }

    pub fn crypto_service(&self) -> CryptoService {
        CryptoService {
            key: Arc::new(self.secret_key.clone())
        }
    }
}