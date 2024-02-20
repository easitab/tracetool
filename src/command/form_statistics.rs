use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::Path;

use crate::config::Filter;
use crate::{util, util::Result};

pub(crate) fn form_statistics<P: AsRef<Path>>(
    database_path: P,
    start: Option<&str>,
    end: Option<&str>,
) -> Result<()> {
    let conn = rusqlite::Connection::open(database_path)?;
    let mut by_form: HashMap<i32, Vec<u64>> = get_samples_by_form(
        &conn,
        start,
        end,
        None,
        "wallclock_time_ns",
        "form_widget_startup",
    )?;

    eprintln!("Calculating statistics...");
    let mut statistics_by_form: Vec<(i32, util::Statistics<f64>)> = by_form
        .iter_mut()
        .map(|(form_id, values)| {
            values.sort();
            let values = util::nanoseconds_duration_to_seconds(values);
            (*form_id, util::get_statistics(&values))
        })
        .collect();
    statistics_by_form.sort_by(|(_, s1), (_, s2)| s1.q3.partial_cmp(&s2.q3).unwrap());

    let mut csv_writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(std::io::stdout());

    csv_writer.write_record([
        "form ID",
        "count",
        "min",
        "max",
        "mean",
        "median",
        "Q1",
        "Q3",
        "IQR",
        "standard deviation",
    ])?;

    for (form_id, statistics) in statistics_by_form.iter() {
        csv_writer.write_record(&[
            form_id.to_string(),
            statistics.count.to_string(),
            statistics.min.to_string(),
            statistics.max.to_string(),
            statistics.mean.to_string(),
            statistics.median.to_string(),
            statistics.q1.to_string(),
            statistics.q3.to_string(),
            statistics.iqr.to_string(),
            statistics.std_dev.to_string(),
        ])?;
    }

    Ok(())
}

pub fn get_samples_by_form(
    conn: &rusqlite::Connection,
    start: Option<&str>,
    end: Option<&str>,
    filter: Option<&Filter>,
    column: &str,
    table: &str,
) -> Result<HashMap<i32, Vec<u64>>> {
    let mut sql = format!("select form_id, {} from {}", column, table);
    let mut criteria = util::get_common_criteria(start, end, filter, None);
    criteria.push("form_id is not null".to_string());
    sql.push_str(" where ");
    sql.push_str(&criteria.join(" and "));
    sql.push_str(" order by timestamp");
    eprintln!("Executing query: {}", sql);

    let mut stmt = conn.prepare(sql.as_str())?;
    let mut rows = stmt.query([])?;
    let mut by_form: HashMap<i32, Vec<u64>> = HashMap::new();
    while let Some(row) = rows.next()? {
        let form_id: i32 = row.get(0)?;
        let values = match by_form.entry(form_id) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(Vec::new()),
        };
        values.push(row.get(1)?);
    }
    Ok(by_form)
}
