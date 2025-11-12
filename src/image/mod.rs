pub mod transform;
pub use transform::Transform;

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, BufWriter};

use png::{Decoder, Encoder};
use hex_color::HexColor;

use crate::error::AppError;
use crate::user_data::PixelInfo;
use crate::tileset::tsconfig::Colorizer;

#[derive(Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<HexColor>,
    pub placement_points: VecDeque<(isize, isize)>
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![HexColor::BLACK; width * height];
        let placement_points = VecDeque::new();

        Self {
            width,
            height,
            pixels,
            placement_points
        }
    }

    pub fn create_template(length: usize) -> Self {
        let mut template = Image::new(length, length);

        let mut midpoints = Vec::new();
        if length % 2 == 0 {
            midpoints.push(length / 2 - 1);
        }
        midpoints.push(length / 2);

        for mp in midpoints {
            template.place_pixel_xy(HexColor::RED, 0,          mp);
            template.place_pixel_xy(HexColor::RED, mp,         0);
            template.place_pixel_xy(HexColor::RED, mp,         length - 1);
            template.place_pixel_xy(HexColor::RED, length - 1, mp);
        }

        template
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

            image.place_pixel_index(HexColor::rgba(r, g, b, a), index);

            index += 1
        }

        Ok(image)
    }

    pub fn create_transform(&self, transform: Transform) -> Self {
        let mut new_image = Image::new(self.width, self.height);

        // more performant than matching inside the closure... but at what cost?
        let converter: Box<dyn Fn(usize) -> usize > = match transform {
            Transform::TurnOnce => Box::new(|index| {
                let (old_x, old_y) = self.index_to_xy(index);
                self.xy_to_index(
                    old_y,
                    self.width - old_x - 1
                )
            }),
            Transform::TurnTwice => Box::new(|index| {
                let (old_x, old_y) = self.index_to_xy(index);
                self.xy_to_index(
                    self.width - old_x - 1,
                    self.width - old_y - 1
                )
            }),
            Transform::TurnThrice => Box::new(|index| {
                let (old_x, old_y) = self.index_to_xy(index);
                self.xy_to_index(
                    self.width - old_y - 1,
                    old_x
                )
            }),
            Transform::Horizontal => Box::new(|index| {
                let (old_x, old_y) = self.index_to_xy(index);
                self.xy_to_index(
                    old_x,
                    self.width - old_y - 1
                )
            }),
            Transform::Vertical => Box::new(|index| {
                let (old_x, old_y) = self.index_to_xy(index);
                self.xy_to_index(
                    self.width - old_x - 1,
                    old_y
                )
            }),
            Transform::Diagonal => Box::new(|index| {
                let (old_x, old_y) = self.index_to_xy(index);
                self.xy_to_index(
                    old_y,
                    old_x
                )
            }),
            Transform::Antidiagonal => Box::new(|index| {
                let (old_x, old_y) = self.index_to_xy(index);
                self.xy_to_index(
                    self.width - old_y - 1,
                    self.width - old_x - 1
                )
            })
        };

        for (index, pixel) in self.pixels.iter().enumerate() {
            new_image.place_pixel_index(*pixel, converter(index));
        }

        new_image
    }

    // TODO this should be faster
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

    pub fn recolor(&mut self, colorizer: &Colorizer) -> Result<(), AppError> {
        let to_pixel_info = |index: usize, color: HexColor| {
            let (x, y) = (index % self.width, index / self.width);
            PixelInfo::new(color, x, y)
        };

        for (index, color) in self.pixels.iter_mut().enumerate() {
            *color = colorizer.apply(to_pixel_info(index, color.clone()))?;
        }

        Ok(())
    }

    pub fn as_vec(&self) -> Vec<u8> {
        self.pixels.iter()
            .map(|pixel| vec![pixel.r, pixel.g, pixel.b, pixel.a])
            .flatten()
            .collect()
    }

    // (x, y) denotes where the top left of the overlay will be placed on the base image
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
    pub fn place_pixel_xy(&mut self, new_pixel: HexColor, x: usize, y: usize) {
        if self.is_in_bounds(x, y) {
            let index = self.xy_to_index(x, y);
            self.pixels[index] = new_pixel
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Option<HexColor> {
        if self.is_in_bounds(x, y) {
            let index = self.xy_to_index(x, y);
            return Some(self.pixels[index]);
        }

        None
    }

    pub fn place_pixel_index(&mut self, new_pixel: HexColor, index: usize) {
        self.pixels[index] = new_pixel
    }

    pub fn index_to_xy(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn xy_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}
