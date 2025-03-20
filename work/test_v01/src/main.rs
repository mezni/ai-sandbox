use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use csv::ReaderBuilder;

#[derive(Debug)]
struct Record {
    imsi: String,
    msisdn: String,
    vlr_number: String,
}

/// Reads the contents of a file asynchronously.
async fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(file_path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    Ok(String::from_utf8(buffer)?)
}

/// Parses CSV data into a vector of `Record` structs.
fn parse_csv(csv_data: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_data.as_bytes());

    let mut records = Vec::new();
    for result in rdr.records() {
        let record = result?;
        records.push(Record {
            imsi: record[0].to_string(),
            msisdn: record[1].to_string(),
            vlr_number: record[2].to_string(),
        });
    }
    Ok(records)
}

/// Prints the parsed records.
fn print_records(records: &[Record]) {
    for record in records {
        println!("{:?}", record);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "HSS9860_1513_20250307002600.txt"; // Replace with your file path

    // Read the file asynchronously
    let csv_data = read_file(file_path).await?;

    // Parse the CSV data
    let records = parse_csv(&csv_data)?;

    // Print the parsed records
    print_records(&records);

    Ok(())
}