use crate::error::AppError;
use crate::parse;

#[derive(Debug)]
pub struct Info {
    pub tile_size: usize,
}

pub fn parse(input: mlua::Value, tileset: &str) -> Result<Info, AppError> {
    match input {
        mlua::Value::Table(contents) => {
            let tile_size = parse::uint_necessary(
                contents.get::<mlua::Value>("tile_size")?,
                tileset.to_owned()
            )?;

            Ok(Info {
                tile_size
            })
        }
        _ => Err(AppError::ConfigType(
            format!("{tileset}.info"),
            format!("table"),
            input.type_name().to_string()
        ))
    }
}
