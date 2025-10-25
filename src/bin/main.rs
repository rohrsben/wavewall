use std::{env::set_current_dir, process};

use hex_color::HexColor;
use libwavewall::{config, image::{Image, pixel_info::PixelInfo}, tileset};
use libwavewall::config::generation::tileset::Tileset;
use rand::prelude::*;

fn main() {
    // TODO make dir if nonexistent? populate with default config?
    if let Err(e) = set_current_dir(config::config_dir()) {
        println!("Failed to open configuration directory {}:\n{}", config::config_dir(), e);
        process::exit(1)
    }

    let mut rng = rand::rng();

    let config = question_mark(config::parse());

    let tileset = match config.generation.tileset {
        Tileset::String(selection) => question_mark(tileset::parse(selection)),
        Tileset::List(options) => {
            let random_choice = options.choose(&mut rng).unwrap().clone();
            question_mark(tileset::parse(random_choice))
        }
        Tileset::Nil => {
            let mut tilesets = question_mark(tileset::parse_all());

            if tilesets.is_empty() {
                println!("No tilesets found.");
                process::exit(1)
            }

            let random_choice = rng.random_range(0..tilesets.len());
            tilesets.remove(random_choice)
        }
    };

    let tileset = question_mark(tileset.into_runtime());

    let mut result = Image::new(config.output.size.width, config.output.size.height);
    let (x_offset, y_offset) = match config.generation.offset {
        true => (
            rng.random_range(0..tileset.width),
            rng.random_range(0..tileset.height)
        ),
        false => (0, 0)
    };

    result.generate_placement_points(x_offset, y_offset, tileset.width, tileset.height);

    println!("Beginning generation...");
    while !result.placement_points.is_empty() {
        // TODO placement_points prob doesn't need to be a VecDeque
        let (x, y) = result.placement_points.pop_front().unwrap();
        let tile = tileset.get_tile();
        result.overlay_image(&tile.image, x, y);
    }

    if let Some(colorizer) = tileset.colorizer {
        println!("Beginning re-color...");
        for (index, pixel) in result.pixels.iter_mut().enumerate() {
            let (x, y) = (index % result.width, index / result.width);
            let info = PixelInfo::new(*pixel, x, y);

            // TODO handle _much_ better
            let return_val = colorizer.call::<mlua::Table>(info).unwrap();
            let r = return_val.get::<mlua::Integer>("r").unwrap() as u8;
            let g = return_val.get::<mlua::Integer>("g").unwrap() as u8;
            let b = return_val.get::<mlua::Integer>("b").unwrap() as u8;
            let a = return_val.get::<mlua::Integer>("a").unwrap() as u8;

            let new_color = HexColor::rgba(r, g, b, a);
            *pixel = new_color;
        }
    }

    let path = config.output.filepath();
    println!("Saving to: {}", path);
    if let Err(e) = result.save(&path) {
        println!("{}", e);
    };
}

fn question_mark<T>(input: Result<T, libwavewall::error::AppError>) -> T {
    match input {
        Ok(result) => result,
        Err(e) => {
            println!("{}", e);
            process::exit(1)
        }
    }
}
