use std::fs;
use std::fs::File;
use std::io::{BufReader, Error as IoError, ErrorKind};
use std::path::Path;

use chrono::{NaiveDateTime, TimeZone, Utc};
use csv::{ReaderBuilder, StringRecord};
use indicatif::{ProgressBar, ProgressStyle};
use rusqlite::{params, params_from_iter, Connection};

use crate::util::Result;

#[derive(Debug, Clone)]
struct ColumnDescriptor {
    csv_name: &'static str,
    column_name: &'static str,
    column_type: ColumnType,
}

#[derive(Debug, Clone)]
struct TableDescriptor {
    file_name: &'static str,
    table_name: &'static str,
    timestamp_column_name: &'static str,
    columns: Vec<ColumnDescriptor>,
}

#[derive(Debug, Clone)]
enum ColumnType {
    Boolean,
    Integer,
    OptionalInteger,
    Text,
    ExternalText,
}

pub(crate) fn import_data<P1: AsRef<Path>, P2: AsRef<Path>>(
    target: P1,
    sources: &[P2],
) -> Result<()> {
    for source in sources {
        let path = source.as_ref();
        eprintln!("{}", path.display());
        match import_directory(&target, source) {
            Ok(()) => (),
            Err(e) => println!("Error importing data from {:?}: {:?}", source.as_ref(), e),
        }
    }
    Ok(())
}

