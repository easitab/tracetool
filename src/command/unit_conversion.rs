use regex::Regex;

use crate::util::{parse_datetime_ceil, parse_datetime_floor};

lazy_static! {
    static ref DURATION_REGEX: Regex = Regex::new(r"^\s*(\d+)\s*([A-Za-z]+)\s*$").unwrap();
    static ref TIMESTAMP_REGEX: Regex = Regex::new(r"^\s*(\d+)\s*$").unwrap();
}

pub(crate) fn convert_unit(value: &str) {
    if let Ok(start) = parse_datetime_floor(value) {
        let end = parse_datetime_ceil(value).unwrap();
        if start == end {
            println!("{}", start.timestamp_nanos_opt().unwrap());
        } else {
            println!(
                "{} - {}",
                start.timestamp_nanos_opt().unwrap(),
                end.timestamp_nanos_opt().unwrap()
            );
            println!("({} - {})", start, end);
        }
        return;
    }

    if let Some(captures) = DURATION_REGEX.captures(value) {
        let quantity_str = captures.get(1).unwrap().as_str();
        let quantity = match quantity_str.parse::<u64>() {
            Ok(quantity) => quantity,
            Err(e) => {
                eprintln!("Cannot parse {}: {}", quantity_str, e);
                return;
            }
        };

        let unit_str = captures.get(2).unwrap().as_str();
        let unit_factor = match unit_str {
            "ns" | "nanoseconds" => 1u64,
            "us" | "microseconds" => 1_000u64,
            "ms" | "milliseconds" => 1_000_000u64,
            "s" | "seconds" => 1_000_000_000u64,
            "m" | "minutes" => 60u64 * 1_000_000_000u64,
            "h" | "hours" => 60u64 * 60u64 * 1_000_000_000u64,
            "D" | "days" => 24u64 * 60u64 * 60u64 * 1_000_000_000u64,
            "W" | "weeks" => 7u64 * 24u64 * 60u64 * 60u64 * 1_000_000_000u64,
            "M" | "months" => 30u64 * 24u64 * 60u64 * 60u64 * 1_000_000_000u64,
            "Y" | "years" => 365u64 * 24u64 * 60u64 * 60u64 * 1_000_000_000u64,
            _ => {
                eprintln!("Unknown unit {}", unit_str);
                return;
            }
        };

        if quantity > u64::MAX / unit_factor {
            eprintln!(
                "Quantity {} is too large for conversion to nanoseconds",
                quantity
            );
            return;
        }

        let nanoseconds = quantity * unit_factor;
        println!("{} nanoseconds", nanoseconds);
        return;
    }

    if let Some(captures) = TIMESTAMP_REGEX.captures(value) {
        let timestamp_str = captures.get(1).unwrap().as_str();
        let timestamp = match timestamp_str.parse::<u64>() {
            Ok(timestamp) => timestamp,
            Err(e) => {
                eprintln!("Cannot parse {}: {}", timestamp_str, e);
                return;
            }
        };

        let datetime = chrono::NaiveDateTime::from_timestamp_opt(
            (timestamp / 1_000_000_000) as i64,
            (timestamp % 1_000_000_000) as u32,
        )
        .unwrap();
        let datetime =
            chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(datetime, chrono::Utc);
        println!("{}", datetime);
        return;
    }

    eprintln!("Unable to parse {} as a timestamp or a duration", value);
}
