//! A [`Transaction`][crate::transaction::transaction_struct::Transaction] that is split across multiple [`Categories`][crate::category::category_struct::Category].

use crate::TransactionError;
use std::str::FromStr;
use xml::attribute::OwnedAttribute;

/// The string separator used to denote split transactions in the HomeBank XML file.
const SPLIT_SEPARATOR: &str = "||";

/// A [`Transaction`][crate::transaction::transaction_struct::Transaction] that is split across multiple [`Categories`][crate::category::category_struct::Category].
#[derive(Debug, PartialEq, Clone)]
pub struct SplitTransaction {
    /// The number of sub-transactions it is split into.
    /// This must be equal to `categories.len()`, `amounts.len()`, and `memos.len()`.
    num_splits: usize,

    /// The [`Categories`][crate::category::category_struct::Category] for the sub-transactions.
    categories: Vec<Option<usize>>,

    /// The amounts for each sub-transaction.
    amounts: Vec<f32>,

    /// The memos for each sub-transactions.
    memos: Vec<Option<String>>,
}

impl SplitTransaction {
    /// Create an empty [`SplitTransaction`].
    pub fn empty() -> Self {
        Self {
            num_splits: 0,
            categories: vec![],
            amounts: vec![],
            memos: vec![],
        }
    }

    /// Create a new [`SplitTransaction`].
    /// This assumes that `categories`, `amounts`, and `memos` all have length `num_splits`.
    pub fn new(
        num_splits: usize,
        categories: &Vec<Option<usize>>,
        amounts: &Vec<f32>,
        memos: &Vec<Option<String>>,
    ) -> Self {
        Self {
            num_splits,
            categories: categories.clone(),
            amounts: amounts.clone(),
            memos: memos.clone(),
        }
    }

    /// Retrieve the number of splits.
    pub fn num_splits(&self) -> usize {
        self.num_splits
    }

    /// Retrieve the mutable number of splits.
    pub fn mut_num_splits(&mut self) -> &mut usize {
        &mut self.num_splits
    }

    /// Retrieve the categories for the splits.
    pub fn categories(&self) -> Vec<&Option<usize>> {
        // using an iteration->collection trick to create the `Vec` on the fly
        // without duplicating the data inside that `Vec`
        self.categories.iter().collect()
    }

    /// Retrieve the mutable categories for the splits.
    pub fn mut_categories(&mut self) -> &mut Vec<Option<usize>> {
        &mut self.categories
    }

    /// Retrieve the total sum of the amounts.
    pub fn total(&self) -> f32 {
        self.amounts().iter().fold(0.0, |a, &b| a + b)
    }

    /// Retrieve the amounts for the splits.
    pub fn amounts(&self) -> Vec<&f32> {
        // using an iteration->collection trick to create the `Vec` on the fly
        // without duplicating the data inside that `Vec`
        self.amounts.iter().collect()
    }

    /// Retrieve the mutable amounts for the splits.
    pub fn mut_amounts(&mut self) -> &mut Vec<f32> {
        &mut self.amounts
    }

    /// Retrieve the memos for the splits.
    pub fn memos(&self) -> Vec<&Option<String>> {
        // using an iteration->collection trick to create the `Vec` on the fly
        // without duplicating the data inside that `Vec`
        self.memos.iter().collect()
    }

    /// Retrieve the mutable memos for the splits.
    pub fn mut_memos(&mut self) -> &mut Vec<Option<String>> {
        &mut self.memos
    }

    /// Subset the [`SplitTransaction`].
    pub fn subset(&self, idx: &[usize]) -> Option<Self> {
        let sub_num = idx.len();
        if sub_num == 0 {
            return None;
        }

        let sub_memos = idx
            .iter()
            .map(|&i| match self.memos()[i] {
                Some(possible_str) => Some(possible_str.to_string()),
                None => None,
            })
            .collect();
        let sub_categories = idx
            .iter()
            .map(|&i| match self.categories()[i] {
                Some(possible_cat) => Some(possible_cat.clone()),
                None => None,
            })
            .collect();
        let sub_amounts = idx.iter().map(|&i| self.amounts()[i].clone()).collect();

        Some(Self::new(
            sub_num,
            &sub_categories,
            &sub_amounts,
            &sub_memos,
        ))
    }
}

impl Default for SplitTransaction {
    fn default() -> Self {
        Self::empty()
    }
}

/// Parse the values stored in a split transaction or template.
pub fn parse_split_values(att: OwnedAttribute) -> Vec<String> {
    let vals = att
        .value
        .as_str()
        .split(SPLIT_SEPARATOR)
        .map(|s| s.to_string())
        .collect();

    vals
}

/// Convert `Vec<String>` into a parsed `Vec<Option<usize>>` to be used as categories.
pub fn parse_split_cat_vec(v: &Vec<String>) -> Result<Vec<Option<usize>>, TransactionError> {
    v.iter()
        // returning a `Result<>` within the iterator can be collected into a `Result<Vec<...>>`
        // see https://stackoverflow.com/a/26370894/7416009 for an example and other discussion
        .map(|s| match usize::from_str(s) {
            Ok(u) => Ok(Some(u)),
            Err(_) => Err(TransactionError::InvalidCategory(s.to_string())),
        })
        .collect()
}

/// Convert `Vec<String>` into a parsed `Vec<f32>` to be used as amounts.
pub fn parse_split_amount_vec(v: &Vec<String>) -> Result<Vec<f32>, TransactionError> {
    v.iter()
        // returning a `Result<>` within the iterator can be collected into a `Result<Vec<...>>`
        // see https://stackoverflow.com/a/26370894/7416009 for an example and other discussion
        .map(|s| match f32::from_str(s) {
            Ok(u) => Ok(u),
            Err(_) => Err(TransactionError::InvalidCategory(s.to_string())),
        })
        .collect()
}

/// Convert `Vec<String>` into a parsed `Vec<Option<String>>` to be used as memos.
pub fn parse_split_memo_vec(v: &Vec<String>) -> Vec<Option<String>> {
    v.iter()
        .map(|s| match s.as_str() {
            "" => None,
            s => Some(s.to_string()),
        })
        .collect()
}
