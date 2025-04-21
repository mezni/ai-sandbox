use crate::config::AppConfig;
use crate::errors::AppError;
use crate::logger::Logger;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub struct FileManager;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ManagedFile {
    pub file_path: PathBuf,
    pub file_type: String,
    pub file_action: String,
    pub archive_path: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
struct SubscriberRecord {
    hlraddr: String,
    nsub: u64,
    nsuba: u64,
}

#[derive(Debug, Serialize)]
struct Summary {
    totnsub: u64,
    totnsuba: u64,
    nsubpr: u64,
    nsubxp: u64,
    nsubpxou: u64,
    nsubsgs: u64,
    nsubgs: u64,
}

#[derive(Debug, Serialize)]
struct Metadata {
    date: String,
    hour: String,
}

#[derive(Debug, Serialize)]
pub struct RoamInData {
    metadata: Option<Metadata>,
    records: Vec<SubscriberRecord>,
}

impl FileManager {
    pub fn new() -> Self {
        FileManager
    }

    pub async fn next(&self, app_config: AppConfig) -> Result<Option<ManagedFile>, AppError> {
        for source in app_config.sources {
            let source_type = source.source_type.to_uppercase();
            let path = Path::new(&source.source_directory);

            if source_type == "ROAM_IN" || source_type == "ROAM_OUT" {
                if path.exists() {
                    let mut files_vec: Vec<_> = fs::read_dir(&path)
                        .ok()
                        .into_iter()
                        .flat_map(|read_dir| read_dir.flatten())
                        .collect();

                    if let Some(pattern) = &source.file_pattern {
                        if let Ok(regex) = Regex::new(pattern) {
                            files_vec.retain(|entry| {
                                entry
                                    .path()
                                    .file_name()
                                    .and_then(OsStr::to_str)
                                    .map(|name| regex.is_match(name))
                                    .unwrap_or(false)
                            });
                        }
                    }

                    files_vec.sort_by_key(|entry| entry.file_name());

                    if let Some(first_file) = files_vec.first() {
                        let post_action_upper = source
                            .post_action
                            .as_ref()
                            .map(|a| a.to_uppercase())
                            .unwrap_or_else(|| "DELETE".to_string());

                        let file_action = if post_action_upper == "ARCHIVE" {
                            "ARCHIVE".to_string()
                        } else {
                            "DELETE".to_string()
                        };

                        let archive_path = source.archive_directory.as_ref().map(PathBuf::from);

                        let managed_file = ManagedFile {
                            file_path: first_file.path(),
                            file_type: source_type,
                            file_action,
                            archive_path,
                        };

                        return Ok(Some(managed_file));
                    }
                }
            }
        }

        Ok(None)
    }

    pub async fn execute(&self, app_config: AppConfig) -> Result<Option<RoamInData>, AppError> {
        if let Some(managed_file) = self.next(app_config).await? {
            match managed_file.file_type.as_str() {
                "ROAM_IN" => {
                    let result = self.roam_in_parser(managed_file).await?;
                    Ok(Some(result))
                }
                "ROAM_OUT" => {
                    self.roam_out_parser(managed_file).await?;
                    Ok(None)
                }
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub async fn roam_in_parser(&self, managed_file: ManagedFile) -> Result<RoamInData, AppError> {
        let file = std::fs::File::open(managed_file.file_path)?;
        let reader = BufReader::new(file);

        let re_row = Regex::new(r"(4-\d+)\s+(\d+)\s+(\d+)")?;
        let re_summary = Regex::new(r"([A-Z]+)\s+(\d+)")?;

        let mut metadata = None;
        let mut in_data_section = false;
        let mut records = Vec::new();
        let mut summary = Summary {
            totnsub: 0,
            totnsuba: 0,
            nsubpr: 0,
            nsubxp: 0,
            nsubpxou: 0,
            nsubsgs: 0,
            nsubgs: 0,
        };

        for line in reader.lines() {
            let line = line?;

            // Extract metadata line: ACT <something> TIME <date> <hour> <cluster>
            if line.starts_with("ACT") && line.contains("TIME") {
                let parts: Vec<&str> = line.split_whitespace().collect();

                // Ensure there are enough parts before accessing the date and hour
                if parts.len() > 6 {
                    // Extract the date and hour (ignore "CLUSTER" part)
                    if let (Some(date), Some(hour)) = (parts.get(4), parts.get(5)) {
                        // Make sure the date and hour values are correct and meaningful
                        metadata = Some(Metadata {
                            date: date.to_string(), // "250306"
                            hour: hour.to_string(), // "1324"
                        });
                    } else {
                        // Log a warning if we couldn't parse date and hour correctly
                        println!(
                            "Warning: Could not parse date and hour from metadata line: {}",
                            line
                        );
                    }
                }
                continue;
            }

            // Detect start of data section
            if line.contains("MT MOBILE SUBSCRIBER SURVEY RESULT") {
                in_data_section = true;
                continue;
            }

            if in_data_section {
                // Match data rows
                for caps in re_row.captures_iter(&line) {
                    records.push(SubscriberRecord {
                        hlraddr: caps[1].to_string(),
                        nsub: caps[2].parse()?,
                        nsuba: caps[3].parse()?,
                    });
                }

                // Match summary lines
                if let Some(caps) = re_summary.captures(&line) {
                    let key = &caps[1];
                    let value: u64 = caps[2].parse()?;
                    match key {
                        "TOTNSUB" => summary.totnsub = value,
                        "TOTNSUBA" => summary.totnsuba = value,
                        "NSUBPR" => summary.nsubpr = value,
                        "NSUBXP" => summary.nsubxp = value,
                        "NSUBPXOU" => summary.nsubpxou = value,
                        "NSUBSGS" => summary.nsubsgs = value,
                        "NSUBGS" => summary.nsubgs = value,
                        _ => {}
                    }
                }
            }
        }

        //   if let Some(meta) = metadata {
        //       println!("Metadata:\n{:#?}", meta);
        //  }

        // Optionally, print parsed records and summary if necessary
        // println!("\nParsed records:\n{:#?}", records);
        //    println!("\nSummary:\n{:#?}", summary);

        Ok(RoamInData { metadata, records })
    }

    pub async fn roam_out_parser(&self, managed_file: ManagedFile) -> Result<(), AppError> {
        println!("Parsing ROAM_OUT: {:?}", managed_file.file_path);
        Ok(())
    }
}
