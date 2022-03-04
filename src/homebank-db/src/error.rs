//! Handling errors of various kinds.

use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum HomeBankDbError {
    #[error("XHB file `{0}` does not exist.")]
    DoesNotExist(PathBuf),
    #[error("Error opening XHB file `{0}`.")]
    CouldNotOpen(PathBuf),
    #[error("Error reading XHB file `{0}`.")]
    CouldNotRead(PathBuf),
    #[error("Error parsing XHB file `{0}`.")]
    CouldNotParse(PathBuf),
}
