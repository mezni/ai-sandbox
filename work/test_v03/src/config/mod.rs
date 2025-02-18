use serde::Deserialize;

#[derive( Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
}