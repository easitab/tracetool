use crate::config::TimeUnit;
use crate::util::TypeCast;

/// Convert time values from nanoseconds to milliseconds.
///
/// # Arguments
///
/// * `nanoseconds` - A vector of `i64` values, where each value represents a time in nanoseconds
/// from the Unix epoch.
///
/// # Returns
///
/// * A vector of `f64` values, where each value represents a time in milliseconds from the epoch.
pub(crate) fn nanoseconds_epoch_to_plotly_time(nanoseconds: &[i64]) -> Vec<f64> {
    // Plotly uses milliseconds as the unit for time when plotting
    // with f64 values.
    nanoseconds
        .iter()
        .map(|x| *x as f64 / 1_000_000.0)
        .collect()
}

/// Convert time values from nanoseconds to seconds.
///
/// # Arguments
///
/// * `nanoseconds` - A vector of `i64` values, where each value represents a time in
/// nanoseconds from the Unix epoch.
///
/// # Returns
///
/// * A vector of `f64` values, where each value represents a time in seconds from the epoch.
pub(crate) fn nanoseconds_duration_to_seconds<T>(duration: &[T]) -> Vec<f64>
where
    T: TypeCast<f64>,
{
    duration
        .iter()
        .map(|x| x.cast() / 1_000_000_000.0)
        .collect()
}

/// Convert time values from nanoseconds to a specified unit.
///
/// # Arguments
///
/// * `duration` - A vector of `T` values, where each value represents a time in nanoseconds.
/// * `unit` - The unit to convert the time values to. If `None`, the time values are
/// converted to seconds.
///
/// # Returns
///
/// * A vector of `f64` values, where each value represents a time in the specified unit.
pub(crate) fn nanoseconds_duration_to_unit<T>(duration: &[T], unit: Option<TimeUnit>) -> Vec<f64>
where
    T: TypeCast<f64>,
{
    let factor = get_unit_denominator(unit);
    duration.iter().map(|x| x.cast() / factor).collect()
}

/// Get the denominator for converting nanosecond time values to a specified unit.
fn get_unit_denominator(unit: Option<TimeUnit>) -> f64 {
    match unit {
        Some(unit) => match unit {
            TimeUnit::Hours => 3_600_000_000_000.0,
            TimeUnit::Minutes => 60_000_000_000.0,
            TimeUnit::Seconds => 1_000_000_000.0,
            TimeUnit::Milliseconds => 1_000_000.0,
            TimeUnit::Microseconds => 1_000.0,
            TimeUnit::Nanoseconds => 1.0,
            _ => panic!("Unsupported unit"),
        },
        None => 1_000_000_000.0,
    }
}
