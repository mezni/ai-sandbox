mod errors;
//mod repository;
//mod utils;

use env_logger::init;
use log::{error, info};

use crate::errors::AppError; //, repository::Repository, utils::CarrierLookup};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    init();
    info!("Start");
    Ok(())
}
