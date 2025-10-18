use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    ConfigLua(#[from] mlua::Error),
    #[error("Incorrect type for '{0}'\n  Expected: {1}\n  Got: {2}")]
    ConfigType(&'static str, &'static str, String),
    #[error("Incorrect type for '{0}'\n  Expected: {1}\n  Got: {2}")]
    ConfigTypeSpecific(String, &'static str, String),
    #[error("Item with incorrect type in list {0}:\n  Expected: {1}\n  Got: {2}")]
    ConfigTypeListItemSpecific(String, &'static str, String),

    #[error("{0}")]
    Runtime(String),

    #[error("Error decoding image: {0}")]
    ImageDecode(#[from] png::DecodingError),
    #[error("Error encoding image: {0}")]
    ImageEncode(#[from] png::EncodingError),

    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
}
