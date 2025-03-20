use std::{fs, path::Path};
use std::collections::HashMap;
use tokio::{time::Duration, fs::File};
use tokio_postgres::{NoTls, Error};
//use csv::ReaderBuilder;
//use serde::Deserialize;
use log::{info, error};
use env_logger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    info!("Starting microservice...");
}