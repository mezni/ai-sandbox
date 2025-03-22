mod errors;
mod config;

use crate::errors::AppError;
use crate::config::read_config;


#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Start");

    let config_file = "config.yaml";
    let config = read_config(config_file).unwrap();
/* 
    println!("Sleep duration seconds: {}", config.sleep_duration_seconds);

    for source in &config.sources {
        println!("Source type: {}", source.source_type);
        println!("Source directory: {}", source.source_directory);
        println!("File pattern: {}", source.file_pattern);
        println!("Post action: {}", source.post_action);
        if let Some(archive_directory) = &source.archive_directory {
            println!("Archive directory: {}", archive_directory);
        }
    }
*/
    Ok(())
}