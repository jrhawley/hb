//! A wrapper to provide a shared interface for [`SimpleTransaction`s][crate::transaction::transaction_simple::SimpleTransaction] and [`SplitTransaction`s][crate::transaction::transaction_split::SplitTransaction].

use super::{SimpleTransaction, SplitTransaction};

/// A wrapper to provide a shared interface for [`SimpleTransaction`s][crate::transaction::transaction_simple::SimpleTransaction] and [`SplitTransaction`s][crate::transaction::transaction_split::SplitTransaction].
#[derive(Debug, PartialEq, Clone)]
pub enum TransactionComplexity {
    Simple(SimpleTransaction),
    Split(SplitTransaction),
}

impl TransactionComplexity {
    /// Check if the [`Transaction`][crate::transaction::transaction::Transaction] is [`Simple`][crate::transaction::transaction_simple::SimpleTransaction] or [`Split`][crate::transaction::transaction_split::SplitTransaction].
    pub fn is_split(&self) -> bool {
        match self {
            Self::Split(_) => true,
            _ => false,
        }
    }

    /// Check if two [`Transaction`s][crate::transaction::transaction::Transaction] are both [`Simple`][crate::transaction::transaction_simple::SimpleTransaction] or both [`Split`][crate::transaction::transaction_split::SplitTransaction].
    pub fn is_similar_to(&self, other: &Self) -> bool {
        self.is_split() == other.is_split()
    }

    /// Return the number of sub-transactions in a [`Transaction`][crate::transaction::transaction::Transaction].
    /// 
    /// A [`SimpleTransaction`][crate::transaction::transaction_simple::SimpleTransaction] will return `0`.
    /// A [`SplitTransaction`][crate::transaction::transaction_split::SplitTransaction] will return the number of sub-transactions is has.
    /// This may be `1`, if the [`Transaction`][crate::transaction::transaction::Transaction] has been filtered or transformed from its original value.
    pub fn num_splits(&self) -> usize {
        match self {
            Self::Simple(_) => 0,
            Self::Split(split_tr) => split_tr.num_splits(),
        }
    }

    /// Retrieve the total for the [`Transaction`][crate::transaction::transaction::Transaction].
    /// This is simply the amount of a [`SimpleTransaction`][crate::transaction::transaction_simple::SimpleTransaction] or the sum of all amounts in a [`SplitTransaction`][crate::transaction::transaction_split::SplitTransaction].
    pub fn total(&self) -> f32 {
        match self {
            Self::Simple(simple) => *simple.amount(),
            Self::Split(split) => split.total(),
        }
    }

    /// Retrieve the category(ies) for the [`Transaction`][crate::transaction::transaction::Transaction].
    pub fn categories(&self) -> Vec<&Option<usize>> {
        match self {
            Self::Simple(simple_tr) => vec![simple_tr.category()],
            Self::Split(split_tr) => split_tr.categories(),
        }
    }

    /// Retrieve the amount(s) for the [`Transaction`][crate::transaction::transaction::Transaction].
    pub fn amounts(&self) -> Vec<&f32> {
        match self {
            Self::Simple(simple_tr) => vec![simple_tr.amount()],
            Self::Split(split_tr) => split_tr.amounts(),
        }
    }

    /// Retrieve the memo(s) for the [`Transaction`][crate::transaction::transaction::Transaction].
    pub fn memos(&self) -> Vec<&Option<String>> {
        match self {
            Self::Simple(simple_tr) => vec![simple_tr.memo()],
            Self::Split(split_tr) => split_tr.memos(),
        }
    }

    /// Subset the [`Transaction`][crate::transaction::transaction::Transaction].
    /// A [`SimpleTransaction`][crate::transaction::transaction_simple::SimpleTransaction] with any single `idx` will just return itself.
    /// If there are multiple indices, then this function will return `None`.
    /// A [`SplitTransaction`][crate::transaction::transaction_split::SplitTransaction] will be subset if possible.
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
