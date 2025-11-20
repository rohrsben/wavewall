use crate::AppError;
use crate::config::parse;
use mlua::Value;

#[derive(Debug)]
pub struct Info {
    pub name: String,
    pub tile_size: Option<usize>,
}

impl Info {
    pub fn parse(input: Value) -> Result<Self, AppError> {
        match input {
            Value::Table(table) => {
                let name = parse::string_necessary(
                    table.get::<Value>("name")?, 
                    format!("tileset.info.name")
                )?;

                let tile_size = parse::uint(
                    table.get::<Value>("tile_size")?, 
                    format!("tileset.info.tile_size")
                )?;

                Ok(Self {
                    name,
                    tile_size
                })
            }
            _ => Err(AppError::ConfigType(
                format!("tileset.info"),
                format!("table"),
                input.type_name().to_string()
            ))
        }
    }
}