lazy_static! {
    static ref STANDARD_COST_COLUMNS: Vec<ColumnDescriptor> = vec![
        ColumnDescriptor {
            csv_name: "wallclock time (ns)",
            column_name: "wallclock_time_ns",
            column_type: ColumnType::Integer
        },
        ColumnDescriptor {
            csv_name: "cpu time (ns)",
            column_name: "cpu_time_ns",
            column_type: ColumnType::Integer
        },
        ColumnDescriptor {
            csv_name: "user time (ns)",
            column_name: "user_time_ns",
            column_type: ColumnType::Integer
        },
    ];
    static ref STANDARD_COUNT_COLUMNS: Vec<ColumnDescriptor> = vec![ColumnDescriptor {
        csv_name: "count",
        column_name: "count",
        column_type: ColumnType::Integer
    },];
    static ref STANDARD_GAUGE_COLUMNS: Vec<ColumnDescriptor> = vec![ColumnDescriptor {
        csv_name: "value",
        column_name: "value",
        column_type: ColumnType::Integer
    },];
    static ref TABLE_DESCRIPTORS: Vec<TableDescriptor> = vec![
        TableDescriptor {
            file_name: "cost/BPEServerImpl.init.csv",
            table_name: "bpe_server_impl_init",
            timestamp_column_name: "timestamp",
            columns: STANDARD_COST_COLUMNS.clone(),
        },
        TableDescriptor {
            file_name: "cost/ItemViewWidget.startup.csv",
            table_name: "item_view_widget_startup",
            timestamp_column_name: "timestamp",
            columns: [
                STANDARD_COST_COLUMNS.clone(),
                vec![
                    ColumnDescriptor {
                        csv_name: "view id",
                        column_name: "view_id",
                        column_type: ColumnType::OptionalInteger
                    },
                    ColumnDescriptor {
                        csv_name: "module id",
                        column_name: "module_id",
                        column_type: ColumnType::Integer
                    },
                    ColumnDescriptor {
                        csv_name: "user name",
                        column_name: "user_name",
                        column_type: ColumnType::Text
                    },
                ]
            ]
            .concat()
        },
        TableDescriptor {
            file_name: "cost/ItemViewExecutor.execute.csv",
            table_name: "item_view_executor_execute",
            timestamp_column_name: "timestamp",
            columns: [
                STANDARD_COST_COLUMNS.clone(),
                vec![
                    ColumnDescriptor {
                        csv_name: "view id",
                        column_name: "view_id",
                        column_type: ColumnType::OptionalInteger
                    },
                    ColumnDescriptor {
                        csv_name: "result offset",
                        column_name: "result_offset",
                        column_type: ColumnType::Integer
                    },
                    ColumnDescriptor {
                        csv_name: "aggregation count",
                        column_name: "aggregation_count",
                        column_type: ColumnType::Integer
                    },
                    ColumnDescriptor {
                        csv_name: "extra criterion",
                        column_name: "extra_criterion",
                        column_type: ColumnType::Boolean
                    },
                    ColumnDescriptor {
                        csv_name: "orders",
                        column_name: "orders",
                        column_type: ColumnType::Integer
                    },
                    ColumnDescriptor {
                        csv_name: "joins",
                        column_name: "joins",
                        column_type: ColumnType::Integer
                    },
                    ColumnDescriptor {
                        csv_name: "query",
                        column_name: "query",
                        column_type: ColumnType::ExternalText
                    },
                ]
            ]
            .concat()
        },
        TableDescriptor {
            file_name: "cost/ItemViewExecutor.getItemViewTotalSizeAndMaxUpdated.csv",
            table_name: "item_view_executor_get_item_view_total_size_and_max_updated",
            timestamp_column_name: "timestamp",
            columns: [
                STANDARD_COST_COLUMNS.clone(),
                vec![
                    ColumnDescriptor {
                        csv_name: "view id",
                        column_name: "view_id",
                        column_type: ColumnType::OptionalInteger
                    },
                    ColumnDescriptor {
                        csv_name: "count",
                        column_name: "count",
                        column_type: ColumnType::Integer
                    },
                ]
            ]
            .concat()
        },
        TableDescriptor {
            file_name: "cost/FormWidget.startup.csv",
            table_name: "form_widget_startup",
            timestamp_column_name: "timestamp",
            columns: [
                STANDARD_COST_COLUMNS.clone(),
                vec![
                    ColumnDescriptor {
                        csv_name: "item id",
                        column_name: "item_id",
                        column_type: ColumnType::OptionalInteger
                    },
                    ColumnDescriptor {
                        csv_name: "form mode",
                        column_name: "form_mode",
                        column_type: ColumnType::Text
                    },
                    ColumnDescriptor {
                        csv_name: "form id",
                        column_name: "form_id",
                        column_type: ColumnType::OptionalInteger
                    },
                    ColumnDescriptor {
                        csv_name: "user name",
                        column_name: "user_name",
                        column_type: ColumnType::Text
                    },
                ]
            ]
            .concat()
        },
        TableDescriptor {
            file_name: "cost/FormWidget.handleSaveOfItem.csv",
            table_name: "form_widget_handle_save_of_item",
            timestamp_column_name: "timestamp",
            columns: [
                STANDARD_COST_COLUMNS.clone(),
                vec![
                    ColumnDescriptor {
                        csv_name: "item id",
                        column_name: "item_id",
                        column_type: ColumnType::OptionalInteger
                    },
                    ColumnDescriptor {
                        csv_name: "user name",
                        column_name: "user_name",
                        column_type: ColumnType::Text
                    },
                ]
            ]
            .concat()
        },
        TableDescriptor {
            file_name: "timer/db.ping.csv",
            table_name: "db_ping",
            timestamp_column_name: "start",
            columns: vec![ColumnDescriptor {
                csv_name: "duration",
                column_name: "duration",
                column_type: ColumnType::Integer
            },]
        },
        TableDescriptor {
            file_name: "micrometer/oshi.os.process.usertime.csv",
            table_name: "oshi_os_process_usertime",
            timestamp_column_name: "timestamp",
            columns: STANDARD_COUNT_COLUMNS.clone(),
        },
        TableDescriptor {
            file_name: "micrometer/session.user.count.csv",
            table_name: "session_user_count",
            timestamp_column_name: "timestamp",
            columns: STANDARD_COUNT_COLUMNS.clone(),
        },
        TableDescriptor {
            file_name: "micrometer/session.count.csv",
            table_name: "session_count",
            timestamp_column_name: "timestamp",
            columns: STANDARD_COUNT_COLUMNS.clone(),
        },
        TableDescriptor {
            file_name: "micrometer/oshi.hardware.memory.available.csv",
            table_name: "oshi_hardware_memory_available",
            timestamp_column_name: "timestamp",
            columns: STANDARD_GAUGE_COLUMNS.clone(),
        }
    ];
}

