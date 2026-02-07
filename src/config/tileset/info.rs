use crate::AppError;
use crate::opt_simple;
use crate::config::Location;
use mlua::{Table, Value};

#[derive(Debug)]
pub struct Info {
    pub name: String,
    pub tile_size: Option<usize>,
}

pub fn parse(input: Value, loc: &Location) -> Result<Info, AppError> {
    let loc = loc.add_parent("info");

    match input {
        Value::Table(table) => parse_table(table, &loc),

        _ => Err(AppError::IncorrectType {
            location: loc.to_string(),
            expected: format!("table"),
            got: input.type_name().to_string()
        })
    }
}

fn parse_table(table: Table, loc: &Location) -> Result<Info, AppError> {
    opt_simple!(name,      string_necessary, table, loc);
    opt_simple!(tile_size, uint,             table, loc);

    Ok(Info {
        name,
        tile_size
    })
}
