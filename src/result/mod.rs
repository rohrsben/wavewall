use std::collections::HashMap;
use std::rc::Rc;

use rand::Rng;
use crate::Runtime;
use crate::AppError;
use crate::Image;
use crate::runtime::Tile;
use crate::user_data::{Anchor, PixelInfo};
use tracing::{trace, debug, info, error};

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
    tile_names: HashMap<String, Rc<String>>,
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
        info!("created {} x {} image, tile size = {}", image.height, image.width, tile_size);

        let mut anchors = Vec::new();

        for y in 0..=image.height as i64 / tile_size + 1 {
            for x in 0..=image.width as i64 / tile_size + 1 {
                let new_anchor = Anchor::new(x, y);
                anchors.push(new_anchor);
            }
        }

        info!("created {} anchors", anchors.len());
        debug!(?anchors);

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
            true => {
                let x_offset = rng.random_range(0..tile_size) * -1;
                let y_offset = rng.random_range(0..tile_size) * -1;
                debug!("using offsets ({x_offset}, {y_offset})");

                (x_offset, y_offset)
            }
            false => (0, 0)
        };

        let mut tiles = Vec::new();

        for tile_anchor in anchors {
            let tile = runtime.get_tile(&tile_anchor)?;
            let pixel_anchor = tile_anchor.scale_by(tile_size).with_offset(x_off, y_off);

            let new_association = (tile_anchor, pixel_anchor, tile);

            tiles.push(new_association);
        }

        info!("created {} tile placements", tiles.len());
        debug!("{tiles:#?}");

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

        let in_bounds = image.in_bounds();

        let mut infos = Vec::new();
        let mut tile_names = HashMap::new();

        for group in tiles {
            let (tile_anchor, pixel_anchor, tile) = group;

            let tile_name = match tile_names.get(&tile.name) {
                Some(name) => Rc::clone(name),
                None => {
                    debug!("adding {} to tile_names", tile.name);
                    let name = Rc::new(tile.name.clone());
                    tile_names.insert(tile.name.clone(), name);

                    // TODO this feels... weird?
                    Rc::clone(tile_names.get(&tile.name).unwrap())
                }
            };

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
                        tile_name: Rc::clone(&tile_name),
                        anchor_x: tile_anchor.x,
                        anchor_y: tile_anchor.y
                    };

                    infos.push(info);
                }
            }
        }

        info!("created {} pixel infos", infos.len());
        debug!("{infos:#?}");

        ResultInfos {
            image,
            tile_names,
            infos
        }
    }
}

impl ResultInfos {
    pub fn to_colors(self, runtime: &Runtime) -> Result<ResultImage, AppError> {
        let ResultInfos {
            mut image,
            tile_names: _,
            infos
        } = self;

        let xyti = image.xyti();

        for info in infos {
            trace!("({}, {})", info.x, info.y);
            let index = xyti(info.x as usize, info.y as usize);
            let new_color = match &runtime.colorizer {
                Some(colorizer) => colorizer.apply(info)?,
                None => info.color
            };

            image.pixels[index] = new_color;
        }

        info!("finished colorizing pixels");
        debug!("{image:#?}");

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
