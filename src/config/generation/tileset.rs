use crate::error::AppError;

#[derive(Debug)]
pub enum Tileset {
    Nil,
    String(String),
    List(Vec::<String>),
}

pub fn parse(input: mlua::Value) -> Result<Tileset, AppError> {
    match input {
        mlua::Value::Nil => Ok(Tileset::Nil),
        mlua::Value::String(str) => Ok(Tileset::String(str.to_string_lossy())),
        mlua::Value::Table(contents) => {
            let mut choices = Vec::new();

            for item in contents.sequence_values::<mlua::Value>() {
                let item = item?;
                match item {
                    mlua::Value::String(str) => choices.push(str.to_string_lossy()),
                    _ => return Err(AppError::ConfigTypeListItem(
                        format!("wavewall.generation.tileset"),
                        format!("string"),
                        item.type_name().to_string()
                    ))
                }
            }

            Ok(Tileset::List(choices))
        }
        _ => Err(AppError::ConfigType(
            format!("wavewall.generation.tileset"),
            format!("nil, string, list of string"),
            input.type_name().to_string()
        ))
    }
}
