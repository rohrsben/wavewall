mod converter;

use converter::Converter;

use crate::{config::Location, AppError};
use crate::config::parse;
use crate::user_data::PixelInfo;
use hex_color::HexColor;
use mlua::{Function, Table, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Colorizer {
    Function(Function),
    Table {
        default: Converter,
        conversions: HashMap<HexColor, Converter>
    },
}

pub fn parse(input: Value, loc: &Location) -> Result<Option<Colorizer>, AppError> {
    let loc = loc.add_parent("colorizer");

    match input {
        Value::Nil => Ok(None),
        Value::Function(func) => Ok(Some( Colorizer::Function(func) )),
        Value::Table(table) => parse_table(table, &loc),

        _ => Err(AppError::IncorrectType {
            location: loc.to_string(),
            expected: format!("nil, table, function"),
            got: input.type_name().to_string()
        })
    }
}

fn parse_table(table: Table, loc: &Location) -> Result<Option<Colorizer>, AppError> {
    if table.is_empty() {
        return Err(AppError::EmptyTable(loc.to_string()))
    }

    let default = converter::parse(
        table.get::<Value>("default")?,
        &loc.add_parent("default")
    )?;

    let mut conversions = HashMap::new();
    for pair in table.pairs::<mlua::String, Value>() {
        let (color, converter) = pair?;

        let color = color.to_string_lossy();
        let converter = converter::parse(
            converter,
            &loc.add_parent(&color)
        )?;

        let color = if color == "default" {
            continue
        } else {
            HexColor::parse(&color)?
        };

        conversions.insert(color, converter);
    }

    Ok(Some(Colorizer::Table {
        default,
        conversions
    }))
}

impl Colorizer {
    pub fn apply(&self, input: PixelInfo) -> Result<HexColor, AppError> {
        let call = |func: &Function, input: PixelInfo| {
            let original_color = input.color;
            let output = func.call(input)?;

            match output {
                Value::Nil => Ok(original_color),
                _ => match parse::color(output) {
                    Ok(color) => Ok(color),
                    Err(e) => Err(AppError::Runtime(
                        format!("When calling colorizer on '{}': {}", original_color.display_rgba(), e)
                    ))
                }
            }
        };

        match self {
            Colorizer::Function(func) => call(func, input),
            Colorizer::Table { default, conversions } => {
                let converter = match conversions.get(&input.color) {
                    Some(converter) => converter,
                    None => default
                };

                match converter {
                    Converter::Identity => Ok(input.color),
                    Converter::Direct(new_color) => Ok(*new_color),
                    Converter::Function(func) => call(func, input)
                }
            }
        }
    }
}

