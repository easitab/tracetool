use std::path::Path;

use sqlparser::dialect::MsSqlDialect;

use crate::util;
use crate::util::Result;

pub(crate) fn match_query_view<P: AsRef<Path>>(database_path: P) -> Result<()> {
    let conn = rusqlite::Connection::open(database_path)?;

    let sql = util::read_stdin_string()?;

    let dialect = MsSqlDialect {};
    let sql = util::normalize_sql(&dialect, &sql)?;

    let mut query = conn
        .prepare(
            "select distinct view_id from item_view_executor_execute_normalized as n
        inner join item_view_executor_execute as e on n.id = e.query
        where n.query = ?
        order by view_id",
        )
        .expect("Error preparing query");
    let mut rows = query.query(rusqlite::params![sql])?;
    while let Some(row) = rows.next().expect("Error reading row") {
        let id: i64 = row.get(0).expect("Error reading view id");
        println!("{}", id);
    }
    Ok(())
}
