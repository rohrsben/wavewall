use std::collections::HashMap;
use std::{env, fs};

use crate::image::Image;
use crate::config::Config;

#[derive(Debug)]
pub struct Tileset {
    pub names: Vec<String>,
    pub tiles: HashMap<String, Image>
}

impl Tileset {
    pub fn load_tilesets() -> HashMap<String, Tileset> {
        let tilesets = Tileset::get_tileset_dirs();

        let mut sets = HashMap::new();

        for ts in tilesets {
            let value = Tileset::load_tileset(ts.1);

            sets.insert(ts.0, value);
        }

        sets
    }

    pub fn load_tileset(path: String) -> Self {
        let pngs = Tileset::get_png_names(path);

        let mut tiles = HashMap::new();

        // for png in pngs {
        //     let value = Tile::from_path(png.1);
        //
        //     // TODO clean this up
        //     tiles.insert(png.0.strip_suffix(".png").unwrap().to_owned(), value);
        // }

        // let names = tiles.keys().map(|key| key.to_owned()).collect();

        let names = Vec::new();

        Tileset {
            names,
            tiles
        }
    }

    // TODO this should probably return a Result
    pub fn get_tileset_dirs() -> Vec<(String, String)> {
        let conf = Config::config_dir(); // temp hack

        let conf_iter = match fs::read_dir(conf) {
            Ok(dir_iter) => dir_iter,
            Err(_) => return Vec::new() // TODO handle properly
        };

        conf_iter // TODO jesus christ...
            .map(|entry| entry.unwrap())
            .filter(|entry| entry.file_type().unwrap().is_dir())
            .map(|entry| (entry.file_name().into_string().unwrap(), entry.path().to_str().unwrap().to_owned()))
            .collect()
    }

    // TODO result here too prob
    pub fn get_png_names(tileset: String) -> Vec<(String, String)> {
        let ts_iter = match fs::read_dir(tileset) {
            Ok(iter) => iter,
            Err(_) => return Vec::new() // TODO
        };

        ts_iter
            .map(|entry| entry.unwrap())
            .filter(|entry| entry.file_name().into_string().unwrap().ends_with(".png"))
            .map(|entry| (entry.file_name().into_string().unwrap(), entry.path().to_str().unwrap().to_owned()))
            .collect()
    }
}
