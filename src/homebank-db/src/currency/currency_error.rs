use thiserror::Error;

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