fn import_directory<P1: AsRef<Path>, P2: AsRef<Path>>(target: &P1, source: P2) -> Result<()> {
    let sizes = get_file_sizes(&source, &TABLE_DESCRIPTORS)?;
    let total_size: u64 = sizes.iter().sum();

    let mut conn = Connection::open(target)?;

    let tx = conn.transaction()?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] [{bar:.cyan/blue}] {msg} {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .unwrap()
            .progress_chars("##-"));

    let mut consumed_so_far = 0u64;
    for (i, table_descriptor) in TABLE_DESCRIPTORS.iter().enumerate() {
        pb.set_message(table_descriptor.file_name);
        import_table(
            &tx,
            table_descriptor,
            &source,
            table_descriptor.timestamp_column_name,
            |progress| {
                let progress = consumed_so_far + progress;
                pb.set_position(progress);
            },
        )?;
        consumed_so_far += sizes[i];
    }

    tx.commit()?;
    pb.finish();

    Ok(())
}

fn get_file_sizes<P: AsRef<Path>>(
    root_path: &P,
    descriptor: &Vec<TableDescriptor>,
) -> Result<Vec<u64>> {
    let mut sizes = Vec::with_capacity(descriptor.len());
    for table_descriptor in descriptor.iter() {
        let source_path = root_path.as_ref().join(table_descriptor.file_name);
        let metadata = match fs::metadata(&source_path) {
            Ok(metadata) => metadata,
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    sizes.push(0);
                    continue;
                }
                return Err(Box::new(IoError::new(
                    err.kind(),
                    format!("Failed to open file {}: {}", source_path.display(), err),
                )));
            }
        };
        sizes.push(metadata.len());
    }
    Ok(sizes)
}

