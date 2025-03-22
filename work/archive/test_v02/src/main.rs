use std::collections::HashMap;
use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Connect to PostgreSQL
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=myuser password=mypassword dbname=roamdb",
        NoTls,
    )
    .await?;

    // Spawn the connection task to keep it running in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Perform the query
    let query = "
        WITH CTE AS (
        select id, carrier_id , carrier_name , country_name , country_code , national_destination_code, country_code || national_destination_code as code,
        ROW_NUMBER() OVER (PARTITION BY country_code || national_destination_code ORDER BY id DESC) AS rn
        from dim_carriers
        where national_destination_code is not null
        )
        SELECT country_name , carrier_id , carrier_name ,  country_code , national_destination_code,  code
        FROM CTE WHERE rn = 1
        ORDER by country_name,carrier_id    
    ";

    let rows = client.query(query, &[]).await?;

    // Create a HashMap to store results: id as key and tuple as value
    let mut carrier_map: HashMap<String, (String, String, String, String, String)> = HashMap::new();

    // Iterate over the rows and insert into the HashMap
    for row in rows {
        let country_name: String = row.get("country_name");
        let carrier_id: String = row.get("carrier_id");
        let carrier_name: String = row.get("carrier_name");
        let country_code: String = row.get("country_code");
        let ndc: String = row.get("national_destination_code");
        let code: String = row.get("code");

        // Insert data into HashMap
        carrier_map.insert(
            code,
            (carrier_name, carrier_id, country_name, country_code, ndc),
        );
    }

    // Print the HashMap
    println!("Carrier Data Stored in HashMap:");
    for (code, (carrier_name, carrier_id, country_name, country_code, ndc)) in &carrier_map {
        println!(
            "{} {} {} {} {} {}",
            code, country_name, carrier_id, carrier_name, country_code, ndc
        );
    }

    Ok(())
}
