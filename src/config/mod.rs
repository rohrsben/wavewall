pub mod output;
pub mod generation;

use std::{env, fs};

use mlua::{Lua};

use crate::error::AppError;

#[derive(Debug)]
pub struct Config {
    lua: Lua,
    pub output: output::Output,
    pub generation: generation::Generation,
}

pub fn parse() -> Result<Config, AppError> {
    let lua = Lua::new();

    let config = fs::read_to_string("wavewall.lua")?;
    let config = lua.load(config)
        .set_name("@wavewall.lua")
        .eval::<mlua::Table>()?;

    let output = output::parse(
        config.get::<mlua::Value>("output")?
    )?;

    let generation = generation::parse(
        config.get::<mlua::Value>("generation")?
    )?;

    Ok(Config {
        lua,
        output,
        generation
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
