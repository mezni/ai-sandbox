use config::{Config as ConfigBuilder, ConfigError, Environment};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let builder = ConfigBuilder::builder();

        let config = builder.add_source(Environment::default()).build()?;

        config.try_deserialize()
    }
}
