use crate::Image;
use hex_color::HexColor;

#[derive(Clone)]
pub struct Tile {
    pub name: String,
    pub weight: usize,
    pub scale: usize,
    pub image: Image,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tile {{ name = {}, weight = {}, image = ({} x {})}}", self.name, self.weight, self.image.width, self.image.height)
    }
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
        let itxy = self.tile.image.itxy();

        if self.index < self.tile.image.pixels.len() {
            let (x, y) = itxy(self.index);
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
