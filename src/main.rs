#[macro_use]
extern crate lazy_static;

use clap::{value_parser, Arg, Command};

use util::Result;

mod command;
mod config;
mod plot;
mod util;

// Ideas
// Classify by number of sorts / joins
// Compare performance of cache-validation query vs full query
// Count number of simultaneously executing queries for each time instant.
// Plot overlap together with execution time
// Output statistics difference between two time periods
//
// Ask for view names from customer and possibly execution plan for some slow view.
// Plot db ping time

fn run() -> Result<()> {
    let matches = Command::new("tracetool")
        .version("0.1.0")
        .author("Easit AB")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            Command::new("import")
                .about("Import CSV data from target into SQLite database")
                .arg(
                    Arg::new("target")
                        .help("The target SQLite database to import data into. Will be created if it does not exist.")
                        .required(true)
                )
                .arg(
                    Arg::new("sources")
                        .help("The source directories containing trace data")
                        .num_args(1..)
                ),
        )
        .subcommand(
            Command::new("show")
                .about("Load plot configuration from YAML file and show the plot")
                .arg(
                    Arg::new("plot.yaml")
                        .help("The plot configuration file to load")
                        .required(true)
                ),
        )
        .subcommand(
            Command::new("compute-overlap")
                .about("Generate table of overlap between queries")
                .arg(
                    Arg::new("database")
                        .help("The target SQLite database to generate table for")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("compute-overlap-pca")
                .about("Perform principal component analysis of overlap vs execution time, per view")
                .arg(
                    Arg::new("database")
                        .help("The target SQLite database to perform PCA on")
                        .required(true)
                )
                .arg(
                    Arg::new("start")
                        .help("Start of time period")
                        .long("start")
                )
                .arg(
                    Arg::new("end")
                        .help("End of time period")
                        .long("end")
                )
        )
        .subcommand(
            Command::new("view-statistics")
                .about("Print statistics for each view")
                .arg(
                    Arg::new("database")
                        .help("The target SQLite database")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("start")
                        .help("Start of time period")
                        .long("start")
                )
                .arg(
                    Arg::new("end")
                        .help("End of time period")
                        .long("end")
                )
        )
        .subcommand(
            Command::new("form-statistics")
                .about("Print statistics for each form")
                .arg(
                    Arg::new("database")
                        .help("The target SQLite database")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("start")
                        .help("Start of time period")
                        .long("start")
                )
                .arg(
                    Arg::new("end")
                        .help("End of time period")
                        .long("end")
                )
        )
        .subcommand(
            Command::new("convert-unit")
                .about("Provide unit conversions for writing manual SQL queries")
                .arg(
                    Arg::new("value")
                        .help("The value to convert. Can be a timestamp or a duration with unit suffix")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("get-cell")
                .about("Extract the cell at a specific row and column in a CSV file")
                .arg(
                    Arg::new("file")
                        .help("The CSV file to read from")
                        .required(true)
                )
                .arg(
                    Arg::new("row")
                        .help("The row number to read from")
                        .value_parser(value_parser!(usize))
                        .required(true)
                )
                .arg(
                    Arg::new("column")
                        .help("The column number to read from")
                        .value_parser(value_parser!(usize))
                        .required(true)
                )
                .arg(
                    Arg::new("delimiter")
                        .help("The delimiter used in the CSV file")
                        .long("delimiter")
                        .default_value(",")
                )
                .arg(
                    Arg::new("headers")
                        .help("The CSV file has headers")
                        .num_args(0)
                        .long("headers")
                        .overrides_with("no-headers")
                )
                .arg(
                    Arg::new("no-headers")
                        .help("The CSV file does not have headers (default)")
                        .num_args(0)
                        .long("no-headers")
                )
        )
        .subcommand(
            Command::new("normalize-sql")
                .about("Normalize SQL query from standard input and print to standard output")
        )
        .subcommand(
            Command::new("build-view-sql-index")
                .about("Create a table of normalized SQL queries for each query that has been encountered")
            .arg(
                Arg::new("target")
                    .help("The target SQLite database")
                    .required(true)
            )
        )
        .subcommand(
            Command::new("match-query-view")
                .about("Find views that match an SQL query. Requires execution of build-view-sql-index first.")
            .arg(
                Arg::new("target")
                    .help("The target SQLite database")
                    .required(true)
            )
        )
        .get_matches();
    match matches.subcommand() {
        Some(("import", matches)) => {
            let target: &String = matches.get_one("target").unwrap();
            let sources: Vec<&String> = matches.get_many::<String>("sources").unwrap().collect();
            command::import_data(target, &sources)?;
        }
        Some(("show", matches)) => {
            let plot_configuration = matches.get_one::<String>("plot.yaml").unwrap();
            println!("Showing plot data from {}", plot_configuration);
            command::plot(plot_configuration)?;
        }
        Some(("compute-overlap", matches)) => {
            let database = matches.get_one::<String>("database").unwrap();
            println!("Computing overlap for {}", database);
            command::compute_overlap(database)?;
        }
        Some(("compute-overlap-pca", matches)) => {
            let database: &String = matches.get_one("database").unwrap();
            let start: Option<&String> = matches.get_one("start");
            let end: Option<&String> = matches.get_one("end");
            command::compute_overlap_pca(
                database,
                start.map(|s| s.as_str()),
                end.map(|s| s.as_str()),
            )?;
        }
        Some(("view-statistics", matches)) => {
            let database: &String = matches.get_one("database").unwrap();
            let start: Option<&String> = matches.get_one("start");
            let end: Option<&String> = matches.get_one("end");
            command::view_statistics(database, start.map(|s| s.as_str()), end.map(|s| s.as_str()))?;
        }
        Some(("form-statistics", matches)) => {
            let database: &String = matches.get_one("database").unwrap();
            let start: Option<&String> = matches.get_one("start");
            let end: Option<&String> = matches.get_one("end");
            command::form_statistics(database, start.map(|s| s.as_str()), end.map(|s| s.as_str()))?;
        }
        Some(("convert-unit", matches)) => {
            let value: &String = matches.get_one("value").unwrap();
            command::convert_unit(value);
        }
        Some(("get-cell", matches)) => {
            let file: &String = matches.get_one("file").unwrap();
            let row: &usize = matches.get_one("row").unwrap();
            let column: &usize = matches.get_one("column").unwrap();
            let delimiter: &String = matches.get_one("delimiter").unwrap();
            let has_headers = matches.get_flag("headers");
            command::get_cell(file, *row, *column, delimiter, has_headers)?;
        }
        Some(("normalize-sql", _)) => {
            command::normalize_sql()?;
        }
        Some(("build-view-sql-index", matches)) => {
            let target: &String = matches.get_one("target").unwrap();
            command::build_view_sql_index(target)?;
        }
        Some(("match-query-view", matches)) => {
            let target: &String = matches.get_one("target").unwrap();
            command::match_query_view(target)?;
        }
        _ => {
            println!("No subcommand was used");
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
