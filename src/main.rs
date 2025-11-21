mod cli;
mod config;
mod error;
mod image;
mod runtime;
mod user_data;

use error::AppError;
use cli::Args;
use clap::Parser;
use config::Config;
use runtime::Runtime;

use image::Image;

use std::{env::set_current_dir, process};

use rand::prelude::*;

fn main() {
    let args = Args::parse();

    match &args.command {
        Some(c) => {
            question_mark(c.run(&args));
            process::exit(0)
        }
        None => {}
    }

    // TODO make dir if nonexistent? populate with default config?
    if let Err(e) = set_current_dir(config::config_dir()) {
        println!("Failed to open configuration directory {}:\n{}", config::config_dir(), e);
        process::exit(1)
    }

    let mut rng = rand::rng();

    let config = question_mark(Config::parse());

    let runtime = question_mark(Runtime::from_config(config));

    let mut result = Image::new(runtime.output.size.width, runtime.output.size.height);
    let (x_offset, y_offset) = match runtime.output.offset {
        true => (
            rng.random_range(0..runtime.tile_size),
            rng.random_range(0..runtime.tile_size)
        ),
        false => (0, 0)
    };

    result.generate_placement_points(x_offset, y_offset, runtime.tile_size);

    println!("Beginning generation...");
    while !result.placement_points.is_empty() {
        // TODO placement_points prob doesn't need to be a VecDeque
        let (x, y) = result.placement_points.pop_front().unwrap();
        let tile = runtime.get_tile_random();
        result.overlay_image(&tile, x, y);
    }

    // TODO this is... weird... but probably not a big deal?
    if let Some(colorizer) = &runtime.colorizer {
        println!("Beginning re-color...");

        question_mark(result.recolor(colorizer));
    }

    let path = runtime.save_path(&args.path);
    println!("Saving to: {}", path);
    if let Err(e) = result.save(&path) {
        println!("{}", e);
    };
}

fn question_mark<T>(input: Result<T, AppError>) -> T {
    match input {
        Ok(result) => result,
        Err(e) => {
            println!("{}", e);
            process::exit(1)
        }
   }
}

