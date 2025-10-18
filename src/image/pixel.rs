#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Pixel {
    pub fn new() -> Self {
        Pixel {
            r: 1,
            g: 1,
            b: 1,
            a: 2
        }
    }

    pub fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Pixel {
            r, g, b,
            a: 0xff
        }
    }

    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Pixel { r, g, b, a }
    }

    pub fn set_r(&mut self, new_r: u8) {
        self.r = new_r
    }

    pub fn set_g(&mut self, new_g: u8) {
        self.g = new_g
    }

    pub fn set_b(&mut self, new_b: u8) {
        self.b = new_b
    }

    pub fn set_a(&mut self, new_a: u8) {
        self.a = new_a
    }

    pub fn as_vec(&self) -> Vec<u8> {
        vec![self.r, self.g, self.b, self.a]
    }
}
