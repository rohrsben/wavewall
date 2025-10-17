pub mod recipe;

use std::collections::HashMap;

use crate::{error::AppError, parse};

#[derive(Debug)]
pub struct TilesetConfig {
    selection: Option<String>,
    recipes: HashMap<String, recipe::Recipe>
}

pub fn parse(input: mlua::Table) -> Result<TilesetConfig, AppError> {
    let selection = match input.get::<mlua::Value>("selection") {
        Ok(result) => parse::string(result, "tileset.selection")?,
        Err(e) => return Err(AppError::ConfigLua(e))
    };

    let recipes = match input.get::<mlua::Value>("recipes") {
        Ok(result) => recipe::parse(result)?,
        Err(e) => return Err(AppError::ConfigLua(e))
    };

    Ok(TilesetConfig {
        selection,
        recipes
    })
}
