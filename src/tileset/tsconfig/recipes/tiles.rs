use std::collections::HashMap;

use crate::error::AppError;

#[derive(Debug)]
pub enum Tiles {
    Nil,
    List(Vec::<String>),
    Table(HashMap<String, i64>),
}

pub fn parse(input: mlua::Value, tileset: &str, recipe: &str) -> Result<Tiles, AppError> {
    match input {
        mlua::Value::Nil => Ok(Tiles::Nil),
        mlua::Value::Table(contents) => {
            if contents.sequence_values::<mlua::Value>().count() > 0 {
                let mut tile_names = Vec::new();

                for item in contents.sequence_values::<mlua::Value>() {
                    let item = item?;
                    match item {
                        mlua::Value::String(str) => tile_names.push(str.to_string_lossy()),
                        _ => return Err(AppError::ConfigTypeListItem(
                            format!("{tileset}.recipes.{recipe}.tiles"),
                            format!("string"),
                            item.type_name().to_string()
                        ))
                    }
                }

                return Ok(Tiles::List(tile_names))
            } else {
                if contents.is_empty() {
                    return Err(AppError::ConfigEmptyTable(
                        format!("{tileset}.recipes.{recipe}.tiles")
                    ))
                }

                let mut tile_weights = HashMap::new();

                for pair in contents.pairs::<mlua::String, mlua::Value>() {
                    let (tile_name, weight) = pair?;

                    let tile_name = tile_name.to_string_lossy();
                    let weight = match weight {
                        mlua::Value::Integer(int) => int,
                        _ => return Err(AppError::ConfigTypeTableItem(
                            format!("{tileset}.recipes.{recipe}.tiles"), 
                            format!("integer"), 
                            weight.type_name().to_string()
                        ))
                    };

                    tile_weights.insert(tile_name, weight);
                }

                return Ok(Tiles::Table(tile_weights))
            }
        }
        _ => Err(AppError::ConfigType(
            format!("{tileset}.recipes.{recipe}.tiles"), 
            format!("nil, list of string, table"),
            input.type_name().to_string()
        ))
    }
}
