pub mod output;
pub mod generation;

pub use generation::Generation;
pub use output::Output;

use mlua::Value;
use crate::AppError;

#[derive(Debug)]
pub struct AppConfig {
    pub output: Output,
    pub generation: Generation,
}

impl AppConfig {
    pub fn parse(input: Value) -> Result<Self, AppError> {
        match input {
            Value::Table(table) => {
                let output = Output::parse(
                    table.get::<Value>("output")?
                )?;

                let generation = Generation::parse(
                    table.get::<Value>("generation")?
                )?;

                Ok(Self {
                    output,
                    generation
                })
            }
            _ => Err(AppError::ConfigType(
                format!("app"),
                format!("table"),
                input.type_name().to_string()
            ))
        }
    }
}
