use crate::AppError;
use crate::image::Transform;
use mlua::Value;
use std::str::FromStr;

#[derive(Debug)]
pub struct Pseudotile {
    pub name: String,
    pub original: String,
    pub transform: Transform,
}

impl Pseudotile {
    pub fn parse_all(input: Value) -> Result<Option<Vec<Self>>, AppError> {
        match input {
            Value::Nil => Ok(None),
            Value::Table(table) => {
                if table.is_empty() {
                    return Err(AppError::ConfigEmptyTable(
                        format!("tileset.pseudotiles")
                    ))
                }

                let mut pseudotiles = Vec::new();
                for pair in table.pairs::<mlua::String, Value>() {
                    let (original, pseudos) = pair?;
                    let original = original.to_string_lossy();

                    let mut pseudos = Self::parse(pseudos, &original)?;
                    pseudotiles.append(&mut pseudos);
                }

                Ok(Some(pseudotiles))
            }
            _ => Err(AppError::ConfigType(
                format!("tileset.pseudotiles"),
                format!("nil, table"),
                input.type_name().to_string()
            ))
        }
    }

    fn parse(input: Value, original: &str) -> Result<Vec<Self>, AppError> {
        match input {
            Value::Table(table) => {
                if table.is_empty() {
                    return Err(AppError::ConfigEmptyTable(
                        format!("tileset.pseudotiles.{original}")
                    ))
                }

            // TODO it feels like theres opportunity for more code clarity here
            let mut pseudos = Vec::new();
            for pair in table.pairs::<mlua::String, mlua::Value>() {
                let (name, transform) = pair?;
                let name = name.to_string_lossy();

                let transform_type = transform.type_name().to_string();
                let transform = if let mlua::Value::String(str) = transform
                    && let Ok(t) = Transform::from_str(&str.to_string_lossy()) {
                    t
                } else {
                    return Err(AppError::ConfigType(
                        format!("tileset.pseudotiles.{original}.{name}"),
                        format!("string in ('90', '180', '270', 'horizontal', 'vertical', 'diagonal', 'antidiagonal')"),
                        transform_type
                    ))
                };

                pseudos.push(Pseudotile {
                    name,
                    original: original.to_owned(),
                    transform
                });
            }

            Ok(pseudos)
            }
            _ => Err(AppError::ConfigType(
                format!("tileset.pseudotiles.{original}"),
                format!("table"),
                input.type_name().to_string()
            ))
        }
    }
}
