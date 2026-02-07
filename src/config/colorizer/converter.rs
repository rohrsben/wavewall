use hex_color::HexColor;
use mlua::{Function, Value};

use crate::{config::{parse, Location}, error::AppError};

#[derive(Debug)]
pub enum Converter {
    Identity,
    Direct(HexColor),
    Function(Function)
}

// here, the loc already includes the color we're trying to create a converter for
pub fn parse(input: Value, loc: &Location) -> Result<Converter, AppError> {
    match input {
        // this can only be hit by colorizer.default
        Value::Nil => Ok(Converter::Identity),
        
        Value::Function(func) => Ok(Converter::Function(func)),
        Value::String(_) | Value::Table(_) => parse_color(input, &loc),

        _ => Err(AppError::IncorrectType {
            location: loc.to_string(),
            expected: format!("color, function (PixelInfo -> color)"),
            got: input.type_name().to_string()
        })
    }
}

fn parse_color(color: Value, loc: &Location) -> Result<Converter, AppError> {
    let color = match parse::color(color) {
        Ok(color) => color,
        Err(err) => return Err(AppError::Runtime(format!("At {loc}:\n  {err}")))
    };

    Ok(Converter::Direct(color))
}
