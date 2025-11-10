use std::collections::HashMap;

use hex_color::HexColor;

use crate::error::AppError;
use crate::image::pixel_info::PixelInfo;
use crate::parse;

#[derive(Debug)]
pub enum Colorizer {
    Nil,
    Table {
        default: Option<mlua::Function>,
        conversions: HashMap<HexColor, Converter>
    },
    Function(mlua::Function)
}

#[derive(Debug)]
pub enum Converter {
    Direct(HexColor),
    Function(mlua::Function)
}

impl Colorizer {
    pub fn apply(&self, input: PixelInfo) ->  Result<HexColor, AppError> {
        let call = | func: &mlua::Function, input: PixelInfo | {
            let ret = func.call::<mlua::Value>(input)?;

            match ret {
                mlua::Value::String(str) => {
                    let str = str.to_string_lossy();
                    Ok(HexColor::parse(&str)?)
                }
                mlua::Value::Table(contents) => {
                    let r = contents.get::<mlua::Integer>("r")? as u8;
                    let g = contents.get::<mlua::Integer>("g")? as u8;
                    let b = contents.get::<mlua::Integer>("b")? as u8;
                    match contents.get::<mlua::Value>("a")? {
                        mlua::Value::Nil => return Ok(HexColor::rgb(r, g, b)),
                        mlua::Value::Integer(a) => Ok(HexColor::rgba(r, g, b, a as u8)),
                        // TODO make this error more clear
                        _ => return Err(AppError::Runtime(
                            format!("Got unexpected result for alpha channel in colorizer.")
                        ))
                    }
                }
                // TODO make this clearer as well
                _ => Err(AppError::Runtime(
                    format!("Got unexpected result from colorizer function call.")
                ))
            }
        };

        match self {
            Colorizer::Nil => Ok(input.pixel), // in theory this won't be used
            Colorizer::Function(func) => call(func, input),
            Colorizer::Table { default, conversions } => {
                match conversions.get(&input.pixel) {
                    None => match default {
                        None => Ok(input.pixel),
                        Some(func) => call(func, input)
                    }
                    Some(converter) => {
                        match converter {
                            Converter::Direct(color) => Ok(*color),
                            Converter::Function(func) => call(func, input)
                        }
                    }
                }
            }
        }
    }
}

pub fn parse(input: mlua::Value, tileset: &str) -> Result<Colorizer, AppError> {
    match input {
        mlua::Value::Nil => Ok(Colorizer::Nil),
        mlua::Value::Function(func) => Ok(Colorizer::Function(func)),
        mlua::Value::Table(contents) => {
            if contents.is_empty() {
                return Err(AppError::ConfigEmptyTable(
                    format!("{tileset}.colorizer")
                ))
            }

            let default = parse::func(
                contents.get::<mlua::Value>("default")?, 
                format!("{tileset}.colorizer.default")
            )?;

            let mut conversions = HashMap::new();
            let conversions_table = parse::table_necessary(
                contents.get::<mlua::Value>("conversions")?,
                format!("{tileset}.colorizer.conversions")
            )?;
            if conversions_table.is_empty() {
                return Err(AppError::ConfigEmptyTable(
                    format!("{tileset}.colorizer.conversions")
                ))
            }
            for pair in conversions_table.pairs::<mlua::String, mlua::Value>() {
                let (color, converter) = pair?;

                let color = color.to_string_lossy();
                let color = HexColor::parse(&color)?;
                let converter = match converter {
                    mlua::Value::String(str) => {
                        let str = str.to_string_lossy();
                        Converter::Direct(HexColor::parse(&str)?)
                    }
                    mlua::Value::Function(func) => Converter::Function(func),
                    _ => return Err(AppError::ConfigTypeTableItem(
                        format!("{tileset}.colorizer.conversions"), 
                        format!("string, function"),
                        converter.type_name().to_string()
                    ))
                };

                conversions.insert(color, converter);
            }

            return Ok(Colorizer::Table {
                default,
                conversions
            })
        }
        _ => Err(AppError::ConfigType(
            format!("{tileset}.colorizer"),
            format!("nil, table, function"),
            input.type_name().to_string()
        ))
    }
}


