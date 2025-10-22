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
            let tileset = match contents.get::<mlua::Value>("tileset") {
                Ok(result) => tileset::parse(result)?,
                Err(e) => return Err(AppError::ConfigLua(e))
            };

            let offset = match contents.get::<mlua::Value>("offset") {
                Ok(result) => offset::parse(result)?,
                Err(e) => return Err(AppError::ConfigLua(e))
            };

            Ok(Generation {
                tileset,
                offset
            })
        }
        _ => Err(AppError::ConfigType("wavewall.generation".to_string(), "nil, table", input.type_name().to_string()))
    }
}
