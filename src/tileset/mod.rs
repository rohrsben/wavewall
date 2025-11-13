pub mod tsconfig;
pub mod tsruntime;

use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::path::Path;

use infer;
use hex_color::HexColor;
use rand::seq::IteratorRandom;

use crate::error::AppError;
use crate::user_data::ColorInfo;
use crate::image::Image;

use tsconfig::recipes::Tiles;
use tsruntime::TilesetRuntime;

pub use tsconfig::TilesetConfig;

#[derive(Debug)]
pub struct Tileset {
    lua: mlua::Lua,
    pub name: String,
    pub config: TilesetConfig,
    pub tiles: HashMap<String, Image>
}

impl Tileset {
    // TODO this is a huge function. perhaps this can be broken up? or moved to tsruntime?
    pub fn into_runtime(self) -> Result<TilesetRuntime, AppError> {
        let Tileset { 
            lua,
            name,
            mut config,
            mut tiles
        } = self;

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

        let width = config.info.size.width;

        let height = config.info.size.height;

        let colorizer = config.colorizer;


        Ok(TilesetRuntime {
            lua,
            tiles,
            width,
            height,
            colorizer
        })
    }

    pub fn new_lua() -> Result<mlua::Lua, AppError> {
        let lua = mlua::Lua::new();

        let convert_rgb = lua.create_function(|_, (r, g, b)| {
            let color = HexColor::rgb(r, g, b);

            Ok(ColorInfo::new(color))
        })?;
        lua.globals().set("convert_rgb", convert_rgb)?;

        let convert_rgba = lua.create_function(|_, (r, g, b, a)| {
            let color = HexColor::rgba(r, g, b, a);

            Ok(ColorInfo::new(color))
        })?;
        lua.globals().set("convert_rgba", convert_rgba)?;

        let convert_hex = lua.create_function(|_, hex: String| {
            let color = match HexColor::parse(&hex) {
                Ok(color) => color,
                Err(_) => return Err(mlua::Error::RuntimeError(format!("While calling hex_to_rgba: failed to parse '{hex}'")))
            };

            Ok(ColorInfo::new(color))
        })?;
        lua.globals().set("convert_hex", convert_hex)?;

        lua.load(r#"
            function create_all_pseudos(original)
                return {
                    [original .. '_90'] = '90',
                    [original .. '_180'] = '180',
                    [original .. '_270'] = '270',
                    [original .. '_horizontal'] = 'horizontal',
                    [original .. '_vertical'] = 'vertical',
                    [original .. '_diagonal'] = 'diagonal',
                    [original .. '_antidiagonal'] = 'antidiagonal',
                }
            end
        "#).exec()?;

        Ok(lua)
    }
}

pub fn parse_all() -> Result<Vec<Tileset>, AppError> {
    let is_tileset = |entry: &DirEntry| {
        if entry.metadata()?.is_dir() {
            let ts_config_path = format!("{}/tileset.lua", entry.path().to_string_lossy());
            
            if Path::new(&ts_config_path).exists() {
                return Ok(true);
            }
        }

        Ok::<bool, AppError>(false)
    };

    let mut tilesets = Vec::new();

    for entry in fs::read_dir(".")? {
        let entry = entry?;
        if is_tileset(&entry).unwrap() {
            let name = entry.file_name().to_string_lossy().to_string();
            let tileset = parse(name)?;

            tilesets.push(tileset)
        }
    }

    Ok(tilesets)
}

pub fn parse(tileset: String) -> Result<Tileset, AppError> {
    let lua = Tileset::new_lua()?;

    let path = format!("{tileset}/tileset.lua");
    let config = fs::read_to_string(&path)?;
    let config = lua.load(config)
        .set_name(format!("@{path}"))
        .eval::<mlua::Table>()?;
    let config = tsconfig::parse(config, &tileset)?;

    let tiles = parse_images(&tileset)?;

    Ok(Tileset {
        lua,
        name: tileset,
        config,
        tiles
    })
}

fn parse_images(tileset: &str) -> Result<HashMap<String, Image>, AppError> {
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
            let tile = Image::from_path(&path)?;

            tiles.insert(name, tile);
        }
    }

    Ok(tiles)
}
