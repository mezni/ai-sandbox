use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("YAML error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}