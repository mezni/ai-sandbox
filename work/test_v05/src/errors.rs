use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] tokio_postgres::Error),

    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
