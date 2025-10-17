use crate::error::AppError;

pub fn string(input: mlua::Value, location: &'static str) -> Result<Option<String>, AppError> {
    match input {
        mlua::Value::Nil => Ok(None),
        mlua::Value::String(str) => Ok(Some(str.to_string_lossy())),
        _ => Err(AppError::ConfigType(location, "nil, string", input.type_name().to_string()))
    }
}


