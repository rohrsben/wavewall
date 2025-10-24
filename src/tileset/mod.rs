pub mod tsconfig;
pub mod tile;
pub mod tsruntime;

use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::path::Path;

use infer;
use rand::seq::IteratorRandom;

use crate::error::AppError;

use tsconfig::TilesetConfig;
use tsconfig::recipes::tiles::Tiles;
use tsconfig::colorizer::Colorizer;
use tile::Tile;
use tsruntime::TilesetRuntime;

#[derive(Debug)]
pub struct Tileset {
    lua: mlua::Lua,
    pub config: TilesetConfig,
    pub tiles: HashMap<String, Tile>
}

impl Tileset {
    // TODO this is a huge function. perhaps this can be broken up?
    pub fn into_runtime(self) -> Result<TilesetRuntime, AppError> {
        let Tileset { lua, mut config, mut tiles } = self;

        let selected_recipe = {
            if config.recipes.is_empty() {
                let msg = format!("No recipes found.");
                return Err(AppError::Runtime(msg))
            };

            if let Some(choice) = &config.selection {
                match config.recipes.remove(choice) {
                    Some(recipe) => recipe,
                    None => {
                        let msg = format!("No recipe found with name: {}", choice);
                        return Err(AppError::Runtime(msg));
                    }
                };
            };

            let random_choice = config.recipes.keys().choose(&mut rand::rng()).unwrap().clone();
            config.recipes.remove(&random_choice).unwrap()
        };

        let selected_tiles = match selected_recipe.tiles {
            Tiles::Nil => tiles, // from the pattern-match on self
            Tiles::List(choices) => {
                let mut validated_tiles = HashMap::new();

                for tile_name in choices {
                    match tiles.remove(&tile_name) {
                        Some(tile) => validated_tiles.insert(tile_name, tile),
                        None => {
                            let msg = format!("No tile found with name: {}", tile_name);
                            return Err(AppError::Runtime(msg));
                        }
                    };
                }

                validated_tiles
            }
        };

        let tile_width = config.info.size.width;

        let tile_height = config.info.size.height;

        let colorizer = match config.colorizer {
            Colorizer::Nil => None,
            Colorizer::Function(func) => Some(func)
        };

        Ok(TilesetRuntime {
            lua,
            tiles: selected_tiles,
            tile_width,
            tile_height,
            colorizer
        })
    }
}

pub fn parse_all() -> Result<HashMap<String, Tileset>, AppError> {
    let is_tileset = |entry: &DirEntry| {
        if entry.metadata()?.is_dir() {
            let ts_config_path = format!("{}/tileset.lua", entry.path().to_string_lossy());
            
            if Path::new(&ts_config_path).exists() {
                return Ok(true);
            }
        }

        Ok::<bool, AppError>(false)
    };

    let mut tilesets = HashMap::new();

    for entry in fs::read_dir(".")? {
        let entry = entry?;
        if is_tileset(&entry).unwrap() {
            let name = entry.file_name().to_string_lossy().to_string();
            let tileset = parse(&name)?;

            tilesets.insert(name, tileset);
        }
    }

    Ok(tilesets)
}

pub fn parse(tileset: &str) -> Result<Tileset, AppError> {
    let lua = mlua::Lua::new();

    let path = format!("{tileset}/tileset.lua");
    let config = fs::read_to_string(&path)?;
    let config = lua.load(config)
        .set_name(format!("@{path}"))
        .eval::<mlua::Table>()?;
    let config = tsconfig::parse(config, tileset)?;

    let tiles = parse_images(tileset)?;

    Ok(Tileset {
        lua,
        config,
        tiles
    })
}

fn parse_images(tileset: &str) -> Result<HashMap<String, Tile>, AppError> {
    let mut tiles = HashMap::new();

    let is_png = |file: &DirEntry| {
        match infer::get_from_path(file.path()) {
            Ok(Some(info)) => info.mime_type() == "image/png",
            _ => false
        }
    };

    let normalize_name = |file: &DirEntry| {
        let name_string = file.file_name().to_string_lossy().to_string();

        if let Some(name) = name_string.strip_suffix(".png") {
            return name.to_string()
        }

        name_string
    };

    for file in fs::read_dir(tileset)? {
        let file = file?;
        if is_png(&file) {
            let name = normalize_name(&file);

            // TODO low priority: this could probably be better
            let path = file.path().to_string_lossy().to_string();
            let tile = Tile::from_path(&path)?;

            tiles.insert(name, tile);
        }
    }

    Ok(tiles)
}
