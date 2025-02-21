// src/error.rs
use r2d2::Error as R2D2Error;
use rusqlite::Error as RusqliteError;
use sled::Error as SledError;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum AppError {
    QueueError(String),
    EventGenerationError(String),
    EventProcessingError(String),
    DatabaseError(SledError),
    RusqliteError(RusqliteError),
    R2D2Error(R2D2Error),
    IoError(IoError),
}

impl std::error::Error for AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::QueueError(msg) => write!(f, "Queue error: {}", msg),
            AppError::EventGenerationError(msg) => write!(f, "Event generation error: {}", msg),
            AppError::EventProcessingError(msg) => write!(f, "Event processing error: {}", msg),
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::RusqliteError(e) => write!(f, "Rusqlite error: {}", e),
            AppError::R2D2Error(e) => write!(f, "R2D2 error: {}", e),
            AppError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<SledError> for AppError {
    fn from(e: SledError) -> Self {
        AppError::DatabaseError(e)
    }
}

impl From<RusqliteError> for AppError {
    fn from(e: RusqliteError) -> Self {
        AppError::RusqliteError(e)
    }
}

impl From<R2D2Error> for AppError {
    fn from(e: R2D2Error) -> Self {
        AppError::R2D2Error(e)
    }
}

impl From<IoError> for AppError {
    fn from(e: IoError) -> Self {
        AppError::IoError(e)
    }
}
