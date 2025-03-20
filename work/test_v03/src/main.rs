use regex::Regex;

fn main() {
    let filename = "HSS9860_1513_20250307002600.txt";

    // Define regex to capture timestamp (14 digits) using a group
    let re = Regex::new(r"_(\d{14})\.txt$").unwrap();

    if let Some(captures) = re.captures(filename) {
        let timestamp = &captures[1]; // Extract the first capture group
        let date = &timestamp[0..8];  // First 8 characters as date (YYYYMMDD)

        println!("Full Timestamp: {}", timestamp);
        println!("Date (YYYYMMDD): {}", date);
    } else {
        println!("No valid timestamp found in the filename.");
    }
}
