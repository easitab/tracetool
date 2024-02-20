use std::collections::BTreeMap;
use std::path::Path;

use indicatif::{ProgressBar, ProgressStyle};

use crate::util::Result;

struct ActiveQuery {
    start_time: i64,
    ordinal: u32,
    end_time: i64,
    overlap: u64,
    overlap_count: u32,
}

pub(crate) fn compute_overlap<P: AsRef<Path>>(database_path: P) -> Result<()> {
    let mut conn = rusqlite::Connection::open(database_path)?;

    conn.execute(
        "drop table if exists item_view_executor_execute_overlap",
        [],
    )?;
    conn.execute("drop table if exists active_query_count", [])?;

    let tx = conn.transaction()?;

    tx.execute(
        "create table item_view_executor_execute_overlap (
        timestamp integer not null,
        ordinal integer not null,
        overlap integer not null,
        overlap_count integer not null,
        primary key (timestamp, ordinal)
    )",
        [],
    )?;

    tx.execute(
        "create table active_query_count (
        timestamp integer not null,
        count integer not null,
        primary key (timestamp)
    )",
        [],
    )?;

    let count = tx.query_row(
        "select count(1) from item_view_executor_execute",
        [],
        |row| row.get(0),
    )?;
    let mut query_stmt = tx.prepare("select timestamp, ordinal, wallclock_time_ns from item_view_executor_execute order by timestamp, ordinal")?;
    let mut overlap_insert = tx.prepare("insert into item_view_executor_execute_overlap (timestamp, ordinal, overlap, overlap_count) values (?, ?, ?, ?)")?;
    let mut active_query_count_insert =
        tx.prepare("insert or replace into active_query_count (timestamp, count) values (?, ?)")?;
    let mut rows = query_stmt.query([])?;

    let mut active_queries: BTreeMap<i64, ActiveQuery> = BTreeMap::new();

    let pb = ProgressBar::new(count);
    pb.set_style(
        // Template to show number of rows processed
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:.cyan/blue}] {human_pos}/{human_len} ({per_sec}, {eta})",
        )
        .unwrap(),
    );

    while let Some(row) = rows.next()? {
        let start_time: i64 = row.get(0)?;
        let ordinal: u32 = row.get(1)?;
        let wallclock_time_ns: i64 = row.get(2)?;

        // Remove queries that have ended, and update the active query count. We iterate over
        // them in order of end time, so we can just count down for each one we remove.
        let mut active_query_count = active_queries.len() as u32;
        let mut removed_queries = active_queries;
        active_queries = removed_queries.split_off(&start_time);
        for query in removed_queries.values() {
            overlap_insert.execute(rusqlite::params![
                query.start_time,
                query.ordinal,
                query.overlap,
                query.overlap_count
            ])?;

            active_query_count -= 1;
            active_query_count_insert
                .execute(rusqlite::params![query.end_time, active_query_count])?;
        }

        let end_time = start_time + wallclock_time_ns;
        let mut initial_overlap = 0;
        for query in active_queries.values_mut() {
            // Compute the overlap in time between the two queries
            assert!(start_time <= query.end_time);

            let start = std::cmp::max(start_time, query.start_time);
            let end = std::cmp::min(end_time, query.end_time);
            let overlap = end - start;
            assert!(overlap >= 0);
            let overlap = overlap as u64;

            // Add the overlap to both queries.
            query.overlap += overlap;
            query.overlap_count += 1;
            initial_overlap += overlap;
        }

        // Add the new query to the active queries
        active_queries.insert(
            end_time,
            ActiveQuery {
                start_time,
                ordinal,
                end_time,
                overlap: initial_overlap,
                overlap_count: active_queries.len() as u32,
            },
        );

        active_query_count_insert.execute(rusqlite::params![start_time, active_queries.len()])?;

        pb.inc(1);
    }

    drop(rows);
    drop(query_stmt);
    drop(overlap_insert);
    drop(active_query_count_insert);

    tx.commit()?;

    pb.finish();

    Ok(())
}
