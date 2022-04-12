//! The type of a `Transaction`

use super::Transfer;
use crate::TransactionError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum TransactionType {
    Expense,
    Income,
    Transfer(Transfer),
}

impl TransactionType {
    /// Determine if the `Transaction` is a transfer
    pub fn is_transfer(&self) -> bool {
        match self {
            TransactionType::Transfer(_) => true,
            _ => false,
        }
    }
}

impl FromStr for TransactionType {
    type Err = TransactionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Expense" | "expense" | "E" | "e" => Ok(TransactionType::Expense),
            "Income" | "income" | "I" | "i" => Ok(TransactionType::Income),
            "Transfer" | "transfer" | "T" | "t" => {
                Ok(TransactionType::Transfer(Transfer::default()))
            }
            _ => Err(TransactionError::InvalidType),
        }
    }
}
