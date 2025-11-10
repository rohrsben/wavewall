use crate::error::AppError;
use crate::image::transform::Transform;

#[derive(Debug)]
pub struct Pseudotile {
    pub name: String,
    pub original: String,
    pub transform: Transform,
}

pub fn parse(input: mlua::Value, tileset: &str) -> Result<Option<Vec<Pseudotile>>, AppError> {
    match input {
        mlua::Value::Nil => Ok(None),
        mlua::Value::Table(contents) => {
            if contents.is_empty() {
                return Err(AppError::ConfigEmptyTable(
                    format!("{tileset}.pseudotiles")
                ))
            }

            let mut pseudotiles = Vec::new();
            for pair in contents.pairs::<mlua::String, mlua::Value>() {
                let (original, pseudos) = pair?;
                let original = original.to_string_lossy();

                let mut pseudos = parse_pseudos(pseudos, &original, tileset)?;
                pseudotiles.append(&mut pseudos);
            }
            
            Ok(Some(pseudotiles))
        },
        _ => Err(AppError::ConfigType(
            format!("{tileset}.pseudotiles"),
            format!("nil, table"),
            input.type_name().to_string()
        ))
    }
}

fn parse_pseudos(input: mlua::Value, original: &str, tileset: &str) -> Result<Vec<Pseudotile>, AppError> {
    match input {
        mlua::Value::Table(contents) => {
            if contents.is_empty() {
                return Err(AppError::ConfigEmptyTable(
                    format!("{tileset}.pseudotiles.{original}")
                ))
            }

            let mut pseudos = Vec::new();
            for pair in contents.pairs::<mlua::String, mlua::Value>() {
                let (name, transform) = pair?;
                let name = name.to_string_lossy();
                let transform = match transform {
                    mlua::Value::String(str) => str.to_string_lossy(),
                    _ => return Err(AppError::ConfigType(
                        format!("{tileset}.pseudotiles.{original}.{name}"),
                        format!("string in ('90', '180', '270', 'horizontal', 'vertical', 'diagonal', 'antidiagonal')"),
                        transform.type_name().to_string()
                    ))
                };

                let transform = match transform.as_str() {
                    "90" => Transform::TurnOnce,
                    "180" => Transform::TurnTwice,
                    "270" => Transform::TurnThrice,
                    "horizontal" => Transform::Horizontal,
                    "vertical" => Transform::Vertical,
                    "diagonal" => Transform::Diagonal,
                    "antidiagonal" => Transform::Antidiagonal,
                    _ => return Err(AppError::ConfigType(
                        format!("{tileset}.pseudotiles.{original}.{name}"),
                        format!("string in ('90', '180', '270', 'horizontal', 'vertical', 'diagonal', 'antidiagonal')"),
                        format!("string '{transform}'")
                    ))
                };

                pseudos.push(Pseudotile {
                    name,
                    original: original.to_string(),
                    transform
                });
            }

            Ok(pseudos)
        }
        _ => Err(AppError::ConfigType(
            format!("{tileset}.pseudotiles.{original}"),
            format!("table"),
            input.type_name().to_string()
        ))
    }
}
