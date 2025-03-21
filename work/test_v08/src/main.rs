mod errors;
mod repository;

use crate::errors::AppError;
use crate::repository::{PostgresClient, RoamOutDBRecord};
use env_logger::init;
use log::{error, info};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize logger
    init();
    info!("Starting the application...");

    // Create Postgres client
    let client = match PostgresClient::new().await {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to connect to the database: {}", e);
            return Err(e);
        }
    };

    // Prepare data to insert
    let stg_roam_outs = vec![
        RoamOutDBRecord { batch_id: 1, imsi: "123456789012345".to_string(), msisdn: "1234567890".to_string(), vlr_number: "1111".to_string() },
        RoamOutDBRecord { batch_id: 2, imsi: "234567890123456".to_string(), msisdn: "2345678901".to_string(), vlr_number: "2222".to_string() },
    ];

    // Perform the insertion
    match client.insert_stg_roam_out(stg_roam_outs).await {
        Ok(_) => info!("Records inserted successfully."),
        Err(e) => error!("Failed to insert records: {}", e),
    }

    info!("Application finished.");
    Ok(())
}
