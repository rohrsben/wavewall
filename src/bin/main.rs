use std::{env::set_current_dir, process};

use libwavewall::{config, image::Image, tileset};

fn main() {
    // TODO make dir if nonexistent? populate with default config?
    if let Err(e) = set_current_dir(config::config_dir()) {
        println!("Failed to open configuration directory {}:\n{}", config::config_dir(), e);
        process::exit(1)
    }

    let tileset = tileset::parse("testing").unwrap();
    let config = config::parse().unwrap();

    let mut result = Image::new(900, 900);
    while result.next_free_xy() != None {
        let (x, y) = result.next_free_xy().unwrap();
        let tile = tileset.get_tile().unwrap();
        result.overlay_image(&tile.image, x, y);
    }


    let path = config.output.filepath();
    println!("Saving to: {}", path);
    result.save(&path);
}
