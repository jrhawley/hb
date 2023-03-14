//! Errors when parsing [`Account`][crate::account::account_struct::Account] information and HomeBank XML files.

use thiserror::Error;

/// Errors when parsing [`Account`][crate::account::account_struct::Account] information and HomeBank XML files.
#[derive(Debug, Error, PartialEq)]
pub enum AccountError {
    /// When an invalid key is provided.
    #[error("Invalid account key.")]
    InvalidKey,

    /// When a flag, or flags, are invalid.
    #[error("Invalid account flags.")]
    InvalidFlags,

    /// When an invalid dispaly position is provided.
    #[error("Invalid account position.")]
    InvalidPosition,

    /// When the [`Account`][crate::account::account_struct::Account] type doesn't map to an [`AccountType`][crate::account::account_type::AccountType].
    #[error("Invalid account type.")]
    InvalidType,

    /// When the currency index does not match a [`Currency`][crate::currency::currency_struct::Currency] in the database.
    #[error("Invalid account currency.")]
    InvalidCurrency,

    /// The name provided is an invalid `String`.
    #[error("Invalid account name.")]
    InvalidName,

    /// The initial amount provided does not properly parse to an `f32`.
    #[error("Invalid initial amount for account.")]
    InvalidInitialAmount,

    /// The minimum amount provided does not properly parse to an `f32`.
    #[error("Invalid account minimum amount.")]
    InvalidMinimumAmount,

    /// The maxiumim amount provided does not properly parse to an `f32`.
    #[error("Invalid account maximum amount.")]
    InvalidMaximumAmount,

    /// The notes describing an account are an ill-formed `String`.
    #[error("Invalid account notes.")]
    InvalidNotes,

    /// When the currency index does not match a [`Group`][crate::group::group_struct::Group] in the database.
    #[error("Invalid account group index.")]
    InvalidGroup,

    /// When the recondiled date provided cannot be properly parsed into a `NaiveDate`.
    #[error("Invalid account date.")]
    InvalidReconcileDate,
}
