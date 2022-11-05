//! Errors when parsing [`Group`s][crate::group::group::Group] from the HomeBank XML file.

use thiserror::Error;

/// Errors when parsing [`Group`s][crate::group::group::Group] from the HomeBank XML file.
#[derive(Debug, Error)]
pub enum GroupError {
    /// When the key for the group is an invalid number.
    #[error("Invalid group key.")]
    InvalidKey,
}
