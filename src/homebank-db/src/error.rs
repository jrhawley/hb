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

#[derive(Debug, Error, PartialEq)]
pub enum AccountError {
    #[error("Invalid account key.")]
    InvalidKey,
    #[error("Invalid account flags.")]
    InvalidFlags,
    #[error("Invalid account position.")]
    InvalidPosition,
    #[error("Invalid account type.")]
    InvalidType,
    #[error("Invalid account currency.")]
    InvalidCurrency,
    #[error("Invalid account name.")]
    InvalidName,
    #[error("Invalid initial amount for account.")]
    InvalidInitialAmount,
    #[error("Invalid account minimum amount.")]
    InvalidMinimumAmount,
    #[error("Invalid account maximum amount.")]
    InvalidMaximumAmount,
    #[error("Invalid account notes.")]
    InvalidNotes,
    #[error("Invalid account group index.")]
    InvalidGroup,
    #[error("Invalid account date.")]
    InvalidRDate,
}

#[derive(Debug, Error)]
pub enum CategoryError {
    #[error("Invalid category key.")]
    InvalidKey,
    #[error("Invalid category flags.")]
    InvalidFlags,
    #[error("Invalid category name.")]
    InvalidName,
    #[error("Invalid category 'b' properties.")]
    InvalidB,
    #[error("Invalid subcategory parent key.")]
    InvalidParentKey,
}

#[derive(Debug, Error)]
pub enum CurrencyError {
    #[error("Invalid currency key.")]
    InvalidKey,
    #[error("Invalid currency flags.")]
    InvalidFlags,
    #[error("Invalid currency ISO string.")]
    InvalidIsoString,
    #[error("Invalid currency name.")]
    InvalidName,
    #[error("Invalid currency symbol character.")]
    InvalidSymbol,
    #[error("Invalid currency syprf.")]
    InvalidSyprf,
    #[error("Invalid currency decimal separator character.")]
    InvalidDecimalSeparator,
    #[error("Invalid currency thousands separator character.")]
    InvalidThousandsSeparator,
    #[error("Invalid number of decimals to display.")]
    InvalidDecimalLength,
    #[error("Invalid currency conversion rate.")]
    InvalidConversionRate,
    #[error("Invalid currency mdate.")]
    InvalidMDate,
}
