use rand::seq::{IndexedRandom, IteratorRandom};

use crate::error::AppError;
use crate::image::Image;
use crate::tileset::tsconfig::colorizer::Colorizer;
use crate::tileset::Tileset;
use crate::tileset::Tiles;

#[derive(Debug)]
pub struct TilesetRuntime {
    pub lua: mlua::Lua,
    pub tiles: Vec<(Image, i64)>,
    pub tile_size: usize,
    pub colorizer: Colorizer,
}

impl TilesetRuntime {
    pub fn get_tile(&self) -> &Image {
        &self.tiles.choose_weighted(&mut rand::rng(), |item| item.1).unwrap().0
    }

    pub fn from_tileset(source: Tileset) -> Result<Self, AppError> {
        let Tileset { 
            lua,
            name,
            mut config,
            mut tiles
        } = source;

        if tiles.is_empty() {
            return Err(AppError::Runtime(
                format!("In tileset {name}: No tiles found.")
            ))
        }

        if config.recipes.is_empty() {
            return Err(AppError::Runtime(
                format!("In tileset {name}: No recipes found.")
            ))
        }

        if let Some(pseudotiles) = &config.pseudotiles {
            for pseudo in pseudotiles {
                match tiles.get(&pseudo.original) {
                    Some(original) => {
                        let new_tile = original.create_transform(pseudo.transform);

                        tiles.insert(pseudo.name.clone(), new_tile);
                    }
                    None => return Err(AppError::Runtime(
                        format!("In tileset {name}, while generating pseudotiles: no original found with name '{}'", pseudo.original)
                    ))
                }
            }
        }

        let selected_recipe = {
            if let Some(choice) = &config.selection {
                match config.recipes.remove(choice) {
                    Some(recipe) => recipe,
                    None => return Err(AppError::Runtime(
                        format!("In tileset {name}: No recipe found with name '{choice}'")
                    ))
                };
            };

            let random_choice = config.recipes.keys().choose(&mut rand::rng()).unwrap().clone();
            config.recipes.remove(&random_choice).unwrap()
        };

        let tiles = match selected_recipe.tiles {
            Tiles::Nil => {
                let mut all_tiles = Vec::new();

                for (_, tile) in tiles {
                    all_tiles.push((tile, 1));
                }

                all_tiles
            }
            Tiles::List(choices) => {
                let mut validated_tiles = Vec::new();

                for tile_name in choices {
                    match tiles.remove(&tile_name) {
                        Some(tile) => validated_tiles.push((tile, 1)),
                        None => return Err(AppError::Runtime(
                            format!("In tileset {name}: No tile found with name '{tile_name}'")
                        ))
                    };
                }

                validated_tiles
            }
            Tiles::Table(probabilities) => {
                let mut weighted_tiles = Vec::new();

                for (tile_name, weight) in probabilities {
                    match tiles.remove(&tile_name) {
                        Some(tile) => weighted_tiles.push((tile, weight)),
                        None => return Err(AppError::Runtime(
                            format!("In tileset {name}: No tile found with name '{tile_name}'")
                        ))
                    }
                }

                weighted_tiles
            }
        };

        let tile_size = config.info.tile_size;

        let colorizer = config.colorizer;


        Ok(TilesetRuntime {
            lua,
            tiles,
            tile_size,
            colorizer
        })
    }
}
