pub mod tiles;

use std::collections::HashMap;

use crate::error::AppError;

#[derive(Debug)]
pub struct Recipe {
    pub tiles: tiles::Tiles,
}

pub fn parse(input: mlua::Value) -> Result<HashMap<String, Recipe>, AppError> {
    match input {
        mlua::Value::Table(contents) => {
            let mut recipes = HashMap::new();

            for pair in contents.pairs::<mlua::String, mlua::Value>() {
                let (name, table) = pair?;
                let name = name.to_string_lossy();

                let recipe = parse_recipe(table, &name)?;
                recipes.insert(name, recipe);
            }

            Ok(recipes)
        }
        _ => Err(AppError::ConfigType("tileset.recipes", "table", input.type_name().to_string()))
    }
}

fn parse_recipe(input: mlua::Value, recipe_name: &str) -> Result<Recipe, AppError> {
    match input {
        mlua::Value::Table(contents) => {
            let tiles = match contents.get::<mlua::Value>("tiles") {
                Ok(result) => tiles::parse(result, recipe_name)?,
                Err(e) => return Err(AppError::ConfigLua(e))
            };

            Ok(Recipe { tiles })
        }
        _ => {
            let location = format!("tileset.recipes.{}", recipe_name);
            Err(AppError::ConfigTypeSpecific(location, "table", input.type_name().to_string()))
        }
    }
}
