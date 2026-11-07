//! Typed application configuration loaded from the environment.

use std::env;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub jwt_access_ttl: i64,
    pub jwt_refresh_ttl: i64,
    pub argon2_memory: u32,
    pub argon2_time: u32,
    pub argon2_threads: u32,
    pub password_min_length: usize,
    pub max_db_connections: u32,
    pub environment: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("missing required environment variable: {0}")]
    Missing(&'static str),
    #[error("invalid value for {0}: {1}")]
    Invalid(&'static str, String),
}

fn parse_env<T>(key: &'static str, value: String) -> Result<T, ConfigError>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    value
        .parse()
        .map_err(|err| ConfigError::Invalid(key, format!("{err}")))
}

impl Config {
    /// Load configuration from environment variables, failing fast on missing secrets.
    pub fn from_env() -> Result<Self, ConfigError> {
        let environment = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

        let jwt_secret = env::var("JWT_SECRET").map_err(|_| ConfigError::Missing("JWT_SECRET"))?;
        if environment == "production" && jwt_secret.len() < 32 {
            return Err(ConfigError::Invalid(
                "JWT_SECRET",
                "must be at least 32 bytes in production".to_string(),
            ));
        }

        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .map_err(|_| ConfigError::Missing("DATABASE_URL"))?,
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: parse_env(
                "PORT",
                env::var("PORT").unwrap_or_else(|_| "8080".to_string()),
            )?,
            jwt_secret,
            jwt_access_ttl: parse_env(
                "JWT_ACCESS_TTL",
                env::var("JWT_ACCESS_TTL").unwrap_or_else(|_| "900".to_string()),
            )?,
            jwt_refresh_ttl: parse_env(
                "JWT_REFRESH_TTL",
                env::var("JWT_REFRESH_TTL").unwrap_or_else(|_| "2592000".to_string()),
            )?,
            argon2_memory: parse_env(
                "ARGON2ID_MEMORY",
                env::var("ARGON2ID_MEMORY").unwrap_or_else(|_| "65536".to_string()),
            )?,
            argon2_time: parse_env(
                "ARGON2_TIME",
                env::var("ARGON2_TIME").unwrap_or_else(|_| "3".to_string()),
            )?,
            argon2_threads: parse_env(
                "ARGON2ID_THREADS",
                env::var("ARGON2ID_THREADS").unwrap_or_else(|_| "4".to_string()),
            )?,
            password_min_length: parse_env(
                "PASSWORD_MIN_LENGTH",
                env::var("PASSWORD_MIN_LENGTH").unwrap_or_else(|_| "12".to_string()),
            )?,
            max_db_connections: parse_env(
                "MAX_DB_CONNECTIONS",
                env::var("MAX_DB_CONNECTIONS").unwrap_or_else(|_| "20".to_string()),
            )?,
            environment,
        })
    }

    /// Deterministic configuration used by tests.
    pub fn for_testing(database_url: impl Into<String>) -> Self {
        Self {
            database_url: database_url.into(),
            host: "127.0.0.1".to_string(),
            port: 8080,
            jwt_secret: "test-secret-test-secret-test-secret-123456".to_string(),
            jwt_access_ttl: 900,
            jwt_refresh_ttl: 2_592_000,
            argon2_memory: 19_456,
            argon2_time: 2,
            argon2_threads: 1,
            password_min_length: 12,
            max_db_connections: 5,
            environment: "test".to_string(),
        }
    }
}
