pub mod tiles;

use std::collections::HashMap;

use crate::error::AppError;

pub use tiles::Tiles;

#[derive(Debug)]
pub struct Recipe {
    pub tiles: Tiles,
}

pub fn parse(input: mlua::Value, tileset: &str) -> Result<HashMap<String, Recipe>, AppError> {
    match input {
        mlua::Value::Table(contents) => {
            let mut recipes = HashMap::new();

            for pair in contents.pairs::<mlua::String, mlua::Value>() {
                let (name, table) = pair?;
                let name = name.to_string_lossy();

                let recipe = parse_recipe(table, tileset, &name)?;
                recipes.insert(name, recipe);
            }

            Ok(recipes)
        }
        _ => Err(AppError::ConfigType(
            format!("{tileset}.recipes"),
            format!("table"),
            input.type_name().to_string()
        ))
    }
}

fn parse_recipe(input: mlua::Value, tileset: &str, recipe: &str) -> Result<Recipe, AppError> {
    match input {
        mlua::Value::Table(contents) => {
            let tiles = tiles::parse(
                contents.get::<mlua::Value>("tiles")?,
                tileset, recipe
            )?;

            Ok(Recipe {
                tiles,
            })
        }
        _ => Err(AppError::ConfigType(
            format!("{tileset}.recipes.{recipe}"),
            format!("table"),
            input.type_name().to_string()
        ))
    }
}
