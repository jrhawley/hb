//! Handling errors of various kinds.

use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HomeBankDbError {
    #[error("XHB file `{0}` does not exist.")]
    DoesNotExist(PathBuf),
}
