//! Errors when parsing or loading a HomeBank database XML file.

use std::path::PathBuf;
use thiserror::Error;

/// Errors when parsing or loading a HomeBank database XML file.
#[derive(Debug, Error, PartialEq)]
pub enum HomeBankDbError {
    /// The database file does not exist or cannot be found.
    #[error("XHB file `{0}` does not exist.")]
    DoesNotExist(PathBuf),

    /// The database file is found, but there is an error when opening it.
    #[error("Error opening XHB file `{0}`.")]
    CouldNotOpen(PathBuf),

    /// The database file is found and was opened, but there is an error when reading from it.
    #[error("Error reading XHB file `{0}`.")]
    CouldNotRead(PathBuf),

    /// The database file is found, opened, and read from, but its contents cannot be parsed.
    #[error("Error parsing XHB file `{0}`.")]
    CouldNotParse(PathBuf),

    /// The last saved date of the database cannot be converted to a `NaiveDate` type.
    #[error("Invalid database date.")]
    InvalidDate,

    /// The database schema version cannot be properly parsed.
    #[error("Invalid database version.")]
    InvalidVersion,
}
