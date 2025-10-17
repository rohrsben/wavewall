use std::{env::set_current_dir, process};

use libwavewall::{tileset, config};

fn main() {
    // TODO make dir if nonexistent? populate with default config?
    if let Err(e) = set_current_dir(config::config_dir()) {
        println!("Failed to open configuration directory {}:\n{}", config::config_dir(), e);
        process::exit(1)
    }

    let res = tileset::parse("testing");

    match res {
        Ok(some) => println!("some: {:?}", some),
        Err(e) => println!("Got error: {}", e),
    }
}
