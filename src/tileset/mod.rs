pub mod tsconfig;
pub mod tile;

use std::collections::HashMap;
use std::fs::{self, DirEntry};

use mlua::Lua;
use infer;
use rand::seq::{IteratorRandom, IndexedRandom};

use crate::error::AppError;

use tsconfig::TilesetConfig;
use tsconfig::recipes::tiles::Tiles;
use tile::Tile;

#[derive(Debug)]
pub struct Tileset {
    lua: Lua,
    pub config: TilesetConfig,
    pub names: Vec<String>,
    pub tiles: HashMap<String, Tile>
}

impl Tileset {
    pub fn get_tile(&self) -> Result<&Tile, AppError> {
        let recipe = self.config.selected_recipe()?;
        let choice = match &recipe.tiles {
            Tiles::List(options) => {
                match options.choose(&mut rand::rng()) {
                    Some(choice) => choice,
                    None => {
                        let msg = format!("No tiles found. Did you set 'tiles' in the recipe but forget to populate it?");
                        return Err(AppError::Runtime(msg));
                    }
                }
            }
            Tiles::Nil => {
                let names = self.tiles.keys();
                match names.choose(&mut rand::rng()) {
                    Some(choice) => choice,
                    None => {
                        let msg = format!("No tiles found.");
                        return Err(AppError::Runtime(msg));
                    }
                }
            }
        };

        match self.tiles.get(choice) {
            Some(result) => Ok(result),
            None => {
                let msg = format!("Found no tile with the name: {}", choice);
                return Err(AppError::Runtime(msg));
            }
        }
    }
}

pub fn parse(tileset: &str) -> Result<Tileset, AppError> {
    let lua = Lua::new();

    let config_file = format!("{tileset}/tileset.lua");
    let config = match fs::read_to_string(config_file) {
        Ok(contents) => lua.load(contents).set_name("@tileset.lua"),
        Err(e) => return Err(AppError::IO(e))
    };

    let config = match config.eval::<mlua::Table>() {
        Ok(result) => tsconfig::parse(result)?,
        Err(e) => return Err(AppError::ConfigLua(e))
    };

    let (names, tiles) = parse_images(tileset)?;

    Ok(Tileset {
        lua,
        config,
        names,
        tiles
    })
}

fn parse_images(tileset: &str) -> Result<(Vec<String>, HashMap<String, Tile>), AppError> {
    let mut names = Vec::new();
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

            let path = file.path().to_string_lossy().to_string();
            let tile = Tile::from_path(&path)?;

            tiles.insert(name.clone(), tile);
            names.push(name);
        }
    }

    Ok((names, tiles))
}
