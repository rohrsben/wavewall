use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Failed to read configuration file:\n{0}")]
    Read(#[from] std::io::Error),

    #[error("Syntax error:\n{0}")]
    Syntax(String),

    #[error("Lua error:\n{0}")]
    OtherLua(#[from] mlua::Error),

    #[error("A value provided in the configuration did not match the expected type:\n{0}")]
    Deserialize(String)
}
