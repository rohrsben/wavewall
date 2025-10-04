use std::fs::File;
// use std::io::BufWriter;
// use std::path::Path;

use png::Decoder;
use std::io::BufReader;

// use mlua::Lua;
//
// use libwavewall::config::Config;
// use libwavewall::tileset::Tileset;
// use libwavewall::image::Image;

fn main() {
    let mut decoder = Decoder::new(BufReader::new(File::open("/home/error/.config/wavewall/testing/cross.png").unwrap()));
    decoder.set_transformations(png::Transformations::EXPAND);
    let mut reader = decoder.read_info().unwrap();
    let mut output = vec![0; reader.output_buffer_size().unwrap()];

    let info = reader.next_frame(&mut output).unwrap();

    println!("info: {:?}", info);

    let whoknows = reader.output_color_type();
    println!("whoknows: {:?}", whoknows);

    // let lua = Lua::new();
    //
    // let res = match Config::get_config(&lua) {
    //     Ok(conf) => conf,
    //     Err(e) => {
    //         println!("e: {:?}", e);
    //         return;
    //     }
    // };
    //
    // let tilesets = Tileset::load_tilesets();
    //
    // let testing = tilesets.get("testing").unwrap();
    //
    // let corner = testing.tiles.get("corner").unwrap();
    // let jump = testing.tiles.get("jump").unwrap();
    // let cross = testing.tiles.get("cross").unwrap();
    //
    // let mut result = Image::new(32, 2, 3);
    //
    // println!("STARTING");
    // result.place_tile(corner, 0, 0);
    // result.place_tile(jump, 0, 1);
    // result.place_tile(cross, 0, 2);
    // result.place_tile(jump, 1, 0);
    // result.place_tile(corner, 1, 1);
    // result.place_tile(cross, 1, 2);
    // println!("FINISHED");

    // let data = result.get_flattened();
    // let ref mut w = BufWriter::new(result.file);
    // let mut encoder = png::Encoder::new(w, 96, 64);
    //
    // encoder.set_color(png::ColorType::Rgb);
    // encoder.set_depth(png::BitDepth::Eight);
    //
    // let mut writer = encoder.write_header().unwrap();
    // writer.write_image_data(&data).unwrap();
    // println!("cross: {:?}", cross);
}
