//! Errors encountered when parsing or formatting [`Currencies`][crate::currency::currency::Currency].

use thiserror::Error;

/// Errors encountered when parsing or formatting [`Currencies`][crate::currency::currency::Currency].
#[derive(Debug, Error)]
pub enum CurrencyError {
    /// When the key for a [`Currency`][crate::currency::currency::Currency] is not a number or not found in the database.
    #[error("Invalid currency key.")]
    InvalidKey,

    /// When the flags on a [`Currency`][crate::currency::currency::Currency] are invalid.
    #[error("Invalid currency flags.")]
    InvalidFlags,

    /// When the ISO code for a [`Currency`][crate::currency::currency::Currency] is an ill-formed string.
    #[error("Invalid currency ISO string.")]
    InvalidIsoString,

    /// When the name for a [`Currency`][crate::currency::currency::Currency] is an ill-formed string.
    #[error("Invalid currency name.")]
    InvalidName,

    /// When the symbol for a [`Currency`][crate::currency::currency::Currency] is an ill-formed character.
    #[error("Invalid currency symbol character.")]
    InvalidSymbol,

    /// When the name for a [`Currency`][crate::currency::currency::Currency] is an ill-formed string.
    #[error("Invalid currency symbol prefix.")]
    InvalidSymbolPrefix,
    
    /// When the decimal separator for a [`Currency`][crate::currency::currency::Currency] is an ill-formed character.
    #[error("Invalid currency decimal separator character.")]
    InvalidDecimalSeparator,

    /// When the thousands separator for a [`Currency`][crate::currency::currency::Currency] is an ill-formed character.
    #[error("Invalid currency thousands separator character.")]
    InvalidThousandsSeparator,
    
    /// When the number of decimals to display for a [`Currency`][crate::currency::currency::Currency] is an invalid whole number.
    #[error("Invalid number of decimals to display.")]
    InvalidDecimalLength,

    /// When the conversion rate for a [`Currency`][crate::currency::currency::Currency] is not a properly parsed `f32`.
    #[error("Invalid currency conversion rate.")]
    InvalidConversionRate,

    /// When the date provided cannot be properly parsed into a `NaiveDate`.
    #[error("Invalid currency mdate.")]
    InvalidMDate,
}
