use std::fs;

use mlua::{Lua, Table};

use libwavewall::config::Config;

fn main() {
    let lua = Lua::new();

    let res = match Config::parse_config(&lua) {
        Ok(conf) => conf,
        Err(e) => {
            println!("e: {:?}", e);
            panic!()
        }
    };

    println!("Chosen tileset: {}", res.tileset);
}
