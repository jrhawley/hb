use thiserror::Error;

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
