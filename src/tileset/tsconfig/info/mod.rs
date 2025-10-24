use crate::error::AppError;

pub mod size;

#[derive(Debug)]
pub struct Info {
    pub size: size::Size,
}

pub fn parse(input: mlua::Value, tileset: &str) -> Result<Info, AppError> {
    match input {
        mlua::Value::Table(contents) => {
            let size = size::parse(
                contents.get::<mlua::Value>("size")?,
                tileset
            )?;

            Ok(Info {
                size
            })
        }
        _ => Err(AppError::ConfigType(
            format!("{tileset}.info"),
            "table",
            input.type_name().to_string()
        ))
    }
}
