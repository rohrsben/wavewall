use crate::error::AppError;

pub fn string(input: mlua::Value, location: &'static str) -> Result<Option<String>, AppError> {
    match input {
        mlua::Value::Nil => Ok(None),
        mlua::Value::String(str) => Ok(Some(str.to_string_lossy())),
        _ => Err(AppError::ConfigType(location, "nil, string", input.type_name().to_string()))
    }
}

pub fn int_definite(input: mlua::Value, location: &'static str) -> Result<i64, AppError> {
    match input {
        mlua::Value::Integer(int) => Ok(int),
        _ => Err(AppError::ConfigType(location, "integer", input.type_name().to_string()))
    }
}
