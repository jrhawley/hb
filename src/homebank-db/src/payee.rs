//! Payees

use std::str::FromStr;
use thiserror::Error;
use xml::attribute::OwnedAttribute;

#[derive(Debug, Error)]
pub enum PayeeError {
    #[error("Invalid group key.")]
    InvalidKey,
    #[error("Invalid category key.")]
    InvalidCategoryKey,
    #[error("Invalid paymode key.")]
    InvalidPayModeKey,
}

#[derive(Debug, PartialEq)]
pub struct Payee {
    key: usize,
    name: String,
    category_idx: usize,
    paymode_idx: usize,
}

impl Payee {
    pub fn empty() -> Self {
        Self {
            key: 0,
            name: "".to_string(),
            category_idx: 0,
            paymode_idx: 0,
        }
    }

    pub fn new(key: usize, name: &str, category: usize, paymode: usize) -> Self {
        Self {
            key,
            name: name.to_string(),
            category_idx: category,
            paymode_idx: paymode,
        }
    }

    pub fn key(&self) -> usize {
        self.key
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn category(&self) -> usize {
        self.category_idx
    }

    pub fn paymode(&self) -> usize {
        self.paymode_idx
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
                    payee.key = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(PayeeError::InvalidCategoryKey),
                    }
                }
                "paymode" => {
                    payee.key = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(PayeeError::InvalidPayModeKey),
                    }
                }
                _ => {}
            }
        }
        Ok(payee)
    }
}
