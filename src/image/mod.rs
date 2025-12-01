pub mod transform;

pub use transform::Transform;

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use png::{Decoder, Encoder};
use hex_color::HexColor;

use crate::error::AppError;


#[derive(Debug, Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<HexColor>
}

impl Image {
    pub fn create_template(length: usize) -> Self {
        let mut template = Self::new(length, length);

        let mut midpoints = Vec::new();
        if length % 2 == 0 {
            midpoints.push(length / 2 - 1);
        }
        midpoints.push(length / 2);

        let xy_to_index = |x: usize, y: usize| y * length + x;
        let mut indices = Vec::new();
        for mp in midpoints {
            indices.push(xy_to_index(0, mp));
            indices.push(xy_to_index(mp, 0));
            indices.push(xy_to_index(mp, length - 1));
            indices.push(xy_to_index(length - 1, mp));
        }

        for index in indices {
            template.pixels[index] = HexColor::RED;
        }

        template
    }

    pub fn from_path(path: PathBuf) -> Result<Self, AppError> {
        let file = File::open(path)?;

        let mut decoder = Decoder::new(BufReader::new(file));
        decoder.set_transformations(png::Transformations::ALPHA);

        let mut reader = decoder.read_info()?;

        // the unwrap here is so unlikely to fail, and so problematic if it does, that its fine
        let mut output = vec![0; reader.output_buffer_size().unwrap()];
        let info = reader.next_frame(&mut output)?;

        let mut image = Self::new(info.width as usize, info.height as usize);
        for i in 0..(image.width * image.height) {
            image.pixels[i] = HexColor::rgba(
                output[i*4 + 0], 
                output[i*4 + 1], 
                output[i*4 + 2], 
                output[i*4 + 3]
            );
        }

        Ok(image)
    }

    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![HexColor::BLACK; width * height];

        Self {
            width,
            height,
            pixels,
        }
    }

    // this is destructive!
    pub fn save(self, path: &str) -> Result<(), AppError> {
        let save_file = File::create(path)?;
        let ref mut w = BufWriter::new(save_file);
        let mut encoder = Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let data = self.pixels.iter()
            .map(|pixel| vec![pixel.r, pixel.g, pixel.b, pixel.a])
            .flatten()
            .collect::<Vec<_>>();
        writer.write_image_data(&data)?;

        Ok(())
    }

    pub fn create_transform(&self, transform: Transform) -> Self {
        let mut new_image = Image::new(self.width, self.height);
        let xy_to_index = |x: usize, y: usize| y * self.width + x;
        let index_to_xy = |index: usize| (index % self.width, index / self.width);

        // more performant than matching inside the closure... but at what cost?
        let converter: Box<dyn Fn(usize) -> usize > = match transform {
            Transform::TurnOnce => Box::new(|index| {
                let (old_x, old_y) = index_to_xy(index);
                xy_to_index(
                    old_y,
                    self.width - old_x - 1
                )
            }),
            Transform::TurnTwice => Box::new(|index| {
                let (old_x, old_y) = index_to_xy(index);
                xy_to_index(
                    self.width - old_x - 1,
                    self.width - old_y - 1
                )
            }),
            Transform::TurnThrice => Box::new(|index| {
                let (old_x, old_y) = index_to_xy(index);
                xy_to_index(
                    self.width - old_y - 1,
                    old_x
                )
            }),
            Transform::Horizontal => Box::new(|index| {
                let (old_x, old_y) = index_to_xy(index);
                xy_to_index(
                    old_x,
                    self.width - old_y - 1
                )
            }),
            Transform::Vertical => Box::new(|index| {
                let (old_x, old_y) = index_to_xy(index);
                xy_to_index(
                    self.width - old_x - 1,
                    old_y
                )
            }),
            Transform::Diagonal => Box::new(|index| {
                let (old_x, old_y) = index_to_xy(index);
                xy_to_index(
                    old_y,
                    old_x
                )
            }),
            Transform::Antidiagonal => Box::new(|index| {
                let (old_x, old_y) = index_to_xy(index);
                xy_to_index(
                    self.width - old_y - 1,
                    self.width - old_x - 1
                )
            })
        };

        for (index, pixel) in self.pixels.iter().enumerate() {
            new_image.pixels[converter(index)] = *pixel;
        }

        new_image
    }

    // pub fn itxy(&self) -> impl Fn(usize) -> (usize, usize) {
    //     |index: usize| (index % width, index / height)
    // }

    pub fn xyti(&self) -> impl Fn(usize, usize) -> usize + use<> {
        let width = self.width;

        move |x: usize, y: usize| y*width + x
    }

    pub fn in_bounds(&self) -> impl Fn(i64, i64) -> bool + use<> {
        let wrange = 0..self.width as i64;
        let hrange = 0..self.height as i64;

        move |x: i64, y: i64| {
            hrange.contains(&y) && wrange.contains(&x)
        }
    }
}

