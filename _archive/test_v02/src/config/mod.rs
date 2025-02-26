use color_eyre::Result;
use eyre::WrapErr;
use serde::Deserialize;
use dotenv::dotenv;
use tracing_subscriber::{fmt, filter::EnvFilter};
use config::{Config as ConfigLoader, Environment};
use tracing::{info, instrument};
#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
}

impl Config {
        #[instrument]
    pub fn from_env() -> Result<Self> {
        dotenv().ok();

        // Initialize tracing
        fmt().with_env_filter(EnvFilter::from_default_env()).init();

        info!("Loading configuration");

        // Load configuration using the new `Config::builder()` approach
        let config = ConfigLoader::builder()
            .add_source(Environment::default()) 
            .build()
            .wrap_err("loading configuration from environment")?;

        config.try_deserialize().wrap_err("parsing configuration")
    }
}
