use crate::config::Location;
use crate::AppError;
use crate::image::Transform;
use mlua::{Table, Value};
use std::str::FromStr;

#[derive(Debug)]
pub struct Pseudotile {
    pub name: String,
    pub original: String,
    pub transform: Transform,
}

pub fn parse(input: Value, loc: &Location) -> Result<Option<Vec<Pseudotile>>, AppError> {
    let loc = loc.add_parent("pseudotiles");

    match input {
        Value::Nil => Ok(None),
        Value::Table(table) => parse_table(table, &loc),

        _ => Err(AppError::IncorrectType {
            location: loc.to_string(),
            expected: format!("nil, table"),
            got: input.type_name().to_string()
        })
    }
}

fn parse_table(table: Table, loc: &Location) -> Result<Option<Vec<Pseudotile>>, AppError> {
    if table.is_empty() { return Err(AppError::EmptyTable(loc.to_string())) }

    let mut pseudotiles = Vec::new();
    for pair in table.pairs::<mlua::String, Value>() {
        let (original, pseudos) = pair?;
        let original = original.to_string_lossy();

        let loc = loc.add_parent(&original);
        let mut pseudos = parse_pseudotile(pseudos, &loc)?;
        pseudotiles.append(&mut pseudos);
    }

    Ok(Some(pseudotiles))
}

// the final elem in loc here is is the name of the pseudotile original
// (since its the key in the table)
fn parse_pseudotile(input: Value, loc: &Location) -> Result<Vec<Pseudotile>, AppError> {
    match input {
        Value::Table(table) => {
            if table.is_empty() { return Err(AppError::EmptyTable(loc.to_string())) }

            let mut pseudos = Vec::new();
            for pair in table.pairs::<mlua::String, Value>() {
                let (name, transform) = pair?;
                let name = name.to_string_lossy();

                // temp variable because the lua type name won't be accessible later
                let transform_type = transform.type_name().to_string();

                // using this instead of a match because both failures would create the 
                // same type error; this reduces duplication
                let transform = if let Value::String(str) = transform
                && let Ok(t) = Transform::from_str(&str.to_string_lossy()) {
                    t
                } else {
                    return Err(AppError::IncorrectType {
                        location: loc.add_parent(&name).to_string(),
                        expected: format!("string in ('90', '180', '270', 'horizontal', 'vertical', 'diagonal', 'antidiagonal')"),
                        got: transform_type
                    })
                };

                pseudos.push(Pseudotile {
                    name,
                    original: loc.last(),
                    transform
                });
            }

            Ok(pseudos)
        }

        _ => Err(AppError::IncorrectType {
            location: loc.to_string(),
            expected: format!("table"),
            got: input.type_name().to_string()
        })
    }
}
