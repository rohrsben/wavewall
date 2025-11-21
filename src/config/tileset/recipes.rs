use crate::error::AppError;
use mlua::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Recipe {
    pub tiles: Option<Vec<(String, usize)>>,
}

impl Recipe {
    pub fn parse_all(input: Value) -> Result<HashMap<String, Self>, AppError> {
        match input {
            Value::Table(table) => {
                if table.is_empty() {
                    return Err(AppError::ConfigEmptyTable(
                        format!("tileset.recipes")
                    ))
                }

                let mut recipes = HashMap::new();

                for pair in table.pairs::<mlua::String, Value>() {
                    let (name, recipe) = pair?;

                    let name = name.to_string_lossy();
                    let recipe = Self::parse(recipe, &name)?;

                    recipes.insert(name, recipe);
                }

                Ok(recipes)
            }
            _ => Err(AppError::ConfigType(
                format!("tileset.recipes"),
                format!("table"),
                input.type_name().to_string()
            ))
        }
    }

    fn parse(input: Value, recipe_name: &str) -> Result<Self, AppError> {
        match input {
            Value::Table(table) => {
                let tiles = Self::parse_tiles(
                    table.get::<Value>("tiles")?, 
                    recipe_name
                )?;

                Ok(Self {
                    tiles
                })
            }
            _ => Err(AppError::ConfigType(
                format!("tileset.recipes.{recipe_name}"),
                format!{"table"},
                input.type_name().to_string()
            ))
        }
    }

    fn parse_tiles(input: Value, recipe_name: &str) -> Result<Option<Vec<(String, usize)>>, AppError> {
        match input {
            Value::Nil => Ok(None),
            Value::Table(table) => {
                if table.is_empty() {
                    return Err(AppError::ConfigEmptyTable(
                        format!("tileset.recipes.{recipe_name}.tiles")
                    ))
                }

                let mut options = Vec::new();

                if table.sequence_values::<Value>().count() > 0 {
                    // list of string path
                    for item in table.sequence_values::<Value>() {
                        let item = item?;
                        match item {
                            Value::String(str) => options.push((str.to_string_lossy(), 1)),
                            _ => return Err(AppError::ConfigTypeListItem(
                                format!("tileset.recipes.{recipe_name}.tiles"), 
                                format!("string"), 
                                item.type_name().to_string()
                            ))
                        }
                    }
                } else {
                    // table path
                    for pair in table.pairs::<mlua::String, Value>() {
                        let (item, weight) = pair?;

                        let item = item.to_string_lossy();

                        let weight = if let Value::Integer(int) = weight && int > 0 {
                            int as usize
                        } else {
                            return Err(AppError::ConfigTypeTableItem(
                                format!("tileset.recipes.{recipe_name}.tiles.{item}"),
                                format!("positive number"),
                                weight.type_name().to_string()
                            ))
                        };

                        options.push((item, weight));
                    }
                }

                Ok(Some(options))
            }
            _ => Err(AppError::ConfigType(
                format!("tileset.recipes.{recipe_name}.tiles"),
                format!("nil, list of string, table"),
                input.type_name().to_string()
            ))
        }
    }
}
