use std::io::Error as IoError;
use std::io::ErrorKind;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use regex::Regex;

use crate::util::Result;

lazy_static! {
    static ref DATE_REGEX: Regex = Regex::new(
        r"(?x)
        ^(?P<year>\d{4})
        (?:-(?P<month>\d{2})
            (?:-(?P<day>\d{2})
                (?:\s(?P<hour>\d{2})
                    (?::(?P<minute>\d{2})
                        (?::(?P<second>\d{2}))?
                    )?
                )?
            )?
        )?$",
    )
    .unwrap();
}

/// Parse a date and time string into a `DateTime<Utc>`. The date and time string must be in the
/// format `YYYY-MM-DD HH:MM:SS`. Any level of precision is allowed, and any missing fields will be
/// filled in with the minimum value for that field.
pub(crate) fn parse_datetime_floor(date_str: &str) -> Result<DateTime<Utc>> {
    parse_datetime_arbitrary_precision(date_str, &[1, 1, 0, 0, 0])
}

/// Parse a date and time string into a `DateTime<Utc>`. The date and time string must be in the
/// format `YYYY-MM-DD HH:MM:SS`. Any level of precision is allowed, and any missing fields will be
/// filled in with the maximum value for that field.
pub(crate) fn parse_datetime_ceil(date_str: &str) -> Result<DateTime<Utc>> {
    parse_datetime_arbitrary_precision(date_str, &[12, 31, 23, 59, 59])
}

const DATE_PART_NAMES: [&str; 5] = ["month", "day", "hour", "minute", "second"];

fn parse_datetime_arbitrary_precision(
    date_str: &str,
    defaults: &[u32; 5],
) -> Result<DateTime<Utc>> {
    let m = DATE_REGEX
        .captures(date_str)
        .ok_or_else(|| IoError::new(ErrorKind::InvalidData, "Invalid date"))?;
    let year: i32 = m
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .map_err(|_| IoError::new(ErrorKind::InvalidData, "Invalid year"))?;
    let mut values = *defaults;
    for i in 0..5 {
        let value = match m.get(i + 2) {
            Some(value) => value,
            None => break,
        };
        let value = value.as_str().parse().map_err(|_| {
            IoError::new(
                ErrorKind::InvalidData,
                format!("Invalid {}", DATE_PART_NAMES[i]),
            )
        })?;
        values[i] = value;
    }

    let date = NaiveDate::from_ymd_opt(year, values[0], values[1]).map_or_else(
        || Err(IoError::new(ErrorKind::InvalidData, "Invalid date")),
        Ok,
    )?;
    let time = NaiveTime::from_hms_opt(values[2], values[3], values[4]).map_or_else(
        || Err(IoError::new(ErrorKind::InvalidData, "Invalid time")),
        Ok,
    )?;

    Ok(Utc.from_utc_datetime(&NaiveDateTime::new(date, time)))
}
