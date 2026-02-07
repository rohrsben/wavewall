pub mod info;
pub mod pseudotiles;
pub mod recipes;

pub use info::Info;
pub use pseudotiles::Pseudotile;
pub use recipes::Recipe;

use crate::AppError;
use crate::opt_complex;
use crate::opt_simple;
use crate::config::Location;
use mlua::{Table, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub struct TilesetConfig {
    pub info: Info,
    pub pseudotiles: Option<Vec<Pseudotile>>,
    pub recipe: Option<String>,
    pub recipes: HashMap<String, Recipe>,
}

pub fn parse(input: Value, loc: &Location) -> Result<TilesetConfig, AppError> {
    let loc = loc.add_parent("tileset");

    match input {
        Value::Table(table) => parse_table(table, &loc),

        _ => Err(AppError::IncorrectType {
            location: loc.to_string(),
            expected: format!("table"),
            got: input.type_name().to_string()
        })
    }
}

fn parse_table(table: Table, loc: &Location) -> Result<TilesetConfig, AppError> {
    opt_complex!(info,        table, loc);
    opt_complex!(recipes,     table, loc);
    opt_complex!(pseudotiles, table, loc);
    // TODO document loss of slost
    opt_simple!(recipe, string, table, loc);

    Ok(TilesetConfig {
        info,
        recipes,
        recipe,
        pseudotiles
    })
}
