pub mod size;

use crate::error::AppError;
use crate::parse;

#[derive(Debug)]
pub struct Output {
    filename: Option<String>,
    directory: Option<String>,
    pub size: size::Size,
}

impl Output {
    pub fn filepath(&self) -> String {
        let name = match &self.filename {
            Some(str) => str.clone(),
            None => String::from("result.png")
        };

        let dir = match &self.directory {
            Some(str) => str.clone(),
            None => crate::config::config_dir()
        };

        format!("{}/{}", dir, name)
    }
}

pub fn parse(input: mlua::Value) -> Result<Output, AppError> {
    match input {
        mlua::Value::Table(contents) => {
            let filename = parse::string(
                contents.get::<mlua::Value>("filename")?,
                "wavewall.output.filename".to_string()
            )?;

            let directory = parse::string(
                contents.get::<mlua::Value>("directory")?,
                "wavewall.output.directory".to_string()
            )?;

            let size = size::parse(
                contents.get::<mlua::Value>("size")?,
            )?;

            Ok(Output { 
                filename,
                directory,
                size,
            })
        }
        _ => Err(AppError::ConfigType(
            "wavewall.output".to_string(),
            "nil, table",
            input.type_name().to_string()
        ))
    }
}
