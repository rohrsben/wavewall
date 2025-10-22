use crate::error::AppError;

pub mod size;

#[derive(Debug)]
pub struct Info {
    pub size: size::Size,
}

pub fn parse(input: mlua::Value, tileset: &str) -> Result<Info, AppError> {
    match input {
        mlua::Value::Table(contents) => {
            let size = match contents.get::<mlua::Value>("size") {
                Ok(result) => size::parse(result, tileset)?,
                Err(e) => return Err(AppError::ConfigLua(e))
            };

            Ok(Info {
                size
            })
        }
        _ => {
            let location = format!("{tileset}.info");
            Err(AppError::ConfigType(location, "table", input.type_name().to_string()))
        }
    }
}
