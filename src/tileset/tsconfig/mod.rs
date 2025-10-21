pub mod recipes;

use std::collections::HashMap;

use crate::{error::AppError, parse};

#[derive(Debug)]
pub struct TilesetConfig {
    pub selection: Option<String>,
    pub recipes: HashMap<String, recipes::Recipe>
}

pub fn parse(input: mlua::Table) -> Result<TilesetConfig, AppError> {
    let selection = match input.get::<mlua::Value>("selection") {
        Ok(result) => parse::string(result, "tileset.selection")?,
        Err(e) => return Err(AppError::ConfigLua(e))
    };

    let recipes = match input.get::<mlua::Value>("recipes") {
        Ok(result) => recipes::parse(result)?,
        Err(e) => return Err(AppError::ConfigLua(e))
    };

    Ok(TilesetConfig {
        selection,
        recipes
    })
}
