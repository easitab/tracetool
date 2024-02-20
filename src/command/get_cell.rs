use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

use csv::ReaderBuilder;

pub(crate) fn get_cell<P: AsRef<Path>>(
    csv_file: P,
    row: usize,
    column: usize,
    delimiter: &str,
    has_headers: bool,
) -> Result<(), Box<dyn Error>> {
    let delimiter = delimiter.as_bytes();
    if delimiter.len() != 1 {
        eprintln!("Delimiter must be in the 7-bit ASCII range.");
        return Ok(());
    }
    let delimiter = delimiter[0];

    let file = match File::open(csv_file.as_ref()) {
        Ok(file) => file,
        Err(err) => {
            eprintln!(
                "Failed to open file {}: {}",
                csv_file.as_ref().display(),
                err
            );
            return Ok(());
        }
    };

    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(has_headers)
        .delimiter(delimiter)
        .from_reader(reader);

    let mut records = csv_reader.records();

    let record = match records.nth(row) {
        Some(Ok(record)) => record,
        Some(Err(err)) => {
            eprintln!("Failed to read row {}: {}", row, err);
            return Ok(());
        }
        None => {
            eprintln!("Row {} not found", row);
            return Ok(());
        }
    };
    let cell = record.get(column).ok_or("Column not found")?.to_string();

    std::io::stdout().write_all(cell.as_bytes())?;

    Ok(())
}
