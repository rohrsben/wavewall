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
            let height = match contents.get::<mlua::Value>("height") {
                Ok(result) => {
                    let location = format!("{tileset}.info.size.height");
                    parse::int_necessary(result, location)?
                }
                Err(e) => return Err(AppError::ConfigLua(e))
            };

            let width = match contents.get::<mlua::Value>("width") {
                Ok(result) => {
                    let location = format!("{tileset}.info.size.width");
                    parse::int_necessary(result, location)?
                }
                Err(e) => return Err(AppError::ConfigLua(e))
            };

            Ok(Size {
                height: height as usize,
                width: width as usize
            })
        }
        _ => {
            let location = format!("{tileset}.info.size");
            Err(AppError::ConfigType(location, "table", input.type_name().to_string()))
        }
    }
}
