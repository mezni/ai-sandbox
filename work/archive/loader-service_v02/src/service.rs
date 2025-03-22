use crate::config::Config;
use crate::AppError;

pub struct LoadService {}

impl LoadService {
    pub async fn new(config: &Config) -> Result<Self, AppError> {
        for source in &config.sources {
            println!("Source directory: {}", source.source_directory);
        }

        Ok(Self {})
    }
}
