mod errors;
mod repository;
mod utils;

use crate::{errors::AppError, repository::Repository, utils::CarrierLookup};

use csv::{ReaderBuilder, Trim};
use env_logger::init;
use log::{error, info};
use std::{
    fs::File,
    io::{self, BufReader},
};

#[derive(Debug)]
struct RoamOutRecord {
    imsi: String,
    msisdn: String,
    vlr_number: String,
}

#[derive(Debug)]
struct RoamOutDBRecord {
    imsi: String,
    msisdn: String,
    vlr_number: String,
    carrier_name: String,
    country_name: String,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    init();
    info!("Start");

    let repository = Repository::new().await?;
    log::info!("Connected to the database.");
    let carrier_lookup = CarrierLookup::new(&repository).await?;

    let file_path = "/home/dali/WORK/DATA/HSS9860_1513_20250307002600.txt";
    let records = read_csv_file(file_path)?;

    for record in records {
        let vlr_number = record.vlr_number;
        let carrier_record = carrier_lookup.lookup(vlr_number.clone());

        println!(
            "{} {} {}",
            vlr_number, carrier_record.country_name, carrier_record.carrier_name
        );
    }

    Ok(())
}

fn read_csv_file(file_path: &str) -> Result<Vec<RoamOutRecord>, AppError> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(BufReader::new(file));

    let mut records = Vec::new();
    for result in reader.records() {
        let record = result?;
        let imsi = record[0].to_string();
        let msisdn = record[1].to_string();
        let vlr_number = record[2].to_string();
        records.push(RoamOutRecord {
            imsi: imsi,
            msisdn: msisdn,
            vlr_number: vlr_number,
        });
    }

    Ok(records)
}
