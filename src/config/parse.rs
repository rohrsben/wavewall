use crate::config::error::ConfigError;

pub fn string(input: mlua::Value, location: &'static str) -> Result<Option<String>, ConfigError> {
    match input {
        mlua::Value::Nil => Ok(None),
        mlua::Value::String(str) => Ok(Some(str.to_string_lossy())),
        _ => Err(ConfigError::Type(location, "string", input.type_name().to_string()))
    }
}
