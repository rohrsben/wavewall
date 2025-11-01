use crate::error::AppError;

pub fn string(input: mlua::Value, location: String) -> Result<Option<String>, AppError> {
    match input {
        mlua::Value::Nil => Ok(None),
        mlua::Value::String(str) => Ok(Some(str.to_string_lossy())),
        _ => Err(AppError::ConfigType(
            location,
            "nil, string",
            input.type_name().to_string()
        ))
    }
}

pub fn int_necessary(input: mlua::Value, location: String) -> Result<i64, AppError> {
    match input {
        mlua::Value::Integer(int) => Ok(int),
        _ => Err(AppError::ConfigType(location, "integer", input.type_name().to_string()))
    }
}

pub fn bool(input: mlua::Value, location: String) -> Result<Option<bool>, AppError> {
    match input {
        mlua::Value::Nil => Ok(None),
        mlua::Value::Boolean(b) => Ok(Some(b)),
        _ => Err(AppError::ConfigType(location, "nil, boolean", input.type_name().to_string()))
    }
}

pub fn bool_necessary(input: mlua::Value, location: String) -> Result<bool, AppError> {
    match input {
        mlua::Value::Boolean(b) => Ok(b),
        _ => Err(AppError::ConfigType(location, "boolean", input.type_name().to_string()))
    }
}

pub fn func(input: mlua::Value, location: String) -> Result<Option<mlua::Function>, AppError> {
    match input {
        mlua::Value::Nil => Ok(None),
        mlua::Value::Function(func) => Ok(Some(func)),
        _ => Err(AppError::ConfigType(
            location,
            "nil, function",
            input.type_name().to_string()
        ))
    }
}

pub fn table_necessary(input: mlua::Value, location: String) -> Result<mlua::Table, AppError> {
    match input {
        mlua::Value::Table(contents) => Ok(contents),
        _ => Err(AppError::ConfigType(
            location,
            "table",
            input.type_name().to_string()
        ))
    }
}
