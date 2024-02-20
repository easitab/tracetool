use chrono::{Datelike, TimeZone, Timelike};

use crate::config::Filter;

/// Filter data points that are not within work hours, if the filter configuration specifies that
/// work hours should be used. Work hours are defined as 08 to 17 on weekdays.
///
/// # Arguments
/// * `timestamps` - A vector of `i64` values, where each value represents a time in nanoseconds
/// from the Unix epoch.
/// * `data` - A vector of `T` values, where each value represents a data point.
/// * `filter` - An optional `Filter` object that contains the filter configuration.
pub(crate) fn apply_workday_filter<T: Copy>(
    timestamps: Vec<i64>,
    data: Vec<T>,
    filter: Option<&Filter>,
) -> (Vec<i64>, Vec<T>) {
    let filter_work_hours = match filter {
        Some(filter) => filter.work_hours.unwrap_or(false),
        None => false,
    };
    if !filter_work_hours {
        return (timestamps, data);
    }

    let mut result_timestamps = Vec::with_capacity(timestamps.len());
    let mut result_data = Vec::with_capacity(data.len());
    for i in 0..timestamps.len() {
        let timestamp = timestamps[i];
        let data = data[i];
        if is_work_hours(timestamp) {
            result_timestamps.push(timestamp);
            result_data.push(data);
        }
    }
    (result_timestamps, result_data)
}

/// Determine if a timestamp is within work hours. Work hours are defined as 08 to 17 on weekdays,
/// local time.
///
/// # Arguments
/// * `timestamp` - A `i64` value, where the value represents a time in nanoseconds from the Unix
/// epoch.
fn is_work_hours(timestamp: i64) -> bool {
    // Work hours are 08 to 17 on weekdays
    let local_time = chrono::Local.timestamp_nanos(timestamp);
    let hour = local_time.hour();
    let weekday = local_time.weekday();
    (8..17).contains(&hour) && weekday != chrono::Weekday::Sat && weekday != chrono::Weekday::Sun
}
