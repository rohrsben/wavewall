use crate::error::AppError;

#[derive(Debug)]
pub enum Tiles {
    Nil,
    List(Vec::<String>),
    // TODO Table {} 
}

pub fn parse(input: mlua::Value, recipe_name: &str) -> Result<Tiles, AppError> {
    match input {
        mlua::Value::Nil => Ok(Tiles::Nil),
        mlua::Value::Table(contents) => {
            let mut tile_names = Vec::new();

            for val in contents.sequence_values::<mlua::Value>() {
                let val = val?;
                match val {
                    mlua::Value::String(str) => tile_names.push(str.to_string_lossy()),
                    _ => {
                        let location = format!("tileset.recipes.{}.tiles", recipe_name);
                        return Err(AppError::ConfigTypeListItemSpecific(location, "string", val.type_name().to_string()))
                    }
                }
            }

            Ok(Tiles::List(tile_names))
        }
        _ => {
            let location = format!("tileset.recipes.{}.tiles", recipe_name);
            Err(AppError::ConfigTypeSpecific(location, "nil, list", input.type_name().to_string()))
        }
    }
}
