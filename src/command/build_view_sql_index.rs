use std::path::Path;

use indicatif::{ProgressBar, ProgressStyle};
use rusqlite::types::Value::Null;

use crate::util;
use crate::util::Result;

pub(crate) fn build_view_sql_index<P: AsRef<Path>>(database_path: P) -> Result<()> {
    let mut conn = rusqlite::Connection::open(database_path)?;

    eprintln!("Building item_view_executor_execute_normalized table...");
    conn.execute(
        "drop table if exists item_view_executor_execute_normalized",
        [],
    )?;

    conn.execute(
        "create table item_view_executor_execute_normalized (
        id integer not null,
        query TEXT,
        primary key (id)
        )",
        [],
    )?;

    let tx = conn.transaction()?;

    let count = tx.query_row(
        "select count(1) from item_view_executor_execute_query",
        [],
        |row| row.get(0),
    )?;

    let pb = ProgressBar::new(count);
    pb.set_style(
        // Template to show number of rows processed
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:.cyan/blue}] {human_pos}/{human_len} ({per_sec}, {eta})",
        )
        .unwrap(),
    );

    let mut query_stmt = tx.prepare("select id, query from item_view_executor_execute_query")?;
    let mut insert_stmt = tx.prepare(
        "insert into item_view_executor_execute_normalized
        (id, query) values (?, ?)",
    )?;

    let mut rows = query_stmt.query([])?;
    let dialect = sqlparser::dialect::MsSqlDialect {};
    while let Some(row) = rows.next()? {
        let id: i64 = row.get(0)?;
        let query: String = row.get(1)?;

        let normalized = match util::normalize_sql(&dialect, &query) {
            Ok(normalized) => Some(normalized),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        };

        if let Some(normalized) = &normalized {
            insert_stmt.execute(rusqlite::params![&id, &normalized])?;
        } else {
            insert_stmt.execute(rusqlite::params![id, Null])?;
        }
        pb.inc(1);
    }

    drop(rows);
    drop(insert_stmt);
    drop(query_stmt);

    pb.finish();

    eprintln!("Creating indices...");
    tx.execute(
        "create index if not exists item_view_executor_execute_normalized_query
    on item_view_executor_execute_normalized (query)",
        [],
    )?;

    tx.commit()?;

    Ok(())
}
