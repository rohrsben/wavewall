mod tile;

pub use tile::Tile;
pub use tile::TileIterInfo;

use crate::config::{self, tileset::Pseudotile, Output, Colorizer, Config};
use crate::error::AppError;
use crate::image::Image;
use crate::user_data::Anchor;
use infer;
use mlua::{Lua, Function};
use rand::prelude::*;
use std::collections::HashMap;
use std::fs::DirEntry;
use std::fs;

#[derive(Debug)]
pub struct Runtime {
    lua: Lua,
    pub tiles: Vec<Tile>,
    pub placer: Option<Function>,
    pub tile_size: usize,
    pub colorizer: Option<Colorizer>,
    pub output: Output,
    pub recipe_name: String,
}

impl Runtime {
    pub fn get_tile(&self, anchor: &Anchor) -> Result<Tile, AppError> {
        match &self.placer {
            Some(placer) => {
                let output = placer.call::<mlua::String>(anchor.clone())?
                    .to_string_lossy();

                self.get_tile_specific(&output)
            }
            None => {
                self.get_tile_random()
            }
        }
    }

    fn get_tile_random(&self) -> Result<Tile, AppError> {
        let tile = self.tiles.choose_weighted(&mut rand::rng(), |it| it.weight)?.clone();

        Ok(tile)
    }

    fn get_tile_specific(&self, tile_name: &str) -> Result<Tile, AppError> {
        for tile in &self.tiles {
            if tile.name == tile_name {
                return Ok(tile.clone())
            }
        }

        Err(AppError::Runtime(
            format!("No tile found with name {tile_name}")
        ))
    }

    pub fn from_config(source: Config) -> Result<Self, AppError> {
        let Config {
            lua,
            output,
            colorizer,
            tileset,
        } = source;

        let recipe_name = match tileset.recipe {
            Some(recipe) => recipe,
            None => {
                tileset.recipes.keys().choose(&mut rand::rng()).unwrap().clone()
            }
        };

        let recipe = match tileset.recipes.get(&recipe_name) {
            Some(recipe) => recipe,
            None => return Err(AppError::Runtime(
                format!("No recipe found with name: {recipe_name}")
            ))
        };


        let mut all_tiles = get_tiles(&tileset.info.name)?;

        if let Some(wanted_pseudos) = tileset.pseudotiles {
            for wanted_pseudo in wanted_pseudos {
                let Pseudotile { name, original, transform } = wanted_pseudo;

                let new_tile = match all_tiles.get(&original) {
                    Some(original_tile) => original_tile.create_transform(transform),
                    None => return Err(AppError::Runtime(
                        format!("No original tile by name {original} found when creating pseudotile {name}")
                    ))
                };

                all_tiles.insert(name, new_tile);
            }
        }

        let mut runtime_tiles = Vec::new();

        match &recipe.tiles {
            Some(wanted_tiles) => {
                for wanted_tile in wanted_tiles {
                    let (name, weight) = wanted_tile;
                    match all_tiles.get(name) {
                        Some(image) => {
                            let new_tile = Tile {
                                name: name.clone(),
                                weight: *weight,
                                image: image.clone()
                            };

                            runtime_tiles.push(new_tile);
                        }
                        None => return Err(AppError::Runtime(
                            format!("No tile found with name: {name}")))
                    };
                }
            }
            None => {
                for tile_info in all_tiles {
                    let (name, image) = tile_info;
                    let new_tile = Tile {
                        name,
                        weight: 1,
                        image
                    };

                    runtime_tiles.push(new_tile);
                }
            }
        }

        let tile_size = match tileset.info.tile_size {
            Some(size) => size,
            None => {
                runtime_tiles[0].image.width
            }
        };

        let placer = recipe.placer.clone();

        Ok(Self {
            lua,
            tiles: runtime_tiles,
            placer,
            tile_size,
            colorizer,
            output,
            recipe_name
        })
    }

    pub fn save_path(&self, args_path: &Option<String>) -> String {
        if let Some(path) = args_path {
            return path.to_owned()
        }

        let dir = match &self.output.directory {
            Some(dir) => dir.to_owned(),
            None => config::config_dir()
        };

        let filename = match &self.output.filename {
            Some(name) => name.to_owned(),
            None => format!("result.png")
        };

        format!("{dir}/{filename}")
    }
}

pub fn get_tiles(tileset_name: &str) -> Result<HashMap<String, Image>, AppError> {
    let is_png = |entry: &DirEntry| {
        match infer::get_from_path(entry.path()) {
            Ok(Some(info)) => info.mime_type() == "image/png",
            _ => false
        }
    };

    let normalize_name = |file: &DirEntry| {
        let name_string = file.file_name().to_string_lossy().to_string();

        if let Some(name) = name_string.strip_suffix(".png") {
            return name.to_owned()
        }

        name_string
    };

    let tileset_path = format!("{}/{}", config::config_dir(), tileset_name);
    let mut tiles = HashMap::new();
    for entry in fs::read_dir(tileset_path)? {
        let entry = entry?;
        if is_png(&entry) {
            let name = normalize_name(&entry);
            let tile = Image::from_path(entry.path())?;
            tiles.insert(name, tile);
        }
    }

    Ok(tiles)
} 
