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
            let width = parse::uint_necessary(
                contents.get::<mlua::Value>("width")?,
                "wavewall.output.size.width".to_string()
            )?;

            let height = parse::uint_necessary(
                contents.get::<mlua::Value>("height")?,
                "wavewall.output.size.height".to_string()
            )?;

            Ok(Size {
                width: width as usize,
                height: height as usize
            })
        }
        _ => Err(AppError::ConfigType(
            format!("wavewall.output.size"),
            format!("table"),
            input.type_name().to_string()
        ))
    }
}
