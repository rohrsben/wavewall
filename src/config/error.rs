use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to open configuration file: {0}")]
    Read(#[from] std::io::Error),

    #[error("{0}")]
    GeneralMlua(#[from] mlua::Error),

    #[error("Incorrect type for '{0}'\n  Expected: {1}\n  Got: {2}")]
    Type(&'static str, &'static str, String),
}
