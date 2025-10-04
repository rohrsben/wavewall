use std::fs::File;
// use std::io::BufWriter;
// use std::path::Path;

use png::Decoder;
use std::io::BufReader;

// use mlua::Lua;
//
use libwavewall::config::Config;
// use libwavewall::tileset::Tileset;
use libwavewall::image::Image;

fn main() {
    let path = format!("{}/testing/cross.png", Config::get_config_dir());

    let res = Image::from_path(path);
    println!("res: {:?}", res);
}
