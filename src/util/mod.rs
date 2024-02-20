pub(crate) use conversions::*;
pub(crate) use filter::*;
pub(crate) use load::*;
pub(crate) use normalize_sql::*;
pub(crate) use overlap::*;
pub(crate) use parse_csv::*;
pub(crate) use plot::*;
pub(crate) use read_stdin_string::*;
pub(crate) use statistics::*;
pub(crate) use type_cast::*;

mod conversions;
mod filter;
mod load;
mod normalize_sql;
mod overlap;
mod parse_csv;
mod plot;
mod read_stdin_string;
mod statistics;
mod type_cast;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
