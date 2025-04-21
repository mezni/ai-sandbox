use crate::repo;
use core::config;
use core::db;
use core::errors::AppError;
use core::file;

const CONFIG_FILE: &str = "config.yaml";

pub struct LoadService {
    srv_config: config::ServerConfig,
    app_config: config::AppConfig,
    db_manager: db::DBManager,
    file_manager: file::FileManager,
}

impl LoadService {
    pub async fn new() -> Result<Self, AppError> {
        let srv_config = config::read_srv_config()?;
        srv_config.validate()?;

        let app_config = config::read_app_config(CONFIG_FILE)?;
        app_config.validate()?;

        let db_manager = db::DBManager::new(srv_config.clone())?;
        let file_manager = file::FileManager::new();

        Ok(LoadService {
            srv_config,
            app_config,
            db_manager,
            file_manager,
        })
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        if let Some(file) = self.file_manager.next(self.app_config.clone()).await? {
            let db_client = self.db_manager.get_client().await?;
            let batch_id = repo::insert_batch_exec(
                &db_client,
                "Loader-srv",
                &file.file_type,
                &file.file_path.to_string_lossy(),
            )
            .await?;
            if let Some(parsed) = self.file_manager.parse_file(file).await? {
                match parsed {
                    file::ParsedData::RoamIn(data) => {
                        let batch_date = data
                            .metadata
                            .as_ref()
                            .map(|m| m.creation_date[..10].to_string());

                        let records: Vec<repo::RoamInDataDBRecord> = data
                            .records
                            .into_iter()
                            .map(|record| repo::RoamInDataDBRecord {
                                batch_id,
                                batch_date: batch_date.clone(),
                                hlraddr: record.hlraddr,
                                nsub: record.nsub,
                                nsuba: record.nsuba,
                            })
                            .collect();

                        repo::insert_roam_in_stg_records(&db_client, records).await?;
                    }

                    file::ParsedData::RoamOut(data) => {
                        let batch_date = data
                            .metadata
                            .as_ref()
                            .map(|m| m.creation_date[..10].to_string());

                        let records: Vec<repo::RoamOutDataDBRecord> = data
                            .records
                            .into_iter()
                            .map(|record| repo::RoamOutDataDBRecord {
                                batch_id,
                                batch_date: batch_date.clone(),
                                imsi: record.imsi,
                                msisdn: record.msisdn,
                                vlr_number: record.vlr_number,
                            })
                            .collect();

                        repo::insert_roam_out_stg_records(&db_client, records).await?;
                    }
                }
            }
        }
        Ok(())
    }
}
