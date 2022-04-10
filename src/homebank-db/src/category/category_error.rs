use thiserror::Error;

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
