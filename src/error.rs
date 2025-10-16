use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    ConfigLua(#[from] mlua::Error),
    #[error("Incorrect type for '{0}'\n  Expected: {1}\n  Got: {2}")]
    ConfigType(&'static str, &'static str, String),

    #[error("Error decoding image: {0}")]
    ImageDecode(#[from] png::DecodingError),

    #[error("Failed to open file: {0}")]
    IO(#[from] std::io::Error),
}
