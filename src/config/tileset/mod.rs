pub mod info;
pub mod pseudotiles;
pub mod recipes;

pub use info::Info;
pub use pseudotiles::Pseudotile;
pub use recipes::Recipe;

use crate::AppError;
use crate::config::parse;
use mlua::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TilesetConfig {
    pub info: Info,
    pub pseudotiles: Option<Vec<Pseudotile>>,
    pub recipe: Option<String>,
    pub recipes: HashMap<String, Recipe>,
}

impl TilesetConfig {
    pub fn parse(input: Value) -> Result<Self, AppError> {
        match input {
            Value::Table(table) => {
                let info = Info::parse(
                    table.get::<Value>("info")?
                )?;

                let pseudotiles = Pseudotile::parse_all(
                    table.get::<Value>("pseudotiles")?
                )?;

                let recipe = parse::slost(
                    table.get::<Value>("recipe")?, 
                    format!("tileset.recipe")
                )?;

                let recipes = Recipe::parse_all(
                    table.get::<Value>("recipes")?
                )?;

                Ok(Self {
                    info,
                    pseudotiles,
                    recipe,
                    recipes,
                })
            }
            _ => Err(AppError::ConfigType(
                format!("tileset"),
                format!("table"),
                input.type_name().to_string()
            ))
        }
    }
}
