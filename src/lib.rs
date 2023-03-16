pub mod error;
pub mod types;

use std::{fmt::Display, fs, path::PathBuf};

use error::Result;
use serde::Serialize;
use tracing::info;

pub use macros::{Json, Toml};
pub use types::FileType;

pub trait Configurable: Serialize + Display {
    /// config dir
    fn config_dir(&self) -> PathBuf;
    /// config file type
    fn config_type(&self) -> FileType;
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
    /// if force = true, will remove exists config_path
    fn init(&self, force: bool) -> Result<()> {
        if self.config_path().exists() {
            if force {
                self.remove()?;
            } else {
                return Err(config_error!(
                    "config file {} already exists!",
                    self.config_path().display(),
                ));
            }
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
    /// remove config dir
    #[inline]
    fn remove_all(&self) -> Result<()> {
        fs::remove_dir_all(self.config_dir())?;
        Ok(())
    }
    /// remove config file
    #[inline]
    fn remove(&self) -> Result<()> {
        fs::remove_file(self.config_path())?;
        Ok(())
    }
}
