//! Errors when parsing [`Category`][crate::category::category::Category] information and HomeBank XML files.

use thiserror::Error;

/// Errors when parsing [`Category`][crate::category::category::Category] information and HomeBank XML files.
#[derive(Debug, Error, PartialEq)]
pub enum CategoryError {
    /// When the key for a category is not a number or not found in the database.
    #[error("Invalid category key.")]
    InvalidKey,

    /// When the flags for a category is not a number or not found in the database.
    #[error("Invalid category flags.")]
    InvalidFlags,
    
    /// When the name for a category cannot be properly parsed.
    #[error("Invalid category name.")]
    InvalidName,
    
    /// When the budget key for some month (`b1`, `b2`, ..., `b12`), or all months (`b0`), is invalid.
    #[error("Invalid category budget property.")]
    InvalidBudgetProperty,
    
    /// When the budget for some month, or all months, cannot be properly parsed as an `f32`.
    #[error("Invalid category budget value.")]
    InvalidBudgetValue,
    
    /// When the key for a category's parent is not a number or not found in the database.
    #[error("Invalid subcategory parent key.")]
    InvalidParentKey,
}
