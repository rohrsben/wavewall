pub mod pixel;

use std::{fs::File, path::Path};
use std::io::BufReader;

use png::{Decoder, DecodingError};

use crate::config::Config;
use crate::image::pixel::Pixel;

#[derive(Debug)]
pub enum ImageError {
    FileIO(std::io::Error),
    Decoding(DecodingError)
}

#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
    pub pixels: Vec<Pixel>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let data: Vec<Pixel> = vec![Pixel::new(); width * height];

        Self {
            width,
            height,
            pixels: data
        }
    }

    pub fn from_path(path: String) -> Result<Self, ImageError> {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(ImageError::FileIO(e))
        };

        let mut decoder = Decoder::new(BufReader::new(file));
        decoder.set_transformations(png::Transformations::ALPHA);

        let mut reader = match decoder.read_info() {
            Ok(i) => i,
            Err(e) => return Err(ImageError::Decoding(e))
        };

        let mut output = vec![0; reader.output_buffer_size().unwrap()];
        let info = reader.next_frame(&mut output).unwrap();

        let mut image = Image::new(info.width as usize, info.height as usize);

        let mut index: usize = 0;
        while index < image.width * image.height {
            let r = output[index*4 + 0];
            let g = output[index*4 + 1];
            let b = output[index*4 + 2];
            let a = output[index*4 + 3];

            image.place_pixel_index(Pixel::new_rgba(r, g, b, a), index);

            index += 1
        }

        Ok(image)
    }

    // if the position isn't in bounds, this fuction is a no-op
    pub fn place_pixel_xy(&mut self, new_pixel: Pixel, x: usize, y: usize) {
        if self.is_in_bounds(x, y) {
            let index = self.xy_to_index(x, y);
            self.pixels[index] = new_pixel
        }
    }

    pub fn place_pixel_index(&mut self, new_pixel: Pixel, index: usize) {
        self.pixels[index] = new_pixel
    }

    fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn xy_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn index_to_xy(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }
}
