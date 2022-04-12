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

    /// Determine if two `Transaction`s are of a similar type.
    /// Useful for comparing if two `TransactionType`s are both `Transfer`s or
    /// not, without checking the values within the transfer.
    pub fn is_similar_to(&self, other: &Self) -> bool {
        match (self, other) {
            (TransactionType::Expense, TransactionType::Expense) => true,
            (TransactionType::Income, TransactionType::Income) => true,
            (TransactionType::Transfer(_this), TransactionType::Transfer(_other)) => true,
            _ => false,
        }
    }
}

impl Default for TransactionType {
    fn default() -> Self {
        Self::Expense
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
