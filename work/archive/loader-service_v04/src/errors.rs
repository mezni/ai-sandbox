use std::fmt;
use std::io;
use tokio_postgres::Error as PostgresError;

#[derive(Debug)]
pub enum AppError {
    IoError(io::Error),
    PostgresError(PostgresError),
    Custom(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::IoError(err) => write!(f, "IO Error: {}", err),
            AppError::PostgresError(err) => write!(f, "PostgreSQL Error: {}", err),
            AppError::Custom(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::IoError(err)
    }
}

impl From<PostgresError> for AppError {
    fn from(err: PostgresError) -> Self {
        AppError::PostgresError(err)
    }
}
