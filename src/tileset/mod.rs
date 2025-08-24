pub mod tile;

use std::collections::HashMap;
use std::fs;
use crate::{config, tileset};

use tile::Tile;

#[derive(Debug)]
pub struct Tileset {
    tiles: HashMap<String, Tile>
}

impl Tileset {
    // pub fn load_tilesets() -> HashMap<String, Tileset> {
    //     todo!()
    // }

    // TODO this should probably return a Result
    pub fn get_tileset_dirs() -> Vec<String> {
        let conf = config::config_dir();

        let conf_iter = match fs::read_dir(conf) {
            Ok(dir_iter) => dir_iter,
            Err(_) => return Vec::new() // TODO handle properly
        };

        conf_iter // TODO jesus christ...
            .map(|entry| entry.unwrap())
            .filter(|entry| entry.file_type().unwrap().is_dir())
            .map(|entry| entry.path().to_str().unwrap().to_owned())
            .collect()
    }

    // TODO result here too prob
    pub fn get_png_names(tileset: String) -> Vec<String> {
        let ts_iter = match fs::read_dir(tileset) {
            Ok(iter) => iter,
            Err(_) => return Vec::new() // TODO
        };

        ts_iter
            .map(|entry| entry.unwrap())
            .filter(|entry| entry.file_name().into_string().unwrap().ends_with(".png"))
            .map(|entry| entry.path().to_str().unwrap().to_owned())
            .collect()
    }
}
