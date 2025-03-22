use std::collections::HashMap;
use std::{fs, path::Path};
use tokio::{fs::File, time::Duration};
use tokio_postgres::{Error, NoTls};
//use csv::ReaderBuilder;
//use serde::Deserialize;
use env_logger;
use log::{error, info};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    info!("Starting microservice...");
}
