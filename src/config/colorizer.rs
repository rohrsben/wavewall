use crate::AppError;
use crate::user_data::PixelInfo;
use hex_color::HexColor;
use mlua::{Value, Function};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Colorizer {
    Function(Function),
    Table {
        default: Converter,
        conversions: HashMap<HexColor, Converter>
    },
}

impl Colorizer {
    pub fn parse(input: Value) -> Result<Option<Self>, AppError> {
        match input {
            Value::Nil => Ok(None),
            Value::Function(func) => Ok(Some(Self::Function(func))),
            Value::Table(table) => {
                if table.is_empty() {
                    return Err(AppError::ConfigEmptyTable(
                        format!("colorizer")
                    ))
                }

                let default = Converter::parse(
                    table.get::<Value>("default")?, 
                    "default"
                )?;

                let mut conversions = HashMap::new();
                for pair in table.pairs::<mlua::String, Value>() {
                    let (original_color, converter) = pair?;

                    let original_color = original_color.to_string_lossy();

                    let converter = Converter::parse(
                        converter,
                        &original_color
                    )?;

                    let original_color = if original_color == "default" {
                        continue
                    } else {
                        HexColor::parse(&original_color)?
                    };

                    conversions.insert(original_color, converter);
                }

                Ok(Some(Self::Table { 
                    default, 
                    conversions 
                }))
            }
            _ => Err(AppError::ConfigType(
                format!("colorizer"), 
                format!("nil, table, function"), 
                input.type_name().to_string()
            ))
        }
    }

    pub fn apply(&self, input: PixelInfo) -> Result<HexColor, AppError> {
        let call = |func: &Function, input: PixelInfo| {
            let original_color = input.color.display_rgba().to_string();
            let output = func.call(input)?;

            match output {
                Value::String(str) => {
                    let str = str.to_string_lossy();
                    match HexColor::parse(&str) {
                        Ok(color) => Ok(color),
                        Err(e) => Err(AppError::Runtime(
                            format!("When calling colorizer on '{original_color}': failed to parse '{str}':\n  {e}")
                        ))
                    }
                }
                Value::Table(table) => {
                    let get_channel = |color: &str| {
                        let channel_val = table.get::<Value>(color)?;
                        match table.get::<Value>(color)? {
                            Value::Nil => {
                                if color == "a" { 
                                    Ok(255 as u8)
                                } else {
                                    Err(AppError::Runtime(
                                        format!("When calling colorizer on '{original_color}': Response had no '{color}' field")
                                    ))
                                }
                            }
                            Value::Integer(int) => {
                                if (0..=255).contains(&int) {
                                    Ok(int as u8)
                                } else {
                                    Err(AppError::Runtime(
                                        format!("When calling colorizer on '{original_color}': Field '{color}' was out of bounds ({int})")
                                    ))
                                }
                            }
                            _ => Err(AppError::Runtime(
                                format!("When calling colorizer on '{original_color}': incorrect type for field '{color}'\n  Expected: positive number in [0, 255]\n  Got: {}", channel_val.type_name())
                            ))
                        }
                    };

                    let r = get_channel("r")?;
                    let g = get_channel("g")?;
                    let b = get_channel("b")?;
                    let a = get_channel("a")?;

                    Ok(HexColor::rgba(r, g, b, a))
                }
                _ => Err(AppError::RuntimeColorizerReturnType(
                    format!("{original_color}"),
                    output.type_name().to_string()
                ))
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

#[derive(Debug)]
pub enum Converter {
    Identity,
    Direct(HexColor),
    Function(Function)
}

impl Converter {
    pub fn parse(input: Value, location: &str) -> Result<Self, AppError> {
        match input {
            Value::Nil => Ok(Converter::Identity),
            Value::String(str) => {
                let color = HexColor::parse(&str.to_string_lossy())?;
                Ok(Self::Direct(color))
            }
            Value::Function(func) => Ok(Self::Function(func)),
            _ => Err(AppError::ConfigType(
                format!("colorizer.{location}"),
                format!("string, function"),
                input.type_name().to_string()
            ))
        }
    }
}
