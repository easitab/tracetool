/// Calculate a normalized overlap percentage for a set of wallclock times and
/// overlaps. The percentage is calculated in relation to the wallclock time,
/// so that a 100% overlap would be equivalent to executing concurrently with
/// a single query for the entire execution time.
///
/// # Arguments
/// * `wallclock_time` - A vector of `u64` values, where each value represents
/// the wallclock duration of a query in nanoseconds.
/// * `overlap` - A vector of `u64` values, where each value represents the
/// total overlap with other queries in nanoseconds.
pub(crate) fn overlap_to_percent(wallclock_time: &[u64], overlap: &[u64]) -> Vec<f64> {
    assert_eq!(wallclock_time.len(), overlap.len());

    let mut result = Vec::new();
    for i in 0..wallclock_time.len() {
        let wallclock_time = wallclock_time[i];
        let overlap = overlap[i];

        result.push((overlap as f64 / wallclock_time as f64) * 100.0);
    }

    result
}
