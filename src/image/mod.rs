pub mod pixel;

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, BufWriter};

use png::{Decoder, Encoder};

use crate::error::AppError;
use pixel::Pixel;

#[derive(Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Pixel>,
    pub placement_points: VecDeque<(isize, isize)>
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Pixel::new(); width * height];
        let placement_points = VecDeque::new();

        Self {
            width,
            height,
            pixels,
            placement_points
        }
    }

    pub fn from_path(path: &str) -> Result<Self, AppError> {
        let file = File::open(path)?;

        let mut decoder = Decoder::new(BufReader::new(file));
        decoder.set_transformations(png::Transformations::ALPHA);

        let mut reader = decoder.read_info()?;

        // the unwrap here is so unlikely to fail, and so problematic if it does, that its fine
        let mut output = vec![0; reader.output_buffer_size().unwrap()];
        let info = reader.next_frame(&mut output)?;

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

    pub fn save(&self, path: &str) -> Result<(), AppError> {
        let save_file = File::create(path)?;
        let ref mut w = BufWriter::new(save_file);
        let mut encoder = Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        
        let data = self.as_vec();
        writer.write_image_data(&data)?;

        Ok(())
    }

    pub fn generate_placement_points(&mut self, x_offset: usize, y_offset: usize, tile_width: usize, tile_height: usize) {
        let mut x_coords = Vec::new();
        let mut current_x = -1 * x_offset as isize;
        while current_x < self.width.try_into().unwrap() {
            x_coords.push(current_x);
            current_x += tile_width as isize;
        }

        let mut y_coords = Vec::new();
        let mut current_y = -1 * y_offset as isize;
        while current_y < self.height.try_into().unwrap() {
            y_coords.push(current_y);
            current_y += tile_height as isize;
        }

        for y in y_coords {
            for x in &x_coords {
                self.placement_points.push_back((*x, y));
            }
        }
    }

    pub fn as_vec(&self) -> Vec<u8> {
        self.pixels.iter()
            .map(|pixel| pixel.as_vec())
            .flatten()
            .collect()
    }

    // (x, y) denotes where the top left of the overlay will be placed on the base image
    // TODO this actually needs to take isize's, and all knock-on effects
    pub fn overlay_image(&mut self, overlay: &Self, start_x: isize, start_y: isize) {
        for overlay_y in 0..overlay.height {
            for overlay_x in 0..overlay.width {
                let x = overlay_x as isize + start_x;
                let y = overlay_y as isize + start_y;

                if x >= 0 && y >= 0 {
                    if let Some(new_pixel) = overlay.pixel_at(overlay_x, overlay_y) {
                        self.place_pixel_xy(new_pixel, x as usize, y as usize);
                    }
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

    pub fn next_free_xy(&self) -> Option<(usize, usize)> {
        let reference = Pixel::new();

        for (index, pixel) in self.pixels.iter().enumerate() {
            if reference == *pixel {
                return Some(self.index_to_xy(index));
            }
        }

        None
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
