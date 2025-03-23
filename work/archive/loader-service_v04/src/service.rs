use crate::errors::AppError;
use tokio_postgres::{Client, NoTls};
use std::{fs, path::Path};

pub struct LoadService {
    client: Client,
}

impl LoadService {
    pub async fn new() -> Result<Self, AppError> {
        // Connect to PostgreSQL
        let (client, connection) =
            tokio_postgres::connect("host=localhost user=your_user password=your_password dbname=your_db", NoTls).await?;

        // Spawn a task to manage the connection
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(Self { client })
    }

    pub async fn process_files(&self, input_dir: &Path) -> Result<(), AppError> {
        if !input_dir.is_dir() {
            return Err(AppError::Custom(format!("{} is not a directory", input_dir.display())));
        }

        for entry in fs::read_dir(input_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "csv") {
                println!("Processing file: {}", path.display());
                self.process_file(&path).await?;
            }
        }

        Ok(())
    }

    async fn process_file(&self, path: &Path) -> Result<(), AppError> {
        let content = fs::read_to_string(path)?;
        let rows: Vec<&str> = content.lines().collect();

        for row in rows {
            let values: Vec<&str> = row.split(',').collect();
            if values.len() != 3 {
                println!("Invalid row: {}", row);
                continue;
            }

            // Example insert (assuming a table with 3 columns)
            self.client.execute(
                "INSERT INTO your_table (col1, col2, col3) VALUES ($1, $2, $3)",
                &[&values[0], &values[1], &values[2]],
            ).await?;
        }

        println!("File loaded to PostgreSQL: {}", path.display());
        fs::remove_file(path)?;
        println!("File deleted: {}", path.display());

        Ok(())
    }
}
