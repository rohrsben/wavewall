use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    ConfigLua(#[from] mlua::Error),
    #[error("Incorrect type for '{0}'\n  Expected: {1}\n  Got: {2}")]
    ConfigType(String, &'static str, String),
    #[error("Item with incorrect type in list '{0}':\n  Expected: {1}\n  Got: {2}")]
    ConfigTypeListItem(String, &'static str, String),
    #[error("Item with incorrect type in table '{0}':\n  Expected: {1}\n  Got: {2}")]
    ConfigTypeTableItem(String, &'static str, String), // TODO specify the item?
    #[error("Unexpectedly empty table: '{0}'")]
    ConfigEmptyTable(String),

    #[error("{0}")]
    Runtime(String),

    #[error("Error decoding image: {0}")]
    ImageDecode(#[from] png::DecodingError),
    #[error("Error encoding image: {0}")]
    ImageEncode(#[from] png::EncodingError),

    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
}
