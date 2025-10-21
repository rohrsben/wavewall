use std::{env::set_current_dir, process};

use libwavewall::{config, error::{self, AppError}, image::Image, tileset};
use rand::seq::IteratorRandom;

fn main() {
    // TODO make dir if nonexistent? populate with default config?
    if let Err(e) = set_current_dir(config::config_dir()) {
        println!("Failed to open configuration directory {}:\n{}", config::config_dir(), e);
        process::exit(1)
    }

    let config = match config::parse() {
        Ok(result) => result,
        Err(e) => {
            println!("{}", e);
            process::exit(1)
        }
    };

    let mut tilesets = match tileset::parse_all() {
        Ok(result) => result,
        Err(e) => {
            println!("{}", e);
            process::exit(1)
        }
    };

    if tilesets.is_empty() {
        println!("No tilesets found.");
        process::exit(1)
    }

    let tileset = match config.tileset {
        Some(selection) => match tilesets.remove(&selection) {
            Some(ts) => ts,
            None => {
                println!("No tileset found with name: {}", selection);
                process::exit(1)
            }
        }
        None => {
            let random_choice = tilesets.keys().choose(&mut rand::rng()).unwrap().clone();
            tilesets.remove(&random_choice).unwrap()
        }
    };

    let tileset = match tileset.into_runtime() {
        Ok(result) => result,
        Err(e) => {
            println!("{}", e);
            process::exit(1)
        }
    };

    let mut result = Image::new(config.output.size.width, config.output.size.height);

    println!("Beginning generation...");
    while result.next_free_xy() != None {
        let (x, y) = result.next_free_xy().unwrap();
        let tile = tileset.get_tile();
        result.overlay_image(&tile.image, x, y);
    }

    let path = config.output.filepath();
    println!("Saving to: {}", path);
    result.save(&path);
}
