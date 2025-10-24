pub mod recipes;
pub mod info;

use std::collections::HashMap;

use crate::{error::AppError, parse};

#[derive(Debug)]
pub struct TilesetConfig {
    pub info: info::Info,
    pub selection: Option<String>,
    pub recipes: HashMap<String, recipes::Recipe>
}

pub fn parse(input: mlua::Table, tileset: &str) -> Result<TilesetConfig, AppError> {
    let info = info::parse(
        input.get::<mlua::Value>("info")?,
        tileset
    )?;

    let selection = parse::string(
        input.get::<mlua::Value>("selection")?,
        format!("{tileset}.selection")
    )?;

    let recipes = recipes::parse(
        input.get::<mlua::Value>("recipes")?,
        tileset
    )?;

    Ok(TilesetConfig {
        info,
        selection,
        recipes
    })
}
