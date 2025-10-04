use std::fs::File;
use std::io::BufReader;
use png::Decoder;

#[derive(Debug)]
pub struct Tile {
    length: u32,
    pixels: Vec<u8>
}

impl Tile {
    pub fn from_path(path: String) -> Self {
        let decoder = Decoder::new(BufReader::new(File::open(path).unwrap()));
        let mut reader = decoder.read_info().unwrap();
        let mut output = vec![0; reader.output_buffer_size().unwrap()];

        let info = reader.next_frame(&mut output).unwrap();

        Tile {
            length: info.width,
            pixels: output
        }
    }
}
