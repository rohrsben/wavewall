use crate::error::AppError;

#[derive(Debug)]
pub enum Colorizer {
    Nil,
    Function(mlua::Function)
}

pub fn parse(input: mlua::Value, tileset: &str) -> Result<Colorizer, AppError> {
    match input {
        mlua::Value::Nil => Ok(Colorizer::Nil),
        mlua::Value::Function(func) => Ok(Colorizer::Function(func)),
        _ => Err(AppError::ConfigType(
            format!("{tileset}.colorizer"),
            "nil, function",
            input.type_name().to_string()
        ))
    }
}
