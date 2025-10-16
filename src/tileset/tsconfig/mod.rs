pub mod recipe;

use crate::error::AppError;

use recipe::Recipe;

#[derive(Debug)]
pub struct TilesetConfig {
    selection: Option<String>,
    recipes: Vec<Recipe>
}

pub fn parse(_input: mlua::Table) -> Result<TilesetConfig, AppError> {
    todo!()
    // figure out how to determine the number/names of the recipes

    // parse them and add to a vec
}
