use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use rand::Rng;
use std::collections::HashMap;
use std::fmt;

/// Error type for MAC address conversion
#[derive(Debug)]
enum MacConversionError {
    InvalidCharacter(char),
    InvalidLength(usize),
}

impl fmt::Display for MacConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MacConversionError::InvalidCharacter(c) => write!(f, "Invalid character: {}", c),
            MacConversionError::InvalidLength(len) => write!(f, "Invalid length: {}", len),
        }
    }
}

/// Generates a random MAC address
fn generate_mac() -> String {
    let mut rng = rand::thread_rng();
    let mut mac = String::new();

    for i in 0..6 {
        let num = rng.gen::<u8>();
        mac.push_str(&format!("{:02x}", num));
        if i < 5 {
            mac.push(':');
        }
    }

    mac
}

/// Converts a MAC address to an integer
fn mac_to_int(mac: &str) -> Result<u64, MacConversionError> {
    let mac = mac.replace(":", "");
    if mac.len() != 12 {
        return Err(MacConversionError::InvalidLength(mac.len()));
    }

    let mut result = 0;
    for (i, c) in mac.chars().enumerate() {
        let digit = match c {
            '0'..='9' => c as u64 - '0' as u64,
            'a'..='f' => c as u64 - 'a' as u64 + 10,
            'A'..='F' => c as u64 - 'A' as u64 + 10,
            _ => return Err(MacConversionError::InvalidCharacter(c)),
        };
        result |= digit << (5 * (11 - i));
    }
    Ok(result)
}

/// Generates a random datetime
fn generate_datetime() -> DateTime<Utc> {
    let mut rng = rand::thread_rng();
    let year = rng.gen_range(2024..2025);
    let month = rng.gen_range(1..13);
    let day = rng.gen_range(1..=31);
    let hour = rng.gen_range(0..24);
    let minute = rng.gen_range(0..60);
    let second = rng.gen_range(0..60);

    let date = NaiveDate::from_ymd_opt(year, month, day).expect("Invalid date");
    let time = NaiveTime::from_hms_opt(hour, minute, second).expect("Invalid time");

    DateTime::from_utc(NaiveDateTime::new(date, time), Utc)
}

fn main() -> Result<(), MacConversionError> {
    let mut events: Vec<(u64, DateTime<Utc>)> = Vec::new();

    for _ in 0..10 {
        let mac = generate_mac();
        let mac_int = mac_to_int(&mac)?;
        events.push((mac_int, generate_datetime()));
        println!("Inserted MAC address: {}", mac);
    }
    events.sort_by_key(|(_, event_time)| *event_time);

    for (mac_int, event_time) in &events {
        println!("{}: {}", mac_int, event_time);
    }
    println!("size: {}", events.len());
    events.remove(0);
    println!("size: {}", events.len());    
    Ok(())
}
