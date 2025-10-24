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
                    _ => return Err(AppError::ConfigTypeListItem(
                        format!("{tileset}.recipes.{recipe}.tiles"),
                        "string",
                        item.type_name().to_string()
                    ))
                }
            }

            Ok(Tiles::List(tile_names))
        }
        _ => Err(AppError::ConfigType(
            format!("{tileset}.recipes.{recipe}.tiles"), 
            "nil, list of string",
            input.type_name().to_string()
        ))
    }
}
