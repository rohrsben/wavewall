pub mod recipes;
pub mod info;
pub mod pseudotiles;
pub mod colorizer;

use std::collections::HashMap;

use crate::{error::AppError, parse};

pub use recipes::Recipe;
pub use info::Info;
pub use pseudotiles::Pseudotile;
pub use colorizer::Colorizer;

#[derive(Debug)]
pub struct TilesetConfig {
    pub info: Info,
    pub selection: Option<String>,
    pub pseudotiles: Option<Vec<Pseudotile>>,
    pub recipes: HashMap<String, Recipe>,
    pub colorizer: colorizer::Colorizer,
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

    let pseudotiles = pseudotiles::parse(
        input.get::<mlua::Value>("pseudotiles")?,
        tileset
    )?;

    let recipes = recipes::parse(
        input.get::<mlua::Value>("recipes")?,
        tileset
    )?;

    let colorizer = colorizer::parse(
        input.get::<mlua::Value>("colorizer")?,
        tileset
    )?;

    Ok(TilesetConfig {
        info,
        selection,
        pseudotiles,
        recipes,
        colorizer
    })
}
