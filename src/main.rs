mod cli;
mod config;
mod error;
mod image;
mod runtime;
mod user_data;
mod result;

use error::AppError;
use cli::Args;
use clap::Parser;
use config::Config;
use runtime::Runtime;
use result::*;

use image::Image;

use std::{env::set_current_dir, process};

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

    let config = question_mark(Config::parse());
    let runtime = question_mark(Runtime::from_config(config));

    let result = ResultAnchors::new(&runtime);
    let result = question_mark(result.to_tiles(&runtime));
    let result = result.to_infos();
    let result = question_mark(result.to_colors(&runtime));
    let result = result.finalize();

    let path = runtime.save_path(&args.path);
    question_mark(result.save(&path));
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

