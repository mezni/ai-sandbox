use env_logger;
use log::{error, info};
mod errors;
mod repository;
mod utils;
use errors::AppError;
use crate::repository::Repository;
use crate::utils::CarrierLookup;

#[tokio::main]
async fn main() -> Result<(), AppError> {

    env_logger::init();
    info!("Start");

    let repository = Repository::new().await?;
    log::info!("Connected to the database.");
//    let carrier_map = get_lookup(&repository).await?; 
//    println!("{:?}", carrier_map);
let carrier_lookup = CarrierLookup::new().await?;
let carrier_record = carrier_lookup.lookup("331067003566".to_string());

println!("Carrier name: {}", carrier_record.carrier_name);
println!("Country name: {}", carrier_record.country_name);


    Ok(())
}





