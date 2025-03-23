use crate::AppError;
use crate::config::Config;
use crate::store;
use log::{error, info};
use regex::Regex;
use std::fs;

pub struct LoadService {
    config: Config,
}

impl LoadService {
    pub async fn new(config: Config) -> Result<Self, AppError> {
        Ok(LoadService { config })
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        for source in &self.config.sources {
            let file_pattern_regex = Regex::new(&source.file_pattern)
                .map_err(|err| AppError::Unexpected(format!("Invalid file pattern: {}", err)))?;

            let files = fs::read_dir(&source.source_directory).map_err(AppError::IoError)?;

            for file in files.flatten() {
                let file_name = file.file_name();
                let file_name_str = file_name.to_string_lossy();

                if file_pattern_regex.is_match(&file_name_str) {
                    info!("Processing file: {}", file.path().display());

                    let source_type = &source.source_type;
                    let post_action = &source.post_action;
                    let archive_directory = source.archive_directory.as_deref().unwrap_or("");

                    if let Err(err) = self
                        .process_file(file.path(), source_type, post_action, archive_directory)
                        .await
                    {
                        error!("Failed to process file {}: {}", file.path().display(), err);
                    }
                }
            }
        }
        Ok(())
    }

    async fn process_file(
        &self,
        path: std::path::PathBuf,
        source_type: &str,
        post_action: &str,
        archive_directory: &str,
    ) -> Result<(), AppError> {
        let client = store::connection().await?;
        let batch_id = store::insert_batch_execs(&client, path.display().to_string()).await?;

        store::update_batch_execs(&client, batch_id, "Completed").await?;

        println!("File processed with Batch ID: {}", batch_id);

        Ok(())
    }
}
