use crate::AppError;
use crate::config::parse;
use mlua::Value;

#[derive(Debug)]
pub struct Generation {
    pub tileset: Option<String>,
    pub offset: bool
}

impl Generation {
    pub fn parse(input: Value) -> Result<Self, AppError> {
        match input {
            Value::Nil => Ok(Self::default()),
            Value::Table(table) => {
                if table.is_empty() {
                    return Err(AppError::ConfigEmptyTable(
                        format!("app.generation")
                    ))
                }

                let tileset = parse::slost(
                    table.get::<Value>("tileset")?, 
                    format!("app.generation.tileset")
                )?;

                let offset = parse::bool(
                    table.get::<Value>("offset")?,
                    true,
                    format!("app.generation.offset")
                )?;

                Ok(Self {
                    tileset,
                    offset
                })
            }
            _ => Err(AppError::ConfigType(
                format!("app.generation"),
                format!("nil, table"),
                input.type_name().to_string()
            ))
        }
    }

    fn default() -> Self {
        Self {
            tileset: None,
            offset: true
        }
    }
}
