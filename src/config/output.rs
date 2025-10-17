use crate::error::AppError;
use crate::parse;

#[derive(Debug)]
pub enum Output {
    Nil,
    Table {
        filename: Option<String>,
        directory: Option<String>,
    }
}

impl Output {
    pub fn filepath(&self) -> String {
        match self {
            Output::Nil => format!("{}/result.png", crate::config::config_dir()),
            Output::Table { filename, directory } => {
                let name = match filename {
                    Some(str) => str.clone(),
                    None => String::from("result.png")
                };

                let dir = match directory {
                    Some(str) => str.clone(),
                    None => crate::config::config_dir()
                };

                format!("{}/{}", dir, name)
            }
        }
    }
}

pub fn parse(input: mlua::Value) -> Result<Output, AppError> {
    match input {
        mlua::Value::Nil => Ok(Output::Nil),
        mlua::Value::Table(contents) => {
            let filename = match contents.get::<mlua::Value>("filename") {
                Ok(result) => parse::string(result, "wavewall.output.filename")?,
                Err(e) => return Err(AppError::ConfigLua(e))
            };

            let directory = match contents.get::<mlua::Value>("directory") {
                Ok(result) => parse::string(result, "wavewall.output.directory")?,
                Err(e) => return Err(AppError::ConfigLua(e))
            };

            Ok(Output::Table { filename, directory })
        }
        _ => Err(AppError::ConfigType("wavewall.output", "nil, table", input.type_name().to_string()))
    }
}
