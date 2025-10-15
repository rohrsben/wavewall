pub mod output;
pub mod generation;
pub mod error;

use output::OutputPre;
use error::ParseError;

use crate::config;

use mlua::Lua;
use mlua::Value;
use mlua::LuaSerdeExt;

use std::fs;

#[derive(serde::Deserialize, Debug)]
pub struct ConfigurationPre {
    pub output: Option<OutputPre>,
}

pub fn configuration(lua: &Lua) -> Result<ConfigurationPre, ParseError> {
    let config = match fs::read_to_string(config::config_file()) {
        Ok(str) => str,
        Err(e) => return Err(ParseError::Read(e))
    };

    let config_result = match lua.load(config).eval::<Value>() {
        Ok(res) => res,
        Err(e) => match e {
            mlua::Error::SyntaxError { message, incomplete_input: _ } => {
                let msg_split = message.split_once(" ").unwrap();
                return Err(ParseError::Syntax(msg_split.1.to_string()));
            },
            _ => return Err(ParseError::OtherLua(e))

        }
    };

    let result: ConfigurationPre = match lua.from_value(config_result) {
        Ok(res) => res,
        Err(e) => match e {
            mlua::Error::DeserializeError(msg) => return Err(ParseError::Deserialize(msg)),
            _ => return Err(ParseError::OtherLua(e))
        }
    };

    Ok(result)
}
