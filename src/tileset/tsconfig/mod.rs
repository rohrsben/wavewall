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
    let info = match input.get::<mlua::Value>("info") {
        Ok(result) => info::parse(result, tileset)?,
        Err(e) => return Err(AppError::ConfigLua(e))
    };

    let selection = match input.get::<mlua::Value>("selection") {
        Ok(result) => {
            let location = format!("{tileset}.selection");
            parse::string(result, location)?
        }
        Err(e) => return Err(AppError::ConfigLua(e))
    };

    let recipes = match input.get::<mlua::Value>("recipes") {
        Ok(result) => recipes::parse(result, tileset)?,
        Err(e) => return Err(AppError::ConfigLua(e))
    };

    Ok(TilesetConfig {
        info,
        selection,
        recipes
    })
}
