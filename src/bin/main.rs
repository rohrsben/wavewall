use std::fs::File;
use std::io::BufWriter;

use libwavewall::config::Config;
use libwavewall::image::Image;
use png::Encoder;

fn img(title: &str) -> Image {
    Image::from_path(format!("{}/testing/{}.png", Config::get_config_dir(), title)).unwrap()
}

fn main() {
    let cross = img("cross");
    let corner = img("corner");
    let jump = img("jump");

    let mut result = Image::new(cross.width*3, cross.height*2);

    result.overlay_image(&corner, 0, 0);
    result.overlay_image(&corner, 32, 0);
    result.overlay_image(&corner, 64, 0);
    result.overlay_image(&corner, 0, 32);
    result.overlay_image(&corner, 32, 32);
    result.overlay_image(&corner, 64, 32);

    let file_path = format!("{}/result.png", Config::get_config_dir());
    let file = File::create(file_path).unwrap();

    let ref mut w = BufWriter::new(file);

    let mut encoder = Encoder::new(w, result.width as u32, result.height as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    let data = result.as_vec();
    writer.write_image_data(&data).unwrap();
}
