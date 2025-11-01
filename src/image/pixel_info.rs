use hex_color::HexColor;

#[derive(Debug)]
pub struct PixelInfo {
    pub pixel: HexColor,
    x: usize,
    y: usize,
}

impl mlua::UserData for PixelInfo {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("r", |_, this| Ok(this.pixel.r));
        fields.add_field_method_get("g", |_, this| Ok(this.pixel.g));
        fields.add_field_method_get("b", |_, this| Ok(this.pixel.b));
        fields.add_field_method_get("a", |_, this| Ok(this.pixel.a));

        fields.add_field_method_get("hex", |_, this| Ok(this.pixel.display_rgba().to_string()));
        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_get("y", |_, this| Ok(this.y));
    }
}

impl PixelInfo {
    pub fn new(pixel: HexColor, x: usize, y: usize) -> PixelInfo {
        PixelInfo {
            pixel,
            x, y
        }
    }
}
