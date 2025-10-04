use std::{fs::File, path::Path};

use crate::config::Config;
use crate::tileset::tile::Tile;

pub struct Image {
    pub file: File,
    pub data: Vec<Vec<u8>>,
}

impl Image {
    pub fn new(tile_width: usize, rows: usize, columns: usize) -> Self {
        let file = Self::create_result_file();
        let data: Vec<Vec<u8>> = vec![vec![0; columns * tile_width]; rows * tile_width];

        Self {
            file,
            data
        }
    }

    pub fn place_tile(&mut self, tile: &Tile, row: usize, col: usize) {
        println!("OPERATING: ROW {} COL {}", row, col);
        for i in 0..tile.length {
            let tile_row = i * tile.length;
            let tile_slice = &tile.pixels[tile_row..tile_row+tile.length];

            let mut pixel_col = col * tile.length;
            let pixel_row = row * tile.length + i;

            for val in tile_slice {
                println!("pixel_col: {:?}", pixel_col);
                println!("pixel_row: {:?}", pixel_row);
                self.data[pixel_row][pixel_col] = *val;
                pixel_col += 1;
            }
        }
        println!("\n\n\n");
    }

    pub fn get_flattened(&self) -> Vec<u8> {
        let result = self.data.clone();

        result.iter().flatten().map(|x| *x).collect()
    }

    pub fn create_result_file() -> File {
        let path = format!("{}/result.png", Config::get_config_dir());
        
        File::create(Path::new(&path)).unwrap()
    }
}
