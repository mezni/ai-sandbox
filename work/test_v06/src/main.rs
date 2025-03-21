use env_logger;
use log::{error, info};
mod errors;
mod repository;
mod utils;
use crate::repository::Repository;
use errors::AppError;

use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    env_logger::init();
    info!("Start");

    let repository = Repository::new().await?;
    log::info!("Connected to the database.");
    let carrier_map = get_lookup(&repository).await?;
    println!("{:?}", carrier_map);

    Ok(())
}

async fn get_lookup(
    repository: &Repository,
) -> Result<HashMap<String, (String, String, String, String, String)>, AppError> {
    let mut carrier_map: HashMap<String, (String, String, String, String, String)> = HashMap::new();
    let carriers = repository.select_carriers().await?;
    for row in carriers {
        let country_name: String = row.get("country_name");
        let carrier_id: String = row.get("carrier_id");
        let carrier_name: String = row.get("carrier_name");
        let country_code: String = row.get("country_code");
        let ndc: String = row.get("national_destination_code");
        let key: String = row.get("code");
        carrier_map.insert(
            key,
            (carrier_name, carrier_id, country_name, country_code, ndc),
        );
    }

    Ok(carrier_map)
}
