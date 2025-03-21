use crate::errors::AppError;
use crate::repository::Repository;

use std::collections::HashMap;

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
