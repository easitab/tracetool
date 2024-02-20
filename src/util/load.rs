use std::collections::hash_map;
use std::collections::HashMap;

use bytesize::ByteSize;
use rusqlite::Connection;

use crate::config::Filter;
use crate::{config, util, util::Result};

/// Read time-based samples from the database.
///
/// # Arguments
/// * `conn` - A connection to the database.
/// * `start` - The start time for the samples. If `None`, the query starts at the
/// beginning of the data.
/// * `end` - The end time for the samples. If `None`, the query ends at the end of the
/// data.
/// * `filter` - An optional `Filter` object that contains the filter configuration.
/// * `column` - The name of the column to read from the database.
/// * `table` - The name of the table to read from the database.
pub(crate) fn get_samples(
    conn: &Connection,
    start: Option<&str>,
    end: Option<&str>,
    filter: Option<&Filter>,
    column: &str,
    table: &str,
) -> Result<(Vec<i64>, Vec<u64>)> {
    // TODO we can get the start and end from the Filter instead.
    let mut sql = format!("select timestamp, {} from {}", column, table);
    let criteria = get_common_criteria(start, end, filter, None);
    if !criteria.is_empty() {
        sql.push_str(" where ");
        sql.push_str(&criteria.join(" and "));
    }
    sql.push_str(" order by timestamp");
    eprintln!("Executing query: {}", sql);

    let mut stmt = conn.prepare(sql.as_str())?;
    let mut rows = stmt.query([])?;
    let mut timestamps: Vec<i64> = Vec::new();
    let mut counts: Vec<u64> = Vec::new();
    while let Some(row) = rows.next()? {
        timestamps.push(row.get(0)?);
        counts.push(row.get(1)?);
    }

    // Show amount of memory consumed by the vectors
    let total_capacity = timestamps.capacity() * std::mem::size_of::<f64>()
        + counts.capacity() * std::mem::size_of::<u64>();
    let total_size =
        timestamps.len() * std::mem::size_of::<f64>() + counts.len() * std::mem::size_of::<u64>();
    println!("Data points: {}", timestamps.len());
    println!(
        "Memory use: {} (heap {})",
        ByteSize(total_size as u64),
        ByteSize(total_capacity as u64)
    );
    Ok((timestamps, counts))
}

/// Read time-based samples from the database, assuming that the sample values are counters
/// (e.g., the number of requests per second).
///
/// # Arguments
/// * `conn` - A connection to the database.
/// * `common_plot_config` - The common configuration for the plot.
/// * `plot_config` - The configuration for the count scatter plot.
pub(crate) fn get_count_samples(
    conn: &Connection,
    common_plot_config: &config::PlotCommon,
    plot_config: &config::CountScatterPlot,
) -> Result<(Vec<i64>, Vec<u64>)> {
    let mut sql = format!(
        "select timestamp, {} from {}",
        plot_config.column, plot_config.table
    );
    let criteria = get_common_criteria(
        common_plot_config
            .filter
            .as_ref()
            .and_then(|f| f.start.as_deref()),
        common_plot_config
            .filter
            .as_ref()
            .and_then(|f| f.end.as_deref()),
        None,
        None,
    );
    if !criteria.is_empty() {
        sql.push_str(" where ");
        sql.push_str(&criteria.join(" and "));
    }
    sql.push_str(" order by timestamp");
    eprintln!("Executing query: {}", sql);

    let mut stmt = conn.prepare(sql.as_str())?;
    let mut rows = stmt.query([])?;
    let mut timestamps: Vec<i64> = Vec::new();
    let mut counts: Vec<u64> = Vec::new();
    while let Some(row) = rows.next()? {
        timestamps.push(row.get(0)?);
        counts.push(row.get(1)?);
    }

    // Show amount of memory consumed by the vectors
    let total_capacity = timestamps.capacity() * std::mem::size_of::<f64>()
        + counts.capacity() * std::mem::size_of::<u64>();
    let total_size =
        timestamps.len() * std::mem::size_of::<f64>() + counts.len() * std::mem::size_of::<u64>();
    println!("Data points: {}", timestamps.len());
    println!(
        "Memory use: {} (heap {})",
        ByteSize(total_size as u64),
        ByteSize(total_capacity as u64)
    );
    Ok((timestamps, counts))
}

