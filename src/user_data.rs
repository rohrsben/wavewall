use std::rc::Rc;

use hex_color::HexColor;
use mlua::UserData;

#[derive(Debug)]
pub struct ColorInfo {
    color: HexColor
}

impl UserData for ColorInfo {
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
    pub y: i64,
    pub x: i64,
    pub color: HexColor,
    pub tile_x: usize,
    pub tile_y: usize,
    pub tile_name: Rc<String>,
    pub anchor_x: i64,
    pub anchor_y: i64,
}

impl UserData for PixelInfo {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("r", |_, this| Ok(this.color.r));
        fields.add_field_method_get("g", |_, this| Ok(this.color.g));
        fields.add_field_method_get("b", |_, this| Ok(this.color.b));
        fields.add_field_method_get("a", |_, this| Ok(this.color.a));
        fields.add_field_method_get("hex", |_, this| Ok(this.color.display_rgba().to_string()));

        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_get("y", |_, this| Ok(this.y));
        fields.add_field_method_get("anchor_x", |_, this| Ok(this.anchor_x));
        fields.add_field_method_get("anchor_y", |_, this| Ok(this.anchor_y));

        fields.add_field_method_get("tile_x", |_, this| Ok(this.tile_x));
        fields.add_field_method_get("tile_y", |_, this| Ok(this.tile_y));
        fields.add_field_method_get("tile_name", |_, this| Ok(
            (*this.tile_name).clone()
        ));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Anchor {
    pub x: i64,
    pub y: i64
}

impl UserData for Anchor {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_get("y", |_, this| Ok(this.y));
    }
}

impl Anchor {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn scale_by(&self, scale: i64) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale
        }
    }

    pub fn with_offset(&self, x_off: i64, y_off: i64) -> Self {
        Self {
            x: self.x + x_off,
            y: self.y + y_off
        }
    }
}
