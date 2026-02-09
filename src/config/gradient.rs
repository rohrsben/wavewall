use hex_color::HexColor;
use mlua::Value;
use crate::{config::{parse, Location}, error::AppError};
use palette::{convert::FromColorUnclamped, FromColor, Okhsv, Oklab, Srgb, Clamp};
use tracing::{debug, trace};
use crate::opt_simple;

pub fn gradient_wrapper(input: Value) -> Result<HexColor, mlua::Error> {
    let grad = match parse_gradient(input) {
        Ok(g) => g,
        Err(e) => return Err(mlua::Error::RuntimeError(format!("{e}")))
    };

    let color = calculate_gradient(&grad);
    debug!("gradient: {:?} -> {}", grad, color.display_rgba());

    Ok(color)
}

#[derive(Debug)]
struct Gradient {
    stops: Vec<HexColor>,
    at: f64
}

fn parse_gradient(input: Value) -> Result<Gradient, AppError> {
    let loc = Location::new("gradient args");
    match input {
        Value::Table(table) => {
            let stops = parse_stops( table.get::<Value>("stops")?, &loc)?;
            opt_simple!(at, float_necessary, table, loc);

            Ok(Gradient { stops, at })
        }

        _ => Err(AppError::Runtime(
            format!("While calling gradient: argument had incorrect type\n  Expected: table\n  Got: {}", input.type_name())
        ))
    }
}

fn parse_stops(input: Value, loc: &Location) -> Result<Vec<HexColor>, AppError> {
    let loc = loc.add_parent("stops");

    match input {
        Value::Table(table) => {
            let mut stops = Vec::new();

            if table.sequence_values::<Value>().count() > 0 {
                for item in table.sequence_values::<Value>() {
                    let item = item?;
                    let color = match parse::color(item) {
                        Ok(color) => color,
                        Err(e) => {
                            let loc = loc.add_parent("table item");
                            return Err(AppError::Runtime( format!("At {loc}: {e}")))
                        }
                    };
                     
                    stops.push(color);
                }
            } else {
                return Err(AppError::EmptyTable(loc.to_string()))
            }

            trace!(?stops);

            Ok(stops)
        }

        _ => Err(AppError::Runtime(
            format!("{loc} was of incorrect type:\n  Expected: table\n  Got: {}", input.type_name())
        ))
    }
}

fn calculate_gradient(grad: &Gradient) -> HexColor {
    let Gradient { stops, at } = grad;
    trace!(at);

    if *at <= 0.0 { return *stops.first().unwrap() }
    if *at >= 1.0 { return *stops.last().unwrap() }

    let n = stops.len() - 1;
    trace!(n);
    let i = f64::floor(at * n as f64) as usize;
    trace!(i);
    let at_local = (at - (i as f64 / n as f64)) / (1.0 / n as f64);
    trace!(at_local);
    let start_color = to_oklab(stops[i]);
    trace!(?start_color);
    let end_color = to_oklab(stops[i + 1]);
    trace!(?end_color);

    let new_color = Oklab::from_components((
        start_color.l + at_local as f32 * (end_color.l - start_color.l),
        start_color.a + at_local as f32 * (end_color.a - start_color.a),
        start_color.b + at_local as f32 * (end_color.b - start_color.b)
    ));

    from_oklab(new_color)
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
