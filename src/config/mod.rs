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

pub fn generate() -> Result<Config, ConfigError> {
    let lua = Lua::new();

    let config = match fs::read_to_string(config_file()) {
        Ok(contents) => contents,
        Err(e) => return Err(ConfigError::Read(e))
    };

    let config = lua.load(config);
    let config = config.set_name("@wavewall.lua");

    let config = match config.eval::<mlua::Table>() {
        Ok(result) => result,
        Err(e) => return Err(ConfigError::GeneralMlua(e))
    };

    let output_value = match config.get::<mlua::Value>("output") {
        Ok(result) => result,
        Err(e) => return Err(ConfigError::GeneralMlua(e))
    };

    let output = output::parse(output_value)?;

    let config = Config {
        lua,
        output
    };

    Ok(config)
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
