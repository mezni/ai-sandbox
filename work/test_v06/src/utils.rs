use crate::errors::AppError;
use crate::repository::Repository;

use std::collections::HashMap;

pub struct CarrierLookup {
    carrier_map: HashMap<String, (String, String, String, String, String)>,
}

pub struct CarrierRecord {
    carrier_name: String,
    country_name: String,
}

impl CarrierLookup {
    pub async fn new() -> Result<Self, AppError> {
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
    
        Ok(CarrierLookup { carrier_map })
    }

    pub fn lookup(&self, mut s: String) -> CarrierRecord {
        loop {
            if let Some(carrier_info) = self.carrier_map.get(s.as_str()) {
                return CarrierRecord{
                    carrier_name: carrier_info.0.clone(),
                    country_name: carrier_info.2.clone(),
                }
            } else if s.len() == 0 {
                return CarrierRecord{
                    carrier_name: "".to_string(),
                    country_name: "".to_string(),
                }
            } else {
                s.pop();
            }
        }
    }
}