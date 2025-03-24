mod errors;
mod logger;

use errors::AppError;
use logger::Logger;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    Logger::init();
    Logger::info("Start Service");
    Logger::info("Stop Service");
    Ok(())
}