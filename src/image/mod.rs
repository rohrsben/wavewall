pub mod pixel;

use std::{fs::File};
use std::io::BufReader;

use png::{Decoder, DecodingError};

use crate::image::pixel::Pixel;

#[derive(Debug)]
pub enum ImageError {
    FileIO(std::io::Error),
    Decoding(DecodingError)
}

#[derive(Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
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
        let num_pixels = image.width * image.height;

        let mut index: usize = 0;
        while index < num_pixels {
            let r = output[index*4 + 0];
            let g = output[index*4 + 1];
            let b = output[index*4 + 2];
            let a = output[index*4 + 3];

            image.place_pixel_index(Pixel::new_rgba(r, g, b, a), index);

            index += 1
        }

        Ok(image)
    }

    pub fn as_vec(&self) -> Vec<u8> {
        self.pixels.iter()
            .map(|pixel| pixel.as_vec())
            .flatten()
            .collect()
    }

    // (x, y) denotes where the top left of the overlay will be placed on the base image
    pub fn overlay_image(&mut self, overlay: &Self, start_x: usize, start_y: usize) {
        for overlay_y in 0..overlay.height {
            for overlay_x in 0..overlay.width {
                let x = overlay_x + start_x;
                let y = overlay_y + start_y;

                if let Some(new_pixel) = overlay.pixel_at(overlay_x, overlay_y) {
                    self.place_pixel_xy(new_pixel, x, y);
                }
            }
        }
    }

    // if the position isn't in bounds, this fuction is a no-op
    pub fn place_pixel_xy(&mut self, new_pixel: Pixel, x: usize, y: usize) {
        if self.is_in_bounds(x, y) {
            let index = self.xy_to_index(x, y);
            self.pixels[index] = new_pixel
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Option<Pixel> {
        if self.is_in_bounds(x, y) {
            let index = self.xy_to_index(x, y);
            return Some(self.pixels[index]);
        }

        None
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
