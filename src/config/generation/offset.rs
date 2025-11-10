use crate::error::AppError;

pub fn default() -> bool {
    true
}

pub fn parse(input: mlua::Value) -> Result<bool, AppError> {
    match input {
        mlua::Value::Nil => Ok(default()),
        mlua::Value::Boolean(b) => Ok(b),
        _ => Err(AppError::ConfigType(
            format!("wavewall.generation.offset"), 
            format!("boolean"),
            input.type_name().to_string()
        ))
    }
}
