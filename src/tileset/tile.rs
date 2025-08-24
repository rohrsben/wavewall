use std::fs::File;
use png::Decoder;

#[derive(Debug)]
pub struct Tile {
    length: u32,
    pixels: Vec<u8>
}

impl Tile {
    pub fn from_path(path: String) -> Self {
        let decoder = Decoder::new(File::open(path).unwrap());
        let mut reader = decoder.read_info().unwrap();
        let mut output = vec![0; reader.output_buffer_size()];

        let info = reader.next_frame(&mut output).unwrap();

        Tile {
            length: info.width,
            pixels: output
        }
    }
}
