pub mod tileset;
pub mod offset;

use crate::error::AppError;

#[derive(Debug)]
pub struct Generation {
    pub tileset: tileset::Tileset,
    pub offset: bool
}

pub fn parse(input: mlua::Value) -> Result<Generation, AppError> {
    match input {
        mlua::Value::Nil => Ok(Generation { 
            tileset: tileset::Tileset::Nil,
            offset: offset::default()
        }),
        mlua::Value::Table(contents) => {
            let tileset = tileset::parse(
                contents.get::<mlua::Value>("tileset")?
            )?;

            let offset = offset::parse(
                contents.get::<mlua::Value>("offset")?
            )?;

            Ok(Generation {
                tileset,
                offset
            })
        }
        _ => Err(AppError::ConfigType(
            format!("wavewall.generation"),
            format!("nil, table"),
            input.type_name().to_string()
        ))
    }
}
