use crate::{AppError, error::TypeErrorLocation};
use crate::config::Location;
use mlua::{Value, Table};
use crate::opt_simple;

#[derive(Debug, PartialEq, Eq)]
pub struct Output {
    pub directory: Option<String>,
    pub filename: Option<String>,
    pub offset: bool,
    pub width: usize,
    pub height: usize,
}

pub fn parse(input: Value, loc: &Location) -> Result<Output, AppError> {
    let loc = loc.add_parent("output");

    match input {
        Value::Table(table) => parse_table(table, &loc),
        _ => Err(TypeErrorLocation {
            location: loc.to_string(),
            expected: format!("table"),
            got: input.type_name().to_string()
        }.into())
    }
}

fn parse_table(table: Table, loc: &Location) -> Result<Output, AppError> {
    opt_simple!(directory, string,         table, loc);
    opt_simple!(filename,  string,         table, loc);
    opt_simple!(offset,    bool,           table, loc);
    opt_simple!(width,     uint_necessary, table, loc);
    opt_simple!(height,    uint_necessary, table, loc);

    // handle offset default
    let offset = offset.unwrap_or_else(|| offset_default());

    Ok(Output {
        directory,
        filename,
        offset,
        width,
        height
    })
}

fn offset_default() -> bool {
    true
}
