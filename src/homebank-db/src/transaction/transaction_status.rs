//! Status of a [`Transaction`][crate::transaction::transaction_struct::Transaction].

use super::TransactionError;
use std::str::FromStr;

/// Status of a [`Transaction`][crate::transaction::transaction_struct::Transaction].
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TransactionStatus {
    None,
    Cleared,
    Reconciled,
    Remind,
    Void,
}

impl Default for TransactionStatus {
    fn default() -> Self {
        Self::None
    }
}

impl TryFrom<usize> for TransactionStatus {
    type Error = TransactionError;

    fn try_from(u: usize) -> Result<Self, Self::Error> {
        match u {
            0 => Ok(TransactionStatus::None),
            1 => Ok(TransactionStatus::Cleared),
            2 => Ok(TransactionStatus::Reconciled),
            3 => Ok(TransactionStatus::Remind),
            4 => Ok(TransactionStatus::Void),
            _ => Err(TransactionError::InvalidStatus),
        }
    }
}

impl FromStr for TransactionStatus {
    type Err = TransactionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" | "None" | "0" => Ok(TransactionStatus::None),
            "cleared" | "Cleared" | "1" => Ok(TransactionStatus::Cleared),
            "reconciled" | "Reconciled" | "2" => Ok(TransactionStatus::Reconciled),
            "remind" | "Remind" | "3" => Ok(TransactionStatus::Remind),
            "void" | "Void" | "4" => Ok(TransactionStatus::Void),
            _ => Err(TransactionError::InvalidStatus),
        }
    }
}
