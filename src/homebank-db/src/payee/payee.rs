//! The donor or recipient of a [`Transaction`][crate::transaction::transaction::Transaction].

use super::PayeeError;
use std::str::FromStr;
use xml::attribute::OwnedAttribute;

/// The donor or recipient of a [`Transaction`][crate::transaction::transaction::Transaction].
#[derive(Debug, PartialEq, Clone)]
pub struct Payee {
    /// Unique key for the payee in the database.
    key: usize,

    /// User-provided name of the payee.
    name: String,

    /// Default [`Category`][crate::category::category_struct::Category] that [`Transaction`s][crate::transaction::transaction::Transaction] involving this payee should belong to.
    default_category_key: Option<usize>,

    /// Default [`PayMode`][crate::paymode::paymode::PayMode] that [`Transaction`s][crate::transaction::transaction::Transaction] involving this payee should belong to.
    default_paymode_key: Option<usize>,
}

impl Payee {
    /// Create a new empty payee.
    pub fn empty() -> Self {
        Self {
            key: 0,
            name: "".to_string(),
            default_category_key: None,
            default_paymode_key: None,
        }
    }

    /// Create a new payee.
    pub fn new(key: usize, name: &str, category: Option<usize>, paymode: Option<usize>) -> Self {
        Self {
            key,
            name: name.to_string(),
            default_category_key: category,
            default_paymode_key: paymode,
        }
    }

    /// Retrieve the payee's key from the database.
    pub fn key(&self) -> usize {
        self.key
    }

    /// Retrieve the payee's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Retrieve the payee's default [`Category`][crate::category::category_struct::Category].
    pub fn category(&self) -> Option<usize> {
        self.default_category_key
    }

    /// Retrieve the payee's default [`PayMode`][crate::paymode::paymode::PayMode].
    pub fn paymode(&self) -> Option<usize> {
        self.default_paymode_key
    }
}

impl Default for Payee {
    fn default() -> Self {
        Self::empty()
    }
}

impl TryFrom<Vec<OwnedAttribute>> for Payee {
    type Error = PayeeError;

    fn try_from(v: Vec<OwnedAttribute>) -> Result<Self, Self::Error> {
        let mut payee = Self::default();

        for i in v {
            match i.name.local_name.as_str() {
                "key" => {
                    payee.key = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(PayeeError::InvalidKey),
                    }
                }
                "name" => {
                    payee.name = i.value.as_str().to_string();
                }
                "category" => {
                    payee.default_category_key = match usize::from_str(&i.value) {
                        Ok(idx) => Some(idx),
                        Err(_) => return Err(PayeeError::InvalidCategoryKey),
                    }
                }
                "paymode" => {
                    payee.default_paymode_key = match usize::from_str(&i.value) {
                        Ok(idx) => Some(idx),
                        Err(_) => return Err(PayeeError::InvalidPayModeKey),
                    }
                }
                _ => {}
            }
        }
        Ok(payee)
    }
}
