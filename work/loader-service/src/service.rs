use crate::AppError;
use crate::config::Config;
use crate::store;
use log::{error, info};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tokio_postgres::Row;

use csv::{ReaderBuilder, Trim};
use std::{
    fs::File,
    io::{self, BufReader},
};

#[derive(Debug)]
pub struct CarrierRecord {
    pub carrier_name: String,
    pub country_name: String,
}

#[derive(Debug)]
struct RoamOutRecord {
    imsi: String,
    msisdn: String,
    vlr_number: String,
}

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

        let carrier_map = get_carriers_map().await?;

        let path_str = path.display().to_string();
        let db_records = convert_to_db_records(&path_str, &carrier_map, batch_id)?;

        if let Err(err) = store::insert_roam_out_stg(&client, db_records).await {
            error!("Failed to insert records into database: {}", err);
        } else {
            info!("Records successfully inserted into database.");           
        }

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

pub async fn get_carriers_map()
-> Result<HashMap<String, (String, String, String, String, String)>, AppError> {
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
            (
                carrier_name,
                carrier_id,
                country_name,
                country_code,
                national_destination_code,
            ),
        );
    }

    Ok(carrier_map)
}

pub fn lookup(
    carrier_map: &HashMap<String, (String, String, String, String, String)>,
    mut s: String,
) -> CarrierRecord {
    loop {
        if let Some(carrier_info) = carrier_map.get(&s) {
            return CarrierRecord {
                carrier_name: carrier_info.0.clone(),
                country_name: carrier_info.2.clone(),
            };
        } else if s.is_empty() {
            return CarrierRecord {
                carrier_name: "".to_string(),
                country_name: "".to_string(),
            };
        } else {
            s.pop(); // Remove the last character
        }
    }
}

pub fn read_csv_file(file_path: &str) -> Result<Vec<RoamOutRecord>, AppError> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(BufReader::new(file));

    let mut records = Vec::new();
    for result in reader.records() {
        let record = result?;
        let imsi = record[0].to_string();
        let msisdn = record[1].to_string();
        let vlr_number = record[2].to_string();
        records.push(RoamOutRecord {
            imsi: imsi,
            msisdn: msisdn,
            vlr_number: vlr_number,
        });
    }

    Ok(records)
}

pub fn convert_to_db_records(
    path: &str,
    carrier_map: &HashMap<String, (String, String, String, String, String)>,
    batch_id: i32,
) -> Result<Vec<store::RoamOutDBRecord>, AppError> {
    let records = read_csv_file(path)?;

    let mut db_records = Vec::new();
    for record in records {
        let carrier_record = lookup(carrier_map, record.vlr_number.clone());

        let db_record = store::RoamOutDBRecord {
            batch_id,
            batch_date: "2025-03-23".to_string(),
            imsi: record.imsi,
            msisdn: record.msisdn,
            vlr_number: record.vlr_number,
            carrier_name: carrier_record.carrier_name,
            country_name: carrier_record.country_name,
        };

        db_records.push(db_record);
    }

    Ok(db_records)
}
