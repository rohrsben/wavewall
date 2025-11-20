use crate::AppError;
use mlua::Value;
use rand::prelude::*;

pub fn string(input: mlua::Value, location: String) -> Result<Option<String>, AppError> {
    match input {
        mlua::Value::Nil => Ok(None),
        mlua::Value::String(str) => Ok(Some(str.to_string_lossy())),
        _ => Err(AppError::ConfigType(location, format!("nil, string"), input.type_name().to_string()))
    }
}

pub fn string_necessary(input: mlua::Value, location: String) -> Result<String, AppError> {
    match input {
        mlua::Value::String(str) => Ok(str.to_string_lossy()),
        _ => Err(AppError::ConfigType(location, format!("string"), input.type_name().to_string()))
    }
}

pub fn slost(input: Value, location: String) -> Result<Option<String>, AppError> {
    match input {
        Value::Nil => Ok(None),
        Value::String(str) => Ok(Some(str.to_string_lossy())),
        Value::Table(table) => {
            if table.is_empty() {
                return Err(AppError::ConfigEmptyTable(
                    location
                ))
            }

            let mut options = Vec::new();

            if table.sequence_values::<Value>().count() > 0 {
                // list of string path
                for item in table.sequence_values::<Value>() {
                    let item = item?;
                    match item {
                        Value::String(str) => options.push((str.to_string_lossy(), 1)),
                        _ => return Err(AppError::ConfigTypeListItem(
                            location,
                            format!("string"),
                            item.type_name().to_string()
                        ))
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
                        return Err(AppError::ConfigTypeTableItem (
                            format!("{location}.{item}"),
                            format!("positive number"),
                            weight.type_name().to_string()
                        ))
                    };

                    options.push((item, weight))
                }
            }

            let choice = options.choose_weighted(&mut rand::rng(), |it| it.1)?;
            Ok(Some(choice.0.clone()))
        }
        _ => Err(AppError::ConfigType(
            location,
            format!("nil, string, list of string, table"),
            input.type_name().to_string()
        ))
    }
}

pub fn uint(input: mlua::Value, location: String) -> Result<Option<usize>, AppError> {
    match input {
        mlua::Value::Nil => Ok(None),
        mlua::Value::Integer(int) => {
            if int < 0 {
                return Err(AppError::ConfigType(location, format!("nil, positive number"), input.type_name().to_string()))
            }

            Ok(Some(int as usize))
        }
        _ => Err(AppError::ConfigType(location, format!("nil, positive integer"), input.type_name().to_string()))
    }
}

pub fn uint_necessary(input: mlua::Value, location: String) -> Result<usize, AppError> {
    match input {
        mlua::Value::Integer(int) => {
            if int < 0 {
                return Err(AppError::ConfigType(location, format!("positive number"), input.type_name().to_string()))
            }

            Ok(int as usize)
        }
        _ => Err(AppError::ConfigType(location, format!("positive integer"), input.type_name().to_string()))
    }
}

// pub fn int(input: mlua::Value, location: String) -> Result<Option<i64>, AppError> {
//     match input {
//         mlua::Value::Nil => Ok(None),
//         mlua::Value::Integer(int) => Ok(Some(int)),
//         _ => Err(AppError::ConfigType(location, format!("nil, number"), input.type_name().to_string()))
//     }
// }

// pub fn int_necessary(input: mlua::Value, location: String) -> Result<i64, AppError> {
//     match input {
//         mlua::Value::Integer(int) => Ok(int),
//         _ => Err(AppError::ConfigType(location, format!("number"), input.type_name().to_string()))
//     }
// }

pub fn bool(input: mlua::Value, default: bool, location: String) -> Result<bool, AppError> {
    match input {
        mlua::Value::Nil => Ok(default),
        mlua::Value::Boolean(b) => Ok(b),
        _ => Err(AppError::ConfigType(location, format!("nil, boolean"), input.type_name().to_string()))
    }
}

pub fn bool_necessary(input: mlua::Value, location: String) -> Result<bool, AppError> {
    match input {
        mlua::Value::Boolean(b) => Ok(b),
        _ => Err(AppError::ConfigType(location, format!("boolean"), input.type_name().to_string()))
    }
}

pub fn func(input: mlua::Value, location: String) -> Result<Option<mlua::Function>, AppError> {
    match input {
        mlua::Value::Nil => Ok(None),
        mlua::Value::Function(func) => Ok(Some(func)),
        _ => Err(AppError::ConfigType(location, format!("nil, function"), input.type_name().to_string()))
    }
}

pub fn table_necessary(input: mlua::Value, location: String) -> Result<mlua::Table, AppError> {
    match input {
        mlua::Value::Table(table) => {
            if table.is_empty() {
                return Err(AppError::ConfigEmptyTable(
                    location
                ))
            }

            Ok(table)
        }
        _ => Err(AppError::ConfigType(location, format!("table"), input.type_name().to_string()))
    }
}


