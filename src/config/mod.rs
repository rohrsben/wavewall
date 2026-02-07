mod gradient;
mod parse;
mod location;
pub mod colorizer;
pub mod output;
pub mod tileset;

pub use output::Output;
pub use colorizer::Colorizer;
pub use tileset::TilesetConfig;
pub use location::Location;

use crate::error::AppError;
use crate::opt_complex;
use crate::user_data::ColorInfo;
use hex_color::HexColor;
use mlua::{Lua, Value};

#[derive(Debug)]
pub struct Config {
    pub lua: Lua,
    pub colorizer: Option<Colorizer>,
    pub output: Output,
    pub tileset: TilesetConfig,
}

impl Config {
    pub fn parse() -> Result<Self, AppError> {
        let lua = Self::new_lua()?;

        let config = lua.load(std::fs::read_to_string(config_file())?)
            .set_name("@wavewall.lua")
            .eval::<mlua::Table>()?;

        let loc = Location::new("wavewall");

        opt_complex!(colorizer, config, loc);
        opt_complex!(output,    config, loc);
        opt_complex!(tileset,   config, loc);

        Ok(Self {
            lua,
            colorizer,
            output,
            tileset,
        })
    }

    fn new_lua() -> Result<Lua, AppError> {
        let lua = Lua::new();

        let convert_rgb = lua.create_function(|_, (r, g, b)| {
            let color = HexColor::rgb(r, g, b);

            Ok(ColorInfo::new(color))
        })?;
        lua.globals().set("convert_rgb", convert_rgb)?;

        let convert_rgba = lua.create_function(|_, (r, g, b, a)| {
            let color = HexColor::rgba(r, g, b, a);

            Ok(ColorInfo::new(color))
        })?;
        lua.globals().set("convert_rgba", convert_rgba)?;

        let convert_hex = lua.create_function(|_, hex: String| {
            let color = match HexColor::parse(&hex) {
                Ok(color) => color,
                Err(e) => return Err(mlua::Error::RuntimeError(format!("While calling hex_to_rgba: failed to parse '{hex}': {}", e)))
            };

            Ok(ColorInfo::new(color))
        })?;
        lua.globals().set("convert_hex", convert_hex)?;

        lua.load(r"
            function create_all_pseudos(original)
                return {
                    [original .. '_90'] = '90',
                    [original .. '_180'] = '180',
                    [original .. '_270'] = '270',
                    [original .. '_horizontal'] = 'horizontal',
                    [original .. '_vertical'] = 'vertical',
                    [original .. '_diagonal'] = 'diagonal',
                    [original .. '_antidiagonal'] = 'antidiagonal',
                }
            end
        ").exec()?;

        let gradient = lua.create_function(|_, args: Value| {
            let color = gradient::gradient_wrapper(args)?;
            Ok(ColorInfo::new(color))
        })?;
        lua.globals().set("gradient", gradient)?;

        Ok(lua)
    }
}

pub fn config_dir() -> String {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        format!("{xdg}/wavewall")
    } else {
        let user = std::env::var("USER").unwrap();
        format!("/home/{user}/.config/wavewall")
    }
}

pub fn config_file() -> String {
    format!("{}/wavewall.lua", config_dir())
}
