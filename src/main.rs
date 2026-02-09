#![feature(assert_matches)]

mod cli;
mod config;
mod error;
mod image;
mod result;
mod runtime;
mod user_data;
mod util;

use clap::Parser;
use cli::Args;
use config::Config;
use error::AppError;
use image::Image;
use result::*;
use runtime::Runtime;
use std::{env::set_current_dir, process};
use tracing_subscriber::EnvFilter;

fn mainmain() -> Result<(), AppError> {
    let trace_format = tracing_subscriber::fmt::format()
        .compact();
    let trace_filter = EnvFilter::from_default_env();
    tracing_subscriber::fmt()
        .event_format(trace_format)
        .with_env_filter(trace_filter)
        .init();

    let args = Args::parse();

    if let Some(c) = &args.command { return c.run(&args) }

    // TODO make dir if nonexistent? populate with default config?
    if let Err(e) = set_current_dir(config::config_dir()) {
        return Err(AppError::Runtime(
            format!("Failed to open configuration directory {}:\n{}", config::config_dir(), e)
        ))
    }

    let runtime = Runtime::from_config( Config::parse()? )?;
    let path = runtime.save_path(&args.path);

    let result = ResultAnchors::new(&runtime)
        .to_tiles(&runtime)?
        .to_infos()
        .to_colors(&runtime)?
        .finalize();

    result.save(&path)
}

fn main() {
    if let Err(err) = mainmain() {
        println!("{err}");

        process::exit(1)
    }
}
