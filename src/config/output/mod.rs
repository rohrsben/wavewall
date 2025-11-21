pub mod size;

pub use size::Size;

use crate::AppError;
use crate::config::parse;
use mlua::Value;

#[derive(Debug)]
pub struct Output {
    pub directory: Option<String>,
    pub filename: Option<String>,
    pub offset: bool,
    pub size: Size,
}

impl Output {
    pub fn parse(input: Value) -> Result<Self, AppError> {
        match input {
            Value::Table(table) => {
                if table.is_empty() {
                    return Err(AppError::ConfigEmptyTable(
                        format!("output")
                    ))
                }

                let directory = parse::string(
                    table.get::<Value>("directory")?, 
                    format!("output.directory")
                )?;

                let filename = parse::string(
                    table.get::<Value>("filename")?, 
                    format!("output.filename")
                )?;

                let offset = parse::bool(
                    table.get::<Value>("offset")?,
                    offset_default(),
                    format!("output.offset")
                )?;

                let size = Size::parse(
                    table.get::<Value>("size")?
                )?;

                Ok(Self {
                    directory,
                    filename,
                    offset,
                    size,
                })
            }
            _ => Err(AppError::ConfigType(
                format!("output"),
                format!("table"),
                input.type_name().to_string()
            ))
        }
    }
}

fn offset_default() -> bool {
    true
}
