use crate::error::AppError;
use crate::parse;

#[derive(Debug)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

pub fn parse(input: mlua::Value) -> Result<Size, AppError> {
    match input {
        mlua::Value::Table(contents) => {
            let width = match contents.get::<mlua::Value>("width") {
                Ok(result) => parse::int_definite(result, "wavewall.output.size.width")?,
                Err(e) => return Err(AppError::ConfigLua(e))
            };

            let height = match contents.get::<mlua::Value>("height") {
                Ok(result) => parse::int_definite(result, "wavewall.output.size.height")?,
                Err(e) => return Err(AppError::ConfigLua(e))
            };

            Ok(Size {
                width: width as usize,
                height: height as usize
            })
        }
        _ => Err(AppError::ConfigType("wavewall.output.size", "table", input.type_name().to_string()))
    }
}
