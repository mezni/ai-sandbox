mod errors;
mod service;

use crate::errors::AppError;
use crate::service::LoadService;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SourceFile {
    file_path: String,
    file_type: String,
    file_pattern: String,
    post_action: String,
    archive_dir: Option<String>,    
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Start");
    let source_file = SourceFile {
        file_path: "/data/input/file.csv".to_string(),
        file_type: "roam_in".to_string(),
        file_pattern: "*.csv".to_string(),
        post_action: "archive".to_string(),
        archive_dir: Some("/data/archive".to_string()),
    };

    println!("Source File: {:?}", source_file);


    //    let service = LoadService::new().await?;

    Ok(())
}
