use std::fmt::Debug;

use crate::config;
use crate::config::{AggregationMode, TimeUnit};
use crate::util::TypeCast;

/// Given a set of x and y values, group the data into bins based on the x values.
/// The x values are assumed to be sorted by the bin index that they map to.
///
/// # Arguments
/// * `x` - A vector of x values.
/// * `y` - A vector of y values.
/// * `min_count` - An optional minimum count for each bin. If a bin has fewer elements than this
/// count, it is skipped.
/// * `map_to_bin` - A function that maps an x value to a bin index.
///
/// # Returns
/// A tuple containing the bin indices and the bin values.
pub(crate) fn group_by_x<T, V, F, B>(
    x: &[T],
    y: &[V],
    min_count: Option<usize>,
    map_to_bin: &F,
) -> (Vec<B>, Vec<Vec<V>>)
where
    T: Copy,
    V: Copy,
    F: Fn(T) -> B,
    B: Copy + PartialEq + PartialOrd,
{
    assert_eq!(x.len(), y.len());
    if x.is_empty() {
        return (Vec::new(), Vec::new());
    }

    let mut bin_indices: Vec<B> = Vec::new();
    let mut bin_values: Vec<Vec<V>> = Vec::new();
    let mut current_bin_values: Vec<V> = Vec::new();

    let mut current_bin_index: B = map_to_bin(x[0]);
    current_bin_values.push(y[0]);

    for i in 1..x.len() {
        let bin_index = map_to_bin(x[i]);
        if bin_index == current_bin_index {
            current_bin_values.push(y[i]);
        } else {
            assert!(
                bin_index > current_bin_index,
                "Values are not sorted by bin index"
            );
            let skip = min_count
                .map(|min_count| current_bin_values.len() < min_count)
                .unwrap_or(false);
            if !skip {
                bin_indices.push(current_bin_index);
                bin_values.push(current_bin_values.clone());
            }
            current_bin_index = bin_index;
            current_bin_values.clear();
        }
    }

    bin_indices.push(current_bin_index);
    current_bin_values.shrink_to_fit();
    bin_values.push(current_bin_values);

    (bin_indices, bin_values)
}

/// Perform (almost) the reverse of the `group_by_x` function: given a set of
/// bin indices and bin values, map the data back to a set of x and y values
/// where the x values are computed from the bin indices.
///
/// # Arguments
/// * `segments` - A vector of tuples, where each tuple contains a vector of x values and a vector
/// of y values.
/// * `ungrouping_function` - A function that maps a bin index to an x value.
///
/// # Returns
/// The x and y values, where the x values are computed from the bin indices. The segments
/// are preserved.
pub(crate) fn ungroup_segments_by_x<X1, Y, X2, F>(
    segments: &Vec<(Vec<X1>, Vec<Y>)>,
    ungrouping_function: &F,
) -> Vec<(Vec<X2>, Vec<Y>)>
where
    X1: Copy,
    Y: Clone,
    F: Fn(X1) -> X2,
{
    let mut new_segments: Vec<(Vec<X2>, Vec<Y>)> = Vec::with_capacity(segments.len());
    for (ref x, y) in segments.iter() {
        let mapped_x: Vec<X2> = x.iter().map(|x| ungrouping_function(*x)).collect();
        new_segments.push((mapped_x, y.clone()));
    }
    new_segments
}

