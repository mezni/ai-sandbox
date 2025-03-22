use crate::AppError;

pub struct LoadService {}

impl LoadService {
    pub async fn new() -> Result<Self, AppError> {
        println!("Load service");

        Ok(Self {})
    }
}
