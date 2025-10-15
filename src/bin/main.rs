use std::fs;

use mlua::Lua;

use libwavewall::parse;
use libwavewall::config;

fn main() {
    let lua = Lua::new();

    let conf = fs::read_to_string(config::config_file()).unwrap();
    let res = lua.load(conf);

    let res2 = res.eval::<mlua::Table>().unwrap();

    println!("res2: {:?}", res2);

    let res3 = res2.get::<mlua::Table>("output").unwrap();
    println!("res3: {:?}", res3);

    let res4 = res3.get::<mlua::Value>("filename").unwrap();
    match res4 {
        mlua::Value::String(str) => println!("Got a string: {:?}", str),
        mlua::Value::Integer(int) => println!("Got an int: {}", int),
        _ => println!("Got an other: {}", 4)
    };
}
