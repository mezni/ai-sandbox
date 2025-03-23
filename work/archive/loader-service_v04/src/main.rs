fn main() {
    println!("Hello, world!");
}
mod errors;
mod service;

use crate::errors::AppError;
use crate::service::LoadService;
use tokio::time::{sleep, Duration};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Starting file processing daemon...");

    let service = LoadService::new().await?;

    loop {
        println!("Checking for new files...");
        service.process_files(Path::new("/data/input")).await?;

        println!("Sleeping for 10 seconds...");
        sleep(Duration::from_secs(10)).await;
    }
}
