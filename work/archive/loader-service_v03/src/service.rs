use std::{fs, path::Path};

use crate::AppError;

pub struct LoadService {}

impl LoadService {
    pub async fn new() -> Result<Self, AppError> {
        println!("Load service");

        Ok(Self {})
    }

    pub async fn load(&self, file_path: &str ) -> Result<(), AppError> {
       
        Ok(())
    }
}
