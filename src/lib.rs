extern crate polars;
use polars::error::PolarsResult;
use polars::frame::DataFrame;
use polars::io::prelude::CsvReader;
use polars::prelude::SerReader;

pub fn read_csv(filename: &str) -> PolarsResult<DataFrame> {
    let file = filename;
    CsvReader::from_path(file)?.finish()
}
