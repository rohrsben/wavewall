use hex_color::HexColor;

#[derive(Debug)]
pub struct ColorInfo {
    color: HexColor
}

impl mlua::UserData for ColorInfo {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("r", |_, this| Ok(this.color.r));
        fields.add_field_method_get("g", |_, this| Ok(this.color.g));
        fields.add_field_method_get("b", |_, this| Ok(this.color.b));
        fields.add_field_method_get("a", |_, this| Ok(this.color.a));

        fields.add_field_method_get("hex", |_, this| Ok(this.color.display_rgba().to_string()));
    }
}

impl ColorInfo {
    pub fn new(color: HexColor) -> Self {
        ColorInfo { color }
    }
}

#[derive(Debug)]
pub struct PixelInfo {
    pub color: HexColor,
    x: usize,
    y: usize,
}

impl mlua::UserData for PixelInfo {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("r", |_, this| Ok(this.color.r));
        fields.add_field_method_get("g", |_, this| Ok(this.color.g));
        fields.add_field_method_get("b", |_, this| Ok(this.color.b));
        fields.add_field_method_get("a", |_, this| Ok(this.color.a));

        fields.add_field_method_get("hex", |_, this| Ok(this.color.display_rgba().to_string()));
        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_get("y", |_, this| Ok(this.y));
    }
}

impl PixelInfo {
    pub fn new(color: HexColor, x: usize, y: usize) -> Self {
        PixelInfo {
            color,
            x, y
        }
    }
}
