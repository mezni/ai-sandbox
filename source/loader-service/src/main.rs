use core::errors::AppError;
use core::logger::Logger;
use core::file::FileManager;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    Logger::init();
    Logger::info("Start process");
    
    Logger::info("Stop process");
    Ok(())
}
