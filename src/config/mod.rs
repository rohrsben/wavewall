pub mod error;
pub mod parse;
pub mod output;

use std::{env, fs};

use mlua::{Lua};

use error::ConfigError;

#[derive(Debug)]
pub struct Config {
    lua: Lua,
    pub output: output::Output,
}

pub fn parse() -> Result<Config, ConfigError> {
    let lua = Lua::new();

    let config = match fs::read_to_string(config_file()) {
        Ok(contents) => lua.load(contents).set_name("@wavewall.lua"),
        Err(e) => return Err(ConfigError::Read(e))
    };

    let config = match config.eval::<mlua::Table>() {
        Ok(result) => result,
        Err(e) => return Err(ConfigError::GeneralMlua(e))
    };

    let output= match config.get::<mlua::Value>("output") {
        Ok(result) => output::parse(result)?,
        Err(e) => return Err(ConfigError::GeneralMlua(e))
    };

    Ok(Config {
        lua,
        output
    })
}

pub fn config_dir() -> String {
    if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        format!("{xdg}/wavewall")
    } else {
        let user = env::var("USER").unwrap();
        format!("/home/{user}/.config/wavewall")
    }
}

pub fn config_file() -> String {
    format!("{}/wavewall.lua", config_dir())
}
