mod tiles;

use crate::{config::Location, error::AppError};
use crate::opt_complex;
use crate::opt_simple;
use mlua::{Function, Table, Value};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Recipe {
    pub tiles: Option<Vec<(String, usize)>>,
    pub placer: Option<Function>
}

pub fn parse(input: Value, loc: &Location) -> Result<HashMap<String, Recipe>, AppError> {
    let loc = loc.add_parent("recipes");

    match input {
        Value::Table(table) => parse_table(table, &loc),

        _ => Err(AppError::IncorrectType {
            location: loc.to_string(),
            expected: format!("table"),
            got: input.type_name().to_string()
        })
    }
}

fn parse_table(table: Table, loc: &Location) -> Result<HashMap<String, Recipe>, AppError> {
    if table.is_empty() {
        return Err(AppError::EmptyTable(loc.to_string()))
    }

    let mut recipes = HashMap::new();

    for pair in table.pairs::<mlua::String, Value>() {
        let (name, recipe) = pair?;
        let name = name.to_string_lossy();

        let loc = loc.add_parent(&name);
        let recipe = parse_recipe(recipe, &loc)?;

        recipes.insert(name, recipe);
    }

    Ok(recipes)
}

fn parse_recipe(input: Value, loc: &Location) -> Result<Recipe, AppError> {
    match input {
        Value::Table(table) => {
            opt_simple!(placer, func,  table, loc);
            opt_complex!(tiles, table, loc);

            Ok(Recipe {
                placer,
                tiles
            })
        }

        _ => Err(AppError::IncorrectType {
            location: loc.to_string(),
            expected: format!("table"),
            got: input.type_name().to_string()
        })
    }
}
