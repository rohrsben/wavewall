use libwavewall;
use libwavewall::tileset::Tileset;

fn main() {
    let res = Tileset::get_tileset_dirs();

    for ts in res {
        let pngs = Tileset::get_png_names(ts);

        println!("pngs: {:?}", pngs);
    }
}










