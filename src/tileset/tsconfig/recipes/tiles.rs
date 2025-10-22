use crate::error::AppError;

#[derive(Debug)]
pub enum Tiles {
    Nil,
    List(Vec::<String>),
    // TODO Table {} 
}

pub fn parse(input: mlua::Value, tileset: &str, recipe: &str) -> Result<Tiles, AppError> {
    match input {
        mlua::Value::Nil => Ok(Tiles::Nil),
        mlua::Value::Table(contents) => {
            let mut tile_names = Vec::new();

            for item in contents.sequence_values::<mlua::Value>() {
                let item = item?;
                match item {
                    mlua::Value::String(str) => tile_names.push(str.to_string_lossy()),
                    _ => {
                        let location = format!("{tileset}.recipes.{recipe}.tiles");
                        return Err(AppError::ConfigTypeListItem(location, "string", item.type_name().to_string()))
                    }
                }
            }

            Ok(Tiles::List(tile_names))
        }
        _ => {
            let location = format!("{tileset}.recipes.{recipe}.tiles");
            Err(AppError::ConfigType(location, "nil, list of string", input.type_name().to_string()))
        }
    }
}
