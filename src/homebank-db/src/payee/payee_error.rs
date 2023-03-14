//! Errors when parsing [`Payee`s][crate::payee::payee::Payee] from the [`HomeBankDb`][crate::db::db_struct::HomeBankDb].

use thiserror::Error;

/// Errors when parsing [`Payee`s][crate::payee::payee::Payee] from the [`HomeBankDb`][crate::db::db_struct::HomeBankDb].
#[derive(Debug, Error)]
pub enum PayeeError {
    /// When the key for the payee is invalid.
    #[error("Invalid payee key.")]
    InvalidKey,
    
    /// When the key for the payee's default category is invalid or not found in the database.
    #[error("Invalid default category key.")]
    InvalidCategoryKey,

    /// When the key for the payee's default pay mode is invalid or not found in the database.
    #[error("Invalid default pay mode key.")]
    InvalidPayModeKey,
}
