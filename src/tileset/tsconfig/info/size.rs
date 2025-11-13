use crate::error::AppError;
use crate::parse;

#[derive(Debug)]
pub struct Size {
    pub height: usize,
    pub width: usize
}

pub fn parse(input: mlua::Value, tileset: &str) -> Result<Size, AppError> {
    match input {
        mlua::Value::Table(contents) => {
            let height = parse::uint_necessary(
                contents.get::<mlua::Value>("height")?,
                format!("{tileset}.info.size.height")
            )?;

            let width = parse::uint_necessary(
                contents.get::<mlua::Value>("width")?,
                format!("{tileset}.info.size.width")
            )?;

            Ok(Size {
                height: height as usize,
                width: width as usize
            })
        }
        _ => Err(AppError::ConfigType(
            format!("{tileset}.info.size"),
            format!("table"),
            input.type_name().to_string()
        ))
    }
}
