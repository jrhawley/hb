use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CategoryError {
    #[error("Invalid category key.")]
    InvalidKey,
    #[error("Invalid category flags.")]
    InvalidFlags,
    #[error("Invalid category name.")]
    InvalidName,
    #[error("Invalid category budget property.")]
    InvalidBudgetProperty,
    #[error("Invalid category budget value.")]
    InvalidBudgetValue,
    #[error("Invalid subcategory parent key.")]
    InvalidParentKey,
}
