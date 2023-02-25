use std::io;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ConfigError>;
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("unknown error")]
    Unknown,
    #[error("{0}")]
    Error(String),
    #[error("io error")]
    IO(#[from] io::Error),
    #[error("toml error")]
    Toml(#[from] toml::ser::Error),
    #[error("toml serialize error")]
    TomlDe(#[from] toml::de::Error),
    #[error("toml deserialize error")]
    SerdeJson(#[from] serde_json::Error),
}
#[macro_export]
macro_rules! config_error {
    ($($arg:tt)*) => {{
        $crate::error::ConfigError::Error(format!($($arg)*))
    }}
}