fn import_table<P: AsRef<Path>, Pr: FnMut(u64)>(
    conn: &Connection,
    descriptor: &TableDescriptor,
    root_path: P,
    timestamp_column_name: &str,
    mut progress: Pr,
) -> Result<()> {
    let source_path = root_path.as_ref().join(descriptor.file_name);
    let file = match File::open(&source_path) {
        Ok(file) => file,
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                println!("{} not found, skipping", source_path.display());
                return Ok(());
            }
            return Err(Box::new(IoError::new(
                err.kind(),
                format!("Failed to open file {}: {}", source_path.display(), err),
            )));
        }
    };
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);
    let headers: Vec<&str> = csv_reader.headers()?.iter().collect();
    if headers.is_empty()
        || headers[0] != timestamp_column_name
        || headers[1..]
            != descriptor
                .columns
                .iter()
                .map(|c| c.csv_name)
                .collect::<Vec<&str>>()
    {
        return Err(Box::new(IoError::new(
            ErrorKind::InvalidData,
            format!("File {} has unexpected headers", source_path.display()),
        )));
    }

    create_table_if_not_exists(conn, descriptor).unwrap();

    let mut insert_stmt = build_insert_statement(conn, descriptor).unwrap();

    let mut external_upsert_statements: Vec<_> = descriptor
        .columns
        .iter()
        .map(|c| {
            if let ColumnType::ExternalText = c.column_type {
                let insert_or_ignore = format!(
                    "insert or ignore into {}_{} (id, {}) values (\
                coalesce((select max(id)+1 from {}_{}), 0), ?)",
                    descriptor.table_name,
                    c.column_name,
                    c.column_name,
                    descriptor.table_name,
                    c.column_name
                );
                let insert_or_ignore = conn.prepare(&insert_or_ignore).unwrap();
                let get_id = format!(
                    "select id from {}_{} where {} = ?",
                    descriptor.table_name, c.column_name, c.column_name
                );
                let get_id = conn.prepare(&get_id).unwrap();
                Some((insert_or_ignore, get_id))
            } else {
                None
            }
        })
        .collect();

    let mut sqlite_values = Vec::with_capacity(descriptor.columns.len() + 2);
    let mut record = StringRecord::with_capacity(256usize, descriptor.columns.len() + 1);

    while csv_reader.read_record(&mut record)? {
        sqlite_values.clear();

        let timestamp = parse_timestamp(&record[0])?;
        sqlite_values.push(rusqlite::types::Value::Integer(timestamp));
        sqlite_values.push(rusqlite::types::Value::Integer(timestamp));

        for (i, column) in descriptor.columns.iter().enumerate() {
            let csv_value = &record[i + 1];
            let sqlite_value = match column.column_type {
                ColumnType::Boolean => parse_boolean(csv_value),
                ColumnType::Integer => parse_integer(csv_value),
                ColumnType::OptionalInteger => parse_optional_integer(csv_value),
                ColumnType::Text => parse_text(csv_value),
                ColumnType::ExternalText => parse_text(csv_value),
            };
            let sqlite_value = match sqlite_value {
                Ok(sqlite_value) => sqlite_value,
                Err(err) => {
                    return Err(Box::new(IoError::new(
                        ErrorKind::InvalidData,
                        format!(
                            "Failed to parse value {:?} in column {} of file {}: {}",
                            csv_value,
                            column.csv_name,
                            source_path.display(),
                            err
                        ),
                    )));
                }
            };

            if let ColumnType::ExternalText = column.column_type {
                let (ref mut insert_or_ignore, ref mut get_id) =
                    external_upsert_statements[i].as_mut().unwrap();
                insert_or_ignore.execute(params![sqlite_value])?;
                let id: i64 = get_id.query_row(params![sqlite_value], |row| row.get(0))?;
                sqlite_values.push(rusqlite::types::Value::Integer(id));
            } else {
                sqlite_values.push(sqlite_value);
            }
        }
        let result: std::result::Result<usize, rusqlite::Error> =
            insert_stmt.execute(params_from_iter(&sqlite_values));
        match result {
            Ok(_) => {}
            Err(err) => {
                if let rusqlite::Error::SqliteFailure(err, _) = err {
                    if err.code == rusqlite::ErrorCode::ConstraintViolation
                        && err.extended_code == 1555
                    {
                        return Err(Box::new(IoError::new(
                            ErrorKind::AlreadyExists,
                            format!(
                                "Duplicate timestamp {} in file {}",
                                &record[0],
                                source_path.display()
                            ),
                        )));
                    }
                }
                Err(err)?;
            }
        };

        record.clear(); // TODO is this necessary?
        progress(csv_reader.position().byte());
    }

    Ok(())
}

fn parse_timestamp(csv_value: &str) -> Result<i64> {
    // Parse timestamp like 20210825122527.278673700 or 20230317170814.424 into nanoseconds
    // since Unix epoch.
    let len = csv_value.len();
    if !(len == 18 || len == 24) {
        return Err(Box::new(IoError::new(
            ErrorKind::InvalidData,
            format!("Invalid timestamp {}", csv_value),
        )));
    }
    let (datetime_str, fraction_str) = csv_value.split_at(14);
    let datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y%m%d%H%M%S")?;
    let mut fraction = fraction_str[1..].parse::<u64>()?;
    if len == 18 {
        // Fraction is in milliseconds, convert to nanoseconds.
        fraction *= 1_000_000;
    }

    let utc_datetime = Utc.from_utc_datetime(&datetime);
    let timestamp = utc_datetime
        .timestamp_nanos_opt()
        .map(|n| n + fraction as i64)
        .expect("value can not be represented in a timestamp with nanosecond precision.");
    Ok(timestamp)
}

fn parse_boolean(csv_value: &str) -> Result<rusqlite::types::Value> {
    let csv_value = csv_value.parse::<bool>()?;
    Ok(rusqlite::types::Value::Integer(csv_value as i64))
}

fn parse_integer(csv_value: &str) -> Result<rusqlite::types::Value> {
    let csv_value = csv_value.parse::<i64>()?;
    Ok(rusqlite::types::Value::Integer(csv_value))
}

