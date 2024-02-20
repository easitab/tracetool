use sqlparser::dialect::MsSqlDialect;

use crate::util;
use crate::Result;

pub(crate) fn normalize_sql() -> Result<()> {
    let sql = util::read_stdin_string()?;
    let dialect = MsSqlDialect {};
    let sql = util::normalize_sql(&dialect, &sql)?;
    println!("{}", sql);
    Ok(())
}
