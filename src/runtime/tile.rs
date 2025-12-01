use crate::Image;
use hex_color::HexColor;

#[derive(Debug, Clone)]
pub struct Tile {
    pub name: String,
    pub weight: usize,
    pub image: Image,
}

pub struct TileIter {
    index: usize,
    tile: Tile
}

pub struct TileIterInfo {
    pub x: usize,
    pub y: usize,
    pub color: HexColor
}

impl IntoIterator for Tile {
    type Item = TileIterInfo;
    type IntoIter = TileIter;

    fn into_iter(self) -> Self::IntoIter {
        TileIter { index: 0, tile: self }
    }
}

impl Iterator for TileIter {
    type Item = TileIterInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let index_to_xy = |index: usize| (index % self.tile.image.width, index / self.tile.image.height);

        if self.index < self.tile.image.pixels.len() {
            let (x, y) = index_to_xy(self.index);
            let info = TileIterInfo {
                x, y,
                color: self.tile.image.pixels[self.index]
            };

            self.index += 1;

            Some(info)
        } else {
            None
        }
    }
}
