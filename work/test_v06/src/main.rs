use env_logger;
use log::{error, info};
mod errors;
mod repository;
mod utils;
use errors::AppError;
use crate::repository::Repository;
use crate::utils::get_lookup;

#[tokio::main]
async fn main() -> Result<(), AppError> {

    env_logger::init();
    info!("Start");

    let repository = Repository::new().await?;
    log::info!("Connected to the database.");
    let carrier_map = get_lookup(&repository).await?; 
//    println!("{:?}", carrier_map);


    let mut s = String::from("331067003566");
    loop {
        if carrier_map.contains_key(s.as_str()) {
            println!("Match found: {}", s);
            break;
        } else if s.len() == 0 {
            println!("No match found");
            break;
        } else {
            s.pop();
        }
    }


    Ok(())
}