/// Generate SQL criteria for common filter configuration.
///
/// # Arguments
/// * `start` - The start time for the query. If `None`, the samples start at the
/// beginning of the data.
/// * `end` - The end time for the query. If `None`, the samples end at the end of the
/// data.
/// * `filter` - An optional `Filter` object that contains the filter configuration.
/// * `timestamp_table` - The name of the table that contains the timestamp column. If `None`,
/// no table qualifier is used, which would typically mean that SQLite gets the column from
/// the only table in the query.
///
/// # Returns
/// * A vector of strings, where each string represents an SQL criteria. These should be
/// joined with `and` to form a complete SQL `where` clause.
pub(crate) fn get_common_criteria(
    start: Option<&str>,
    end: Option<&str>,
    filter: Option<&Filter>,
    timestamp_table: Option<&str>,
) -> Vec<String> {
    let mut criteria: Vec<String> = Vec::new();
    if let Some(filter) = filter {
        if let Some(sql_where) = &filter.sql_where {
            criteria.push(sql_where.to_string());
        }
    }
    let qualifier = match timestamp_table {
        Some(table) => format!("{}.", table),
        None => String::new(),
    };
    if let Some(start) = &start {
        // FIXME need to report parse error up here instead of throwing a backtrace in the caller's face.
        // Also perhaps document what format is expected!?
        let start = util::parse_datetime_floor(start).unwrap();
        let start = start
            .timestamp_nanos_opt()
            .expect("value can not be represented in a timestamp with nanosecond precision.");
        criteria.push(format!("{}timestamp >= {}", qualifier, start));
    }
    if let Some(end) = &end {
        // FIXME same here
        let end = util::parse_datetime_ceil(end).unwrap();
        let end = end
            .timestamp_nanos_opt()
            .expect("value can not be represented in a timestamp with nanosecond precision.");
        criteria.push(format!("{}timestamp <= {}", qualifier, end));
    }
    criteria
}

/// Read overlap information for a view from the database.
///
/// # Arguments
/// * `conn` - A connection to the database.
/// * `view_id` - The ID of the view to read overlap information for.
/// * `start` - The start time for the samples. If `None`, the query starts at the
/// beginning of the data.
/// * `end` - The end time for the samples. If `None`, the query ends at the end of the
/// data.
/// * `filter` - An optional `Filter` object that contains the filter configuration.
///
/// # Returns
/// * A `ViewDurationVsOverlap` object that contains the wallclock time and overlap time
/// for the view.
pub(crate) fn get_overlap_samples_for_view(
    conn: &Connection,
    view_id: i32,
    start: Option<&str>,
    end: Option<&str>,
    filter: Option<&Filter>,
) -> Result<ViewDurationVsOverlap> {
    let mut criteria = get_common_criteria(start, end, filter, Some("e"));

    criteria.push(format!("e.view_id = {}", view_id));

    let sql = format!(
        "{}{}{}{}{}",
        "select e.wallclock_time_ns, o.overlap ",
        "from item_view_executor_execute as e ",
        "inner join item_view_executor_execute_overlap as o ",
        "on e.timestamp = o.timestamp and e.ordinal = o.ordinal where ",
        &criteria.join(" and ")
    );
    eprintln!("Executing query: {}", sql);
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query([])?;

    let mut wallclock_time: Vec<u64> = Vec::new();
    let mut overlap: Vec<u64> = Vec::new();
    while let Some(row) = rows.next()? {
        wallclock_time.push(row.get(0)?);
        overlap.push(row.get(1)?);
    }

    Ok(ViewDurationVsOverlap {
        wallclock_time,
        overlap,
    })
}

/// Overlap information for a view.
/// Each element in the contained vectors corresponds to a single
// execution of a view. The first vector contains the , and the second vector contains . Both durations are in nanoseconds.
pub(crate) struct ViewDurationVsOverlap {
    /// Wallclock duration of the execution.
    pub wallclock_time: Vec<u64>,
    /// Overlap duration of the execution. This is the total time that the view
    /// was executing concurrently with other views
    pub overlap: Vec<u64>,
}

/// Read overlap information for all views from the database.
///
/// # Arguments
/// * `conn` - A connection to the database.
/// * `start` - The start time for the samples. If `None`, the query starts at the
/// beginning of the data.
/// * `end` - The end time for the samples. If `None`, the query ends at the end of the
/// data.
/// * `filter` - An optional `Filter` object that contains the filter configuration.
///
/// # Returns
/// * A map with view IDs as keys and `ViewDurationVsOverlap` objects as values. Each
/// `ViewDurationVsOverlap` object contains the wallclock time and overlap time for a
/// view.
pub(crate) fn get_overlap_samples(
    conn: &Connection,
    start: Option<&str>,
    end: Option<&str>,
    filter: Option<&Filter>,
) -> Result<HashMap<i32, ViewDurationVsOverlap>> {
    let mut criteria = get_common_criteria(start, end, filter, Some("e"));

    let mut sql: Vec<String> = vec![
        "select e.view_id, e.wallclock_time_ns, o.overlap ".to_string(),
        "from item_view_executor_execute as e ".to_string(),
        "inner join item_view_executor_execute_overlap as o ".to_string(),
        "on e.timestamp = o.timestamp and e.ordinal = o.ordinal where ".to_string(),
    ];
    criteria.push("view_id is not null".to_string());
    sql.push(criteria.join(" and "));

    let sql = sql.join("");
    eprintln!("Executing query: {}", sql);

    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query([])?;

    let mut by_view: HashMap<i32, ViewDurationVsOverlap> = HashMap::new();
    while let Some(row) = rows.next()? {
        let view_id: i32 = row.get(0)?;
        let ViewDurationVsOverlap {
            wallclock_time,
            overlap,
        } = match by_view.entry(view_id) {
            hash_map::Entry::Occupied(e) => e.into_mut(),
            hash_map::Entry::Vacant(e) => e.insert(ViewDurationVsOverlap {
                wallclock_time: Vec::new(),
                overlap: Vec::new(),
            }),
        };
        wallclock_time.push(row.get(1)?);
        overlap.push(row.get(2)?);
    }

    Ok(by_view)
}
