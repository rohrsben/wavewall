use hex_color::HexColor;
use mlua::{Number, Value};
use crate::config::parse;
use palette::{convert::FromColorUnclamped, FromColor, Okhsv, Oklab, Srgb, Clamp};

pub fn gradient_wrapper(input: Value) -> Result<HexColor, mlua::Error> {
    let args = GradientArgs::parse(input)?;

    Ok(get_gradient_value(args))
}

struct GradientArgs {
    start: HexColor,
    last: HexColor,
    at: f64
}

impl GradientArgs {
    fn parse(input: mlua::Value) -> Result<Self, mlua::Error> {
        match input {
            mlua::Value::Table(table) => {
                let start = match parse::color(table.get::<Value>("start")?) {
                    Ok(color) => color,
                    Err(e) => return Err(mlua::Error::RuntimeError(
                        format!("While processing 'gradient(start)': {e}")
                    ))
                };

                let last = match parse::color(table.get::<Value>("last")?) {
                    Ok(color) => color,
                    Err(e) => return Err(mlua::Error::RuntimeError(
                        format!("While processing 'gradient(last)': {e}")
                    ))
                };

                let at = match table.get::<Number>("at") {
                    Ok(at) => at,
                    Err(e) => return Err(mlua::Error::RuntimeError(
                        format!("While processing 'gradient(at)': {e}")
                    ))
                };

                Ok(Self { start, last, at })
            }
            _ => Err(mlua::Error::RuntimeError(format!("While calling provided function 'gradient':\n  Expected: GradientArgs\n  Got: {}", input.type_name())))
        }
    }
}

fn to_oklab(color: HexColor) -> Oklab {
    let srgb: Srgb::<f32> = Srgb::new(color.r, color.g, color.b).into_format();
    
    Oklab::from_color(srgb)
}

fn from_oklab(color: Oklab) -> HexColor {
    // as recommended by the palette docs
    let hsv = Okhsv::from_color_unclamped(color).clamp();
    let srgb: Srgb<u8> = Srgb::from_color_unclamped(hsv).into();

    HexColor::rgb(srgb.red, srgb.green, srgb.blue)
}


fn get_gradient_value(args: GradientArgs) -> HexColor {
    let loc = args.at as f32;

    if loc <= 0.0 { return args.start }
    if loc >= 1.0 { return args.last }

    let (sl, sa, sb) = to_oklab(args.start).into_components();
    let (el, ea, eb) = to_oklab(args.last).into_components();

    let new = Oklab::new(
        sl + loc * (el - sl),
        sa + loc * (ea - sa),
        sb + loc * (eb - sb)
    );

    from_oklab(new)
}
