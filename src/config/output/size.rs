use crate::AppError;
use crate::config::parse;
use mlua::Value;

#[derive(Debug)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn parse(input: Value) -> Result<Self, AppError> {
        match input {
            Value::Table(table) => {
                let width = parse::uint_necessary(
                    table.get::<Value>("width")?,
                    "output.size.width".to_string()
                )?;

                let height = parse::uint_necessary(
                    table.get::<Value>("height")?,
                    "output.size.height".to_string()
                )?;

                Ok(Self {
                    width: width as usize,
                    height: height as usize
                })
            }
            _ => Err(AppError::ConfigType(
                format!("output.size"),
                format!("table"),
                input.type_name().to_string()
            ))
        }
    }
}

