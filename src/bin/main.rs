use std::{env::set_current_dir, process};

use libwavewall::{config, image::Image, tileset::{self, tsconfig::colorizer::Colorizer}};
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

    // TODO this is... weird... but probably not a big deal?
    if !matches!(tileset.colorizer, Colorizer::Nil) {
        println!("Beginning re-color...");

        question_mark(result.recolor(&tileset.colorizer));
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
