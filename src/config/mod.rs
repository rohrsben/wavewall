use std::{env, fs};

use mlua::{Lua, LuaSerdeExt, Table, Value};
// use mlua::{Error, Lua, LuaSerdeExt, Result, UserData, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ConfigError {
    Read,
    Evaluation,
    Deserialization,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub tileset: String,
}

impl Config {
    pub fn parse_config(lua: &Lua) -> Result<Self, ConfigError> {
        let config = match fs::read_to_string(Self::get_wavewall_config_file()) {
            Ok(str) => str,
            Err(_) => return Err(ConfigError::Read)
        };

        let config_result = match lua.load(config).eval::<Value>() {
            Ok(res) => res,
            Err(_) => return Err(ConfigError::Evaluation)
        };

        let result: Config = match lua.from_value(config_result) {
            Ok(res) => res,
            Err(_) => return Err(ConfigError::Deserialization)
        };

        Ok(result)
    }

    pub fn get_config_dir() -> String {
        if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
            format!("{xdg}/wavewall")
        } else {
            let user = env::var("USER").unwrap();
            format!("/home/{user}/.config/wavewall")
        }
    }

    pub fn get_wavewall_config_file() -> String {
        format!("{}/wavewall.lua", Self::get_config_dir())
    }
}
