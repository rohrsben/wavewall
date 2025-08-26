use libwavewall;
use libwavewall::tileset::Tileset;

fn main() {
    let tilesets = Tileset::load_tilesets();

    println!("tilesets: {:?}", tilesets);
}










