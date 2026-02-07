use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    Lua(#[from] mlua::Error),

    #[error("Incorrect type ({location}):\n  Expected: {expected}\n  Got: {got}")]
    IncorrectType { 
        location: String,
        expected: String,
        got: String
    },
    #[error("Unexpectedly empty table: '{0}'")]
    EmptyTable(String),

    #[error("{0}")]
    Runtime(String),
    #[error("Incorrect return type while calling colorizer on '{0}':\n  Expected: string, table\n  Got: {1}")]
    RuntimeColorizerReturnType(String, String),

    #[error("Failed to convert string to a Transform")]
    TransformParse,

    #[error("Error decoding image: {0}")]
    ImageDecode(#[from] png::DecodingError),
    #[error("Error encoding image: {0}")]
    ImageEncode(#[from] png::EncodingError),

    #[error("Error choosing item: {0}")]
    RandChoice(#[from] rand::distr::weighted::Error),

    #[error("Error converting color code: {0}")]
    HexColor(#[from] hex_color::ParseHexColorError),

    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
}

pub struct TypeError {
    pub expected: String,
    pub got: String
}

impl TypeError {
    pub fn with_location(self, location: impl Display) -> TypeErrorLocation {
        let Self { expected, got } = self;

        TypeErrorLocation {
            location: location.to_string(),
            expected,
            got
        }
    }
}

#[derive(Debug)]
pub struct TypeErrorLocation {
    pub location: String,
    pub expected: String,
    pub got: String
}

impl Into<AppError> for TypeErrorLocation {
    fn into(self) -> AppError {
        let Self { location, expected, got } = self;

        AppError::IncorrectType { location, expected, got }
    }
}
