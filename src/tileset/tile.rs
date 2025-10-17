use crate::{error::AppError, image::Image};

#[derive(Debug)]
pub struct Tile {
    image: Image,
}

impl Tile {
    pub fn from_path(path: &str) -> Result<Self, AppError> {
        let image = Image::from_path(path)?;

        Ok(Tile {
            image
        })
    }
}
