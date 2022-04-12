//! Handle simple or split transactions.

use super::{SimpleTransaction, SplitTransaction};

#[derive(Debug, PartialEq)]
pub enum TransactionComplexity {
    Simple(SimpleTransaction),
    Split(SplitTransaction),
}

impl TransactionComplexity {
    /// Check if the `Transaction` is 'Simple' or 'Split'
    pub fn is_split(&self) -> bool {
        match self {
            Self::Split(_) => true,
            _ => false,
        }
    }

    /// Check if two `Transaction`s are of a similar complexity
    pub fn is_similar_to(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Simple(_this), Self::Simple(_other)) => true,
            (Self::Split(_this), Self::Split(_other)) => true,
            (_, _) => false,
        }
    }
}
