pub mod output;

use std::{env, fs};

use mlua::{Lua};

use crate::error::AppError;
use crate::parse;

#[derive(Debug)]
pub struct Config {
    lua: Lua,
    pub output: output::Output,
    pub tileset: Option<String>,
}

pub fn parse() -> Result<Config, AppError> {
    let lua = Lua::new();

    let config = match fs::read_to_string("wavewall.lua") {
        Ok(contents) => lua.load(contents).set_name("@wavewall.lua"),
        Err(e) => return Err(AppError::IO(e))
    };

    let config = match config.eval::<mlua::Table>() {
        Ok(result) => result,
        Err(e) => return Err(AppError::ConfigLua(e))
    };

    let output = match config.get::<mlua::Value>("output") {
        Ok(result) => output::parse(result)?,
        Err(e) => return Err(AppError::ConfigLua(e))
    };

    let tileset = match config.get::<mlua::Value>("tileset") {
        Ok(result) => parse::string(result, "wavewall.tileset")?,
        Err(e) => return Err(AppError::ConfigLua(e))
    };

    Ok(Config {
        lua,
        output,
        tileset
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
