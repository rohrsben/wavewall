use std::collections::HashMap;

use rand::seq::IteratorRandom;

use crate::tileset::tile::Tile;

#[derive(Debug)]
pub struct TilesetRuntime {
    pub lua: mlua::Lua,
    pub tiles: HashMap<String, Tile>
}

impl TilesetRuntime {
    pub fn get_tile(&self) -> &Tile {
        self.tiles.values().choose(&mut rand::rng()).unwrap()
    }
}
