use rand::seq::IndexedRandom;

use crate::image::Image;
use crate::tileset::tsconfig::colorizer::Colorizer;

#[derive(Debug)]
pub struct TilesetRuntime {
    pub lua: mlua::Lua,
    pub tiles: Vec<(Image, i64)>,
    pub width: usize,
    pub height: usize,
    pub colorizer: Colorizer,
}

impl TilesetRuntime {
    pub fn get_tile(&self) -> &Image {
        &self.tiles.choose_weighted(&mut rand::rng(), |item| item.1).unwrap().0
    }
}
