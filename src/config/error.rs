//! Errors when parsing the configuration file

use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Configuration file `{0}` does not exist.")]
    DoesNotExist(PathBuf),
    #[error("Configuration file `{0}` is not a file.")]
    NotAFile(PathBuf),
    #[error("Configuration file is missing a `path` variable.")]
    MissingHomeBankPath,
    #[error("Error parsing configuration file `{0}`.")]
    ParseError(PathBuf),
    #[error("HomeBank file `{0}` does not exist.")]
    HomeBankFileDoesNotExist(PathBuf),
}
