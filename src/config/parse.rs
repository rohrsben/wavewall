use crate::error::TypeError;
use mlua::Value;
use hex_color::{self, HexColor};

pub fn string(input: Value) -> Result<Option<String>, TypeError> {
    let err = Err(TypeError {
        expected: format!("nil, string"),
        got: input.type_name().to_string()
    });

    match input {
        Value::Nil => Ok(None),
        Value::String(str) => Ok(Some(str.to_string_lossy())),
        _ => err
    }
}

pub fn string_necessary(input: Value) -> Result<String, TypeError> {
    let err = Err(TypeError {
        expected: format!("string"),
        got: input.type_name().to_string()
    });

    match input {
        Value::String(str) => Ok(str.to_string_lossy()),
        _ => err
    }
}

pub fn uint(input: Value) -> Result<Option<usize>, TypeError> {
    let err = Err(TypeError {
        expected: format!("nil, positive integer"),
        got: input.type_name().to_string()
    });

    match input {
        Value::Nil => Ok(None),
        Value::Integer(int) => {
            if int < 0 { return err }

            Ok(Some(int as usize))
        }
        _ => err
    }
}

pub fn uint_necessary(input: Value) -> Result<usize, TypeError> {
    let err = Err(TypeError {
        expected: format!("positive integer"),
        got: input.type_name().to_string()
    });

    match input {
        Value::Integer(int) => {
            if int < 0 { return err }

            Ok(int as usize)
        }
        _ => err
    }
}

pub fn bool(input: Value) -> Result<Option<bool>, TypeError> {
    let err = Err(TypeError {
        expected: format!("nil, boolean"),
        got: input.type_name().to_string()
    });

    match input {
        Value::Nil => Ok(None),
        Value::Boolean(b) => Ok(Some(b)),
        _ => err
    }
}

pub fn func(input: Value) -> Result<Option<mlua::Function>, TypeError> {
    let err = Err(TypeError {
        expected: format!("nil, function"),
        got: input.type_name().to_string()
    });

    match input {
        Value::Nil => Ok(None),
        Value::Function(func) => Ok(Some(func)),
        _ => err
    }
}

// RUNTIME CONVERTERS

pub fn color(input: Value) -> Result<HexColor, String> {
    match input {
        Value::String(str) => {
            match HexColor::parse(&str.to_string_lossy()) {
                Ok(color) => Ok(color),
                Err(e) => Err(format!("{e}"))
            }
        }
        Value::Table(table) => {
            let get_channel = |color: &str| {
                let channel_val = match table.get::<Value>(color) {
                    Ok(val) => val,
                    Err(e) => return Err(format!("{e}"))
                };

                match channel_val {
                    Value::Nil => {
                        if color == "a" {
                            Ok(255u8)
                        } else {
                            Err(format!("Input had no '{color}' field"))
                        }
                    }
                    Value::Integer(int) => {
                        if (0..=255).contains(&int) {
                            Ok(int as u8)
                        } else {
                            Err(format!("Field '{color}' was out of bounds ({int})"))
                        }
                    }
                    _ => Err(format!("Incorrect type for field '{color}'\n  Expected: positive number in [0, 255]\n  Got: {}", channel_val.type_name()))
                }
            };

            let r = get_channel("r")?;
            let g = get_channel("g")?;
            let b = get_channel("b")?;
            let a = get_channel("a")?;

            Ok(HexColor::rgba(r, g, b, a))
        }
        _ => Err(format!("Incorrect type:\n  Expected: color\n  Got: {}", input.type_name()))
    }
}
