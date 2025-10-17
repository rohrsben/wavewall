pub mod recipes;

use std::collections::HashMap;

use rand::seq::IteratorRandom;

use crate::{error::AppError, parse};

#[derive(Debug)]
pub struct TilesetConfig {
    selection: Option<String>,
    recipes: HashMap<String, recipes::Recipe>
}

impl TilesetConfig {
    pub fn selected_recipe(&self) -> Result<&recipes::Recipe, AppError> {
        if let Some(choice) = &self.selection {
            match self.recipes.get(choice) {
                Some(recipe) => return Ok(recipe),
                None => {
                    // TODO validate when parsing the tsconfig?
                    let msg = format!("No recipe found with name: {}", choice);
                    return Err(AppError::Runtime(msg));
                }
            }
        }

        let recipes = self.recipes.values();
        match recipes.choose(&mut rand::rng()) {
            Some(choice) => Ok(choice),
            None => {
                // TODO check for no recipes when parsing the tsconfig?
                let msg = format!("No recipes found.");
                return Err(AppError::Runtime(msg))
            }
        }
    }
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
