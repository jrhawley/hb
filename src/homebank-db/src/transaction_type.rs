//! The type of a `Transaction`

use crate::TransactionError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum TransactionType {
    Expense,
    Income,
    Transfer,
}

impl FromStr for TransactionType {
    type Err = TransactionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Expense" | "expense" | "E" | "e" => Ok(TransactionType::Expense),
            "Income" | "income" | "I" | "i" => Ok(TransactionType::Income),
            "Transfer" | "transfer" | "T" | "t" => Ok(TransactionType::Transfer),
            _ => Err(TransactionError::InvalidType),
        }
    }
}
