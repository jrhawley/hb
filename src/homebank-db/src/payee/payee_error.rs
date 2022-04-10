use thiserror::Error;

#[derive(Debug, Error)]
pub enum PayeeError {
    #[error("Invalid group key.")]
    InvalidKey,
    #[error("Invalid category key.")]
    InvalidCategoryKey,
    #[error("Invalid pay mode key.")]
    InvalidPayModeKey,
}
