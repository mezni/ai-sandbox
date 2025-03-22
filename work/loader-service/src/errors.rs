use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] tokio_postgres::Error),

    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("CSV error: {0}")]
    CsvError(#[from] csv::Error),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Unexpected(format!("IO error: {}", err))
    }
}
