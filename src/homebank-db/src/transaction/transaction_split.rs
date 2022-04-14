use crate::TransactionError;
use std::str::FromStr;
use xml::attribute::OwnedAttribute;

/// The string separator used to denote split transactions
const SPLIT_SEPARATOR: &str = "||";

#[derive(Debug, PartialEq, Clone)]
pub struct SplitTransaction {
    /// How many sub-transactions is it split into
    num_splits: usize,
    /// What are the categories for the sub-transactions
    categories: Vec<Option<usize>>,
    /// What are the amounts for the sub-transactions
    amounts: Vec<f32>,
    /// What are the memos for the sub-transactions
    memos: Vec<Option<String>>,
}

impl SplitTransaction {
    /// Create an empty `SplitTransaction`
    pub fn empty() -> Self {
        Self {
            num_splits: 0,
            categories: vec![],
            amounts: vec![],
            memos: vec![],
        }
    }

    /// Create an empty `SplitTransaction`
    pub fn new(
        num_splits: usize,
        categories: Vec<Option<usize>>,
        amounts: Vec<f32>,
        memos: Vec<Option<String>>,
    ) -> Self {
        Self {
            num_splits,
            categories,
            amounts,
            memos,
        }
    }

    /// Retrieve the number of splits
    pub fn num_splits(&self) -> usize {
        self.num_splits
    }

    /// Retrieve the mutable number of splits
    pub fn mut_num_splits(&mut self) -> &mut usize {
        &mut self.num_splits
    }

    /// Retrieve the categories for the splits
    pub fn categories(&self) -> Vec<&Option<usize>> {
        // using an iteration->collection trick to create the `Vec` on the fly
        // without duplicating the data inside that `Vec`
        self.categories.iter().collect()
    }

    /// Retrieve the mutable categories for the splits
    pub fn mut_categories(&mut self) -> &mut Vec<Option<usize>> {
        &mut self.categories
    }

    /// Retrieve the amounts for the splits
    pub fn amounts(&self) -> Vec<&f32> {
        // using an iteration->collection trick to create the `Vec` on the fly
        // without duplicating the data inside that `Vec`
        self.amounts.iter().collect()
    }

    /// Retrieve the mutable amounts for the splits
    pub fn mut_amounts(&mut self) -> &mut Vec<f32> {
        &mut self.amounts
    }

    /// Retrieve the memos for the splits
    pub fn memos(&self) -> Vec<&Option<String>> {
        // using an iteration->collection trick to create the `Vec` on the fly
        // without duplicating the data inside that `Vec`
        self.memos.iter().collect()
    }

    /// Retrieve the mutable memos for the splits
    pub fn mut_memos(&mut self) -> &mut Vec<Option<String>> {
        &mut self.memos
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

/// Convert `Vec<String>` into a parsed `Vec<Option<usize>>` to be used as categories
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

/// Convert `Vec<String>` into a parsed `Vec<f32>` to be used as amounts
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

/// Convert `Vec<String>` into a parsed `Vec<Option<String>>` to be used as memos
pub fn parse_split_memo_vec(v: &Vec<String>) -> Vec<Option<String>> {
    v.iter()
        .map(|s| match s.as_str() {
            "" => None,
            s => Some(s.to_string()),
        })
        .collect()
}