fn parse_optional_integer(csv_value: &str) -> Result<rusqlite::types::Value> {
    if csv_value.is_empty() {
        Ok(rusqlite::types::Value::Null)
    } else {
        parse_integer(csv_value)
    }
}

fn parse_text(csv_value: &str) -> Result<rusqlite::types::Value> {
    Ok(rusqlite::types::Value::Text(csv_value.to_string()))
}

fn build_insert_statement<'conn>(
    conn: &'conn Connection,
    descriptor: &TableDescriptor,
) -> Result<rusqlite::Statement<'conn>> {
    let mut sql = format!("INSERT INTO {} (timestamp, ordinal", descriptor.table_name);

    for column in descriptor.columns.iter() {
        sql.push_str(", ");
        sql.push_str(column.column_name);
    }

    sql.push_str(&format!(
        ") VALUES (?, \
        (SELECT COALESCE(MAX(ordinal), -1) + 1 FROM {} WHERE timestamp = ?)",
        descriptor.table_name
    ));

    for _ in 0..descriptor.columns.len() {
        sql.push_str(", ?");
    }

    sql.push(')');

    let statement = conn.prepare(&sql)?;
    Ok(statement)
}

fn create_table_if_not_exists(conn: &Connection, descriptor: &TableDescriptor) -> Result<()> {
    let mut sql = format!("CREATE TABLE IF NOT EXISTS {} (", descriptor.table_name);

    sql.push_str("timestamp INTEGER NOT NULL, ordinal INTEGER NOT NULL");

    for column in descriptor.columns.iter() {
        sql.push_str(", ");
        sql.push_str(column.column_name);
        sql.push(' ');
        sql.push_str(column_type_to_sql(column));
    }
    sql.push_str(", PRIMARY KEY (timestamp, ordinal))");
    conn.execute(&sql, [])?;

    for column in descriptor.columns.iter() {
        if !matches!(column.column_type, ColumnType::ExternalText) {
            continue;
        }
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {}_{} ( \
            id INTEGER NOT NULL, \
            {} TEXT UNIQUE NOT NULL, \
            PRIMARY KEY (id))",
            descriptor.table_name, column.column_name, column.column_name
        );
        conn.execute(&sql, [])?;
    }

    Ok(())
}

fn column_type_to_sql(column: &ColumnDescriptor) -> &str {
    match column.column_type {
        ColumnType::Boolean => "INTEGER NOT NULL",
        ColumnType::Integer => "INTEGER NOT NULL",
        ColumnType::OptionalInteger => "INTEGER",
        ColumnType::Text => "TEXT NOT NULL",
        ColumnType::ExternalText => "INTEGER NOT NULL",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_timestamp() {
        assert_eq!(
            parse_timestamp("20210825122527.278673700").unwrap(),
            1629894327278673700
        );

        // Test lower boundary for each component of the timestamp
        assert_eq!(
            parse_timestamp("16780101000000.000000000").unwrap(),
            -9214560000000000000
        );

        // Test upper boundary for each component of the timestamp
        assert_eq!(
            parse_timestamp("22611231235959.999999999").unwrap(),
            9214646399999999999
        );

        // Test boundary for the whole timestamp
        assert_eq!(parse_timestamp("19700101000000.000000000").unwrap(), 0);

        // Test leap second (where the second part is 60). This is nonsensical for unix timestamps
        // but chrono still parses it and apparently adds 1 second to the timestamp.
        assert_eq!(
            parse_timestamp("20150630185960.000000000").unwrap(),
            1435690800000000000
        );

        // Test timestamps before Unix epoch
        assert_eq!(
            parse_timestamp("19691231235959.000000000").unwrap(),
            -1000000000
        );

        // Test invalid length
        assert!(parse_timestamp("19700101000000.00000000").is_err());

        // Test invalid format (non-numeric characters)
        assert!(parse_timestamp("1970a101000000.000000000").is_err());

        // Test invalid nanoseconds format (non-numeric characters)
        assert!(parse_timestamp("19700101000000.a00000000").is_err());
    }
}
