use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    
    #[error("Missing configuration file: {0}")]
    MissingConfigFile(String),
}