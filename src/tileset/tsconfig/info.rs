use crate::error::AppError;
use crate::parse;

#[derive(Debug)]
pub struct Info {
    pub tile_size: Option<usize>,
}

pub fn parse(input: mlua::Value, tileset: &str) -> Result<Info, AppError> {
    match input {
        mlua::Value::Nil => Ok(Info { tile_size: None }),
        mlua::Value::Table(contents) => {
            let tile_size = parse::uint(
                contents.get::<mlua::Value>("tile_size")?,
                format!("{tileset}.info.tile_size")
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
