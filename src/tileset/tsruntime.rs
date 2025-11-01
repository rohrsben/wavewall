use rand::seq::IndexedRandom;

use crate::tileset::tile::Tile;
use crate::tileset::tsconfig::colorizer::Colorizer;

#[derive(Debug)]
pub struct TilesetRuntime {
    pub lua: mlua::Lua,
    pub tiles: Vec<(Tile, i64)>,
    pub width: usize,
    pub height: usize,
    pub colorizer: Colorizer,
}

impl TilesetRuntime {
    pub fn get_tile(&self) -> &Tile {
        &self.tiles.choose_weighted(&mut rand::rng(), |item| item.1).unwrap().0
    }
}
