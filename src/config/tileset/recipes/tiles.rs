use mlua::{Table, Value};

use crate::{config::Location, error::AppError};

pub fn parse(input: Value, loc: &Location) -> Result<Option<Vec<(String, usize)>>, AppError> {
    let loc = loc.add_parent("tiles");

    match input {
        Value::Nil => Ok(None),
        Value::Table(table) => parse_table(table, &loc),

        _ => Err(AppError::IncorrectType {
            location: loc.to_string(),
            expected: format!("nil, list of string, table"),
            got: input.type_name().to_string()
        })
    }
}

fn parse_table(table: Table, loc: &Location) -> Result<Option<Vec<(String, usize)>>, AppError> {
    if table.is_empty() { return Err(AppError::EmptyTable(loc.to_string())) }

    let mut options = Vec::new();

    if table.sequence_values::<Value>().count() > 0 {
        // list of string path
        for item in table.sequence_values::<Value>() {
            let item = item?;
            match item {
                Value::String(str) => options.push((str.to_string_lossy(), 1)),
                _ => return Err(AppError::IncorrectType {
                    location: format!("{loc} list item"),
                    expected: format!("string"),
                    got: item.type_name().to_string()
                })
            }
        }
    } else {
        // table path
        for pair in table.pairs::<mlua::String, Value>() {
            let (item, weight) = pair?;

            let item = item.to_string_lossy();

            let weight = if let Value::Integer(int) = weight && int > 0 {
                int as usize
            } else {
                let loc = loc.add_parent(&item);
                return Err(AppError::IncorrectType {
                    location: loc.to_string(),
                    expected: format!("positive number"),
                    got: weight.type_name().to_string()
                })
            };

            options.push((item, weight));
        }
    }

    Ok(Some(options))
}
