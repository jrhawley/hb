//! The type of a [`Transaction`][crate::transaction::transaction_struct::Transaction].

use super::Transfer;
use crate::TransactionError;
use std::str::FromStr;

/// The type of a [`Transaction`][crate::transaction::transaction_struct::Transaction].
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TransactionType {
    /// An amount that is withdrawn from an [`Account`][crate::account::account_struct::Account].
    /// Also known as a "credit" in a [double-entry bookkeeping system](https://en.wikipedia.org/wiki/Double-entry_bookkeeping).
    Expense,

    /// An amount that is given to an [`Account`][crate::account::account_struct::Account].
    /// Also known as a "debit" in a [double-entry bookkeeping system](https://en.wikipedia.org/wiki/Double-entry_bookkeeping).
    Income,

    /// An `Expense` to one [`Account`][crate::account::account_struct::Account] and an `Income` to another, both of which are stored in the [`HomeBankDb`][crate::db::db_struct::HomeBankDb].
    Transfer(Transfer),
}

impl TransactionType {
    /// Determine if the [`Transaction`][crate::transaction::transaction_struct::Transaction] is a [`Transfer`][crate::transaction::transaction_transfer::Transfer].
    pub fn is_transfer(&self) -> bool {
        matches!(self, TransactionType::Transfer(_))
    }

    /// Determine if two [`Transaction`s][crate::transaction::transaction_struct::Transaction] are of a similar type.
    /// Useful for comparing if two [`TransactionType`]s are both [`Transfer`s][crate::transaction::transaction_transfer::Transfer] or
    /// not, without checking the values within the transfer.
    pub fn is_similar_to(&self, other: &Self) -> bool {
        matches!((self, other), (TransactionType::Expense, TransactionType::Expense) | (TransactionType::Income, TransactionType::Income) | (TransactionType::Transfer(_), TransactionType::Transfer(_)))
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
