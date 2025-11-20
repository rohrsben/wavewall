pub mod size;

pub use size::Size;

use crate::AppError;
use crate::config::parse;
use mlua::Value;

#[derive(Debug)]
pub struct Output {
    pub filename: Option<String>,
    pub directory: Option<String>,
    pub size: Size,
}

impl Output {
    pub fn parse(input: Value) -> Result<Self, AppError> {
        match input {
            Value::Table(table) => {
                let filename = parse::string(
                    table.get::<Value>("filename")?, 
                    format!("app.output.filename")
                )?;

                let directory = parse::string(
                    table.get::<Value>("directory")?, 
                    format!("app.output.directory")
                )?;

                let size = Size::parse(
                    table.get::<Value>("size")?
                )?;

                Ok(Self {
                    filename,
                    directory,
                    size
                })
            }
            _ => Err(AppError::ConfigType(
                format!("app.output"),
                format!("nil, table"),
                input.type_name().to_string()
            ))
        }
    }
}
