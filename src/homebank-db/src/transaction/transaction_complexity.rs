//! Handle simple or split transactions.

use super::{SimpleTransaction, SplitTransaction};

#[derive(Debug, PartialEq, Clone)]
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

    /// Return the number of splits in the `Transaction`
    pub fn num_splits(&self) -> usize {
        match self {
            Self::Simple(_) => 0,
            Self::Split(split_tr) => split_tr.num_splits(),
        }
    }

    /// Retrieve the total for the `Transaction`
    pub fn total(&self) -> f32 {
        match self {
            Self::Simple(simple) => *simple.amount(),
            Self::Split(split) => split.total(),
        }
    }

    /// Retrieve the category(ies) for the `Transaction`
    pub fn categories(&self) -> Vec<&Option<usize>> {
        match self {
            Self::Simple(simple_tr) => vec![simple_tr.category()],
            Self::Split(split_tr) => split_tr.categories(),
        }
    }

    /// Retrieve the amount(s) for the `Transaction`
    pub fn amounts(&self) -> Vec<&f32> {
        match self {
            Self::Simple(simple_tr) => vec![simple_tr.amount()],
            Self::Split(split_tr) => split_tr.amounts(),
        }
    }

    /// Retrieve the memo(s) for the `Transaction`
    pub fn memos(&self) -> Vec<&Option<String>> {
        match self {
            Self::Simple(simple_tr) => vec![simple_tr.memo()],
            Self::Split(split_tr) => split_tr.memos(),
        }
    }

    /// Subset the `Transaction`.
    pub fn subset(&self, idx: &[usize]) -> Option<Self> {
        match (self, idx.len()) {
            (Self::Simple(simple), 1) => {
                if idx == &[0] {
                    Some(Self::Simple(simple.clone()))
                } else {
                    None
                }
            }
            (Self::Simple(_simple), _) => None,
            (Self::Split(split), _) => match split.subset(idx) {
                Some(sub_split) => Some(Self::Split(sub_split)),
                None => None,
            },
        }
    }
}

impl Default for TransactionComplexity {
    fn default() -> Self {
        TransactionComplexity::Simple(SimpleTransaction::default())
    }
}
