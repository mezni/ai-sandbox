use crate::AppError;
use crate::config::Config;
use crate::store;
use log::{error, info};
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use tokio_postgres::Row;

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

        match post_action {
            "delete" => {
                fs::remove_file(&path).map_err(AppError::IoError)?;
                info!("File deleted: {}", path.display());
            }
            "archive" if !archive_directory.is_empty() => {
                let archive_path =
                    PathBuf::from(archive_directory).join(path.file_name().ok_or_else(|| {
                        AppError::Unexpected(format!(
                            "Failed to get file name for: {}",
                            path.display()
                        ))
                    })?);

                fs::rename(&path, &archive_path).map_err(AppError::IoError)?;
                info!("File moved to archive: {}", archive_path.display());
            }
            _ => {
                error!(
                    "Invalid post_action: {}. Supported actions: delete, archive.",
                    post_action
                );
                return Err(AppError::Unexpected(format!(
                    "Invalid post_action: {}",
                    post_action
                )));
            }
        }

        println!("File processed with Batch ID: {}", batch_id);

        Ok(())
    }
}


pub async fn get_carriers_map() -> Result<HashMap<String, (String, String, String, String, String)>, AppError> {
    let mut carrier_map = HashMap::new();
    let client = store::connection().await?;

    let carriers = store::select_all_carriers(&client).await?;

    for row in carriers {
        let carrier_key: String = row.get("code");
        let carrier_name: String = row.get("carrier_name");
        let carrier_id: String = row.get("carrier_id");
        let country_name: String = row.get("country_name");
        let country_code: String = row.get("country_code");
        let national_destination_code: String = row.get("national_destination_code");

        carrier_map.insert(
            carrier_key,
            (carrier_name, carrier_id, country_name, country_code, national_destination_code),
        );
    }

    Ok(carrier_map)
}