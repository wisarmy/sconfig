pub mod error;
pub mod types;

use std::{fmt::Display, fs, path::PathBuf};

use error::Result;
use serde::Serialize;
use tracing::info;

pub use types::FileType;

pub trait Configurable: Serialize + Display + Definable {
    /// config dir
    fn config_dir(&self) -> PathBuf;
    #[inline]
    fn config_name(&self) -> String {
        format!("config.{}", self.config_type().to_string())
    }
    /// config file path
    #[inline]
    fn config_path(&self) -> PathBuf {
        self.config_dir().join(self.config_name())
    }
    #[inline]
    fn init(&self) -> Result<()> {
        if self.config_path().exists() {
            return Err(config_error!(
                "config file {} already exists!",
                self.config_path().display(),
            ));
        }
        fs::create_dir_all(self.config_dir())?;
        self.write()?;
        info!("Initializing config dir at {}", self.config_dir().display());
        Ok(())
    }
    /// save change
    #[inline]
    fn write(&self) -> Result<()> {
        let config_string = self.to_string();

        fs::write(self.config_path(), config_string)?;
        Ok(())
    }
}
pub trait Definable {
    fn config_type(&self) -> FileType;
}