/// Sort the bins in a vector of bins in place. The order of the bins is not changed, but the
/// elements within each bin are sorted.
pub(crate) fn sort_bins_inplace<T>(bins: &mut Vec<Vec<T>>)
where
    T: PartialOrd,
{
    for bin in bins {
        bin.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
}

/// A collection of statistics.
#[derive(Debug, Clone)]
pub(crate) struct Statistics<T>
where
    T: Debug + Clone,
{
    /// The mean (or average) of the data.
    pub mean: f64,
    /// The median of the data.
    pub median: f64,
    /// The minimum value in the data.
    pub min: T,
    /// The maximum value in the data.
    pub max: T,
    /// The first quartile of the data.
    pub q1: f64,
    /// The third quartile of the data.
    pub q3: f64,
    /// The interquartile range of the data, defined as q3 - q1.
    pub iqr: f64,
    /// The standard deviation of the data.
    pub std_dev: f64,
    /// The number of data points.
    pub count: usize,
}

/// Calculate statistics for a vector of bins, where each bin contains a vector of numbers.
pub(crate) fn get_statistics_per_bin<T>(bins: &Vec<Vec<T>>) -> Vec<Statistics<T>>
where
    T: TypeCast<f64> + Default + Copy + Debug,
{
    let mut stats = Vec::with_capacity(bins.len());
    for bin in bins {
        stats.push(get_statistics(bin));
    }
    stats
}

/// Calculate statistics for a sorted slice of numbers.
pub(crate) fn get_statistics<T>(sorted_slice: &[T]) -> Statistics<T>
where
    T: Default + Clone + Copy + Debug + TypeCast<f64>,
{
    let mut stats = Statistics {
        mean: 0.0,
        median: 0.0,
        min: Default::default(),
        max: Default::default(),
        q1: 0.0,
        q3: 0.0,
        iqr: 0.0,
        std_dev: 0.0,
        count: 0,
    };
    stats.count = sorted_slice.len();
    if stats.count == 0 {
        return stats;
    }
    stats.min = sorted_slice[0];
    stats.max = sorted_slice[stats.count - 1];

    let sum = kahan_sum(sorted_slice);
    let mean = sum / stats.count as f64;
    stats.mean = mean;

    let sum_of_squares = kahan_sum(sorted_slice.iter().map(|x| {
        let delta = x.cast() - mean;
        delta * delta
    }));
    stats.std_dev = (sum_of_squares / stats.count as f64).sqrt();
    stats.median = get_median(sorted_slice);
    let half_size = stats.count / 2;
    if half_size == 0 {
        stats.q1 = stats.median;
        stats.q3 = stats.median;
    } else {
        stats.q1 = get_median(&sorted_slice[0..half_size]);
        stats.q3 = get_median(&sorted_slice[sorted_slice.len() - half_size..]);
    }
    stats.iqr = stats.q3 - stats.q1;
    stats
}

/// Dispatch function to extract the requested statistic from a vector of statistics.
pub(crate) fn extract_statistic<T>(statistics: &[Statistics<T>], mode: &AggregationMode) -> Vec<f64>
where
    T: Copy + Default + Debug + TypeCast<f64>,
{
    // Map to y values depending on which aggregation mode is requested.
    match mode {
        AggregationMode::Mean => statistics.iter().map(|s| s.mean).collect(),
        AggregationMode::Min => statistics.iter().map(|s| s.min.cast()).collect(),
        AggregationMode::Q1 => statistics.iter().map(|s| s.q1).collect(),
        AggregationMode::Median => statistics.iter().map(|s| s.median).collect(),
        AggregationMode::Q3 => statistics.iter().map(|s| s.q3).collect(),
        AggregationMode::Max => statistics.iter().map(|s| s.max.cast()).collect(),
        AggregationMode::Count => statistics.iter().map(|s| s.count as f64).collect(),
    }
}

/// Get the median of a sorted slice of numbers. If the slice has an even number of elements,
/// the median is the average of the two middle elements.
pub(crate) fn get_median<T>(sorted_slice: &[T]) -> f64
where
    T: TypeCast<f64> + Copy,
{
    let len = sorted_slice.len();
    if len % 2 == 0 {
        (sorted_slice[len / 2].cast() + sorted_slice[len / 2 - 1].cast()) / 2.0
    } else {
        sorted_slice[len / 2].cast()
    }
}

/// Calculate the sum of a sequence of numbers using the Kahan summation algorithm.
/// This algorithm is used to reduce the error in the sum of a sequence of floating point numbers.
/// The algorithm is described in detail at https://en.wikipedia.org/wiki/Kahan_summation_algorithm.
pub(crate) fn kahan_sum<T, U>(iterable: T) -> f64
where
    T: IntoIterator<Item = U>,
    U: TypeCast<f64>,
{
    let mut sum = 0.0;
    let mut c = 0.0;
    for value in iterable {
        let y = value.cast() - c;
        let t = sum + y;
        c = (t - sum) - y;
        sum = t;
    }
    sum
}

/// Get the function to group timestamps into bins for a given time window.
///
/// # Arguments
/// * `time_period` - The time period to group by.
///
/// # Returns
/// A function that takes a timestamp and returns the bin index.
pub(crate) fn get_grouping_function_for_time_window(
    time_period: &config::TimePeriod,
) -> Box<dyn Fn(i64) -> i64> {
    let quantity = time_period.quantity as i64;
    match time_period.unit {
        TimeUnit::Nanoseconds => {
            #[allow(clippy::identity_op)]
            let factor = quantity * 1;
            Box::new(move |timestamp| timestamp / factor)
        }
        TimeUnit::Microseconds => {
            let factor = quantity * 1_000;
            Box::new(move |timestamp| timestamp / factor)
        }
        TimeUnit::Milliseconds => {
            let factor = quantity * 1_000_000;
            Box::new(move |timestamp| timestamp / factor)
        }
        TimeUnit::Seconds => {
            let factor = quantity * 1_000_000_000;
            Box::new(move |timestamp| timestamp / factor)
        }
        TimeUnit::Minutes => {
            let factor = quantity * 60_000_000_000;
            Box::new(move |timestamp| timestamp / factor)
        }
        TimeUnit::Hours => {
            let factor = quantity * 3_600_000_000_000;
            Box::new(move |timestamp| timestamp / factor)
        }
        TimeUnit::Days => {
            let factor = quantity * 86_400_000_000_000;
            Box::new(move |timestamp| timestamp / factor)
        }
        TimeUnit::Weeks => {
            todo!()
        }
        TimeUnit::Months => todo!(),
        TimeUnit::Years => todo!(),
    }
}

/// Get the function to ungroup timestamps from bins for a given time window.
/// This is the inverse of the grouping function.
///
/// # Arguments
/// * `time_period` - The time period to group by.
///
/// # Returns
/// A function that takes a bin index and returns the timestamp at which that bin starts.
pub(crate) fn get_ungrouping_function_for_time_period(
    time_period: &config::TimePeriod,
) -> Box<dyn Fn(i64) -> i64> {
    let quantity = time_period.quantity as i64;
    match time_period.unit {
        TimeUnit::Nanoseconds => {
            #[allow(clippy::identity_op)]
            let factor = quantity * 1;
            Box::new(move |timestamp| timestamp * factor)
        }
        TimeUnit::Microseconds => {
            let factor = quantity * 1_000;
            Box::new(move |timestamp| timestamp * factor)
        }
        TimeUnit::Milliseconds => {
            let factor = quantity * 1_000_000;
            Box::new(move |timestamp| timestamp * factor)
        }
        TimeUnit::Seconds => {
            let factor = quantity * 1_000_000_000;
            Box::new(move |timestamp| timestamp * factor)
        }
        TimeUnit::Minutes => {
            let factor = quantity * 60_000_000_000;
            Box::new(move |timestamp| timestamp * factor)
        }
        TimeUnit::Hours => {
            let factor = quantity * 3_600_000_000_000;
            Box::new(move |timestamp| timestamp * factor)
        }
        TimeUnit::Days => {
            let factor = quantity * 86_400_000_000_000;
            Box::new(move |timestamp| timestamp * factor)
        }
        TimeUnit::Weeks => {
            todo!()
        }
        TimeUnit::Months => todo!(),
        TimeUnit::Years => todo!(),
    }
}
