use rand::Rng;
use crate::Runtime;
use crate::AppError;
use crate::Image;
use crate::runtime::Tile;
use crate::user_data::{Anchor, PixelInfo};

// if, by some small odds, someone other than me is reading this:
// there's not like an actually good reason to use the typestate
// pattern here. it's just to help me separate the phases more clearly
// in my mind, and maybe make it easier to add phases in the future (hopefully)

pub struct ResultAnchors {
    image: Image,
    tile_size: i64,
    anchors: Vec<Anchor>
}

pub struct ResultTiles {
    image: Image,
    tiles: Vec<(Anchor, Anchor, Tile)>
}

pub struct ResultInfos {
    image: Image,
    infos: Vec<PixelInfo>,
}

pub struct ResultImage {
    image: Image
}

impl ResultAnchors {
    pub fn new(runtime: &Runtime) -> Self {
        let image = Image::new(
            runtime.output.width, 
            runtime.output.height
        );
        let tile_size = runtime.tile_size as i64;

        let mut anchors = Vec::new();

        for y in 0..=image.height as i64 / tile_size + 1 {
            for x in 0..=image.width as i64 / tile_size + 1 {
                anchors.push(Anchor::new(x, y));
            }
        }

        Self {
            image,
            tile_size,
            anchors
        }
    }

    pub fn to_tiles(self, runtime: &Runtime) -> Result<ResultTiles, AppError> {
        let ResultAnchors {
            image,
            tile_size,
            anchors
        } = self;

        let mut rng = rand::rng();
        let (x_off, y_off) = match runtime.output.offset {
            true => (
                rng.random_range(0..tile_size) * -1,
                rng.random_range(0..tile_size) * -1
            ),
            false => (0, 0)
        };

        let mut tiles = Vec::new();

        for tile_anchor in anchors {
            let tile = runtime.get_tile(&tile_anchor)?;
            let pixel_anchor = tile_anchor.scale_by(tile_size).with_offset(x_off, y_off);
            tiles.push((tile_anchor, pixel_anchor, tile));
        }

        Ok(ResultTiles {
            image,
            tiles
        })
    }
}

impl ResultTiles {
    pub fn to_infos(self) -> ResultInfos {
        let ResultTiles {
            image,
            tiles
        } = self;

        let mut infos = Vec::new();
        let in_bounds = image.in_bounds();

        for group in tiles {
            let (tile_anchor, pixel_anchor, tile) = group;
            let tile_name = tile.name.clone();

            for pixel in tile {
                let Anchor { 
                    x, y
                } = pixel_anchor.with_offset(pixel.x as i64, pixel.y as i64);

                if in_bounds(x, y) {
                    let info = PixelInfo {
                        x,
                        y,
                        tile_x: pixel.x,
                        tile_y: pixel.y,
                        color: pixel.color,
                        tile_name: tile_name.clone(),
                        anchor_x: tile_anchor.x,
                        anchor_y: tile_anchor.y
                    };

                    infos.push(info);
                }
            }
        }

        ResultInfos {
            image,
            infos
        }
    }
}

impl ResultInfos {
    pub fn to_colors(self, runtime: &Runtime) -> Result<ResultImage, AppError> {
        let ResultInfos {
            mut image,
            infos
        } = self;

        let xyti = image.xyti();

        for info in infos {
            let index = xyti(info.x as usize, info.y as usize);
            let new_color = match &runtime.colorizer {
                Some(colorizer) => colorizer.apply(info)?,
                None => info.color
            };

            image.pixels[index] = new_color;
        }

        Ok(ResultImage { 
            image
        })
    }
}

impl ResultImage {
    pub fn finalize(self) -> Image {
        let Self { image } = self;

        image
    }
}
