//! Chequing accounts, credits cards, and details for all kinds of accounts.

use super::{AccountError, AccountType};
use crate::transaction::julian_date_from_u32;
use chrono::NaiveDate;
use std::str::FromStr;
use xml::attribute::OwnedAttribute;

/// Chequing accounts, credits cards, and details for all kinds of accounts.
#[derive(Debug, PartialEq, Clone)]
pub struct Account {
    /// Unique key for this account.
    key: usize,

    /// Flags on this account.
    flags: usize,

    /// Display position.
    pos: usize,

    /// What type of account this is.
    atype: AccountType,

    /// Index of currency used for transactions in this account.
    currency_idx: usize,

    /// Account name.
    name: String,

    /// Institution where the account is managed.
    bank_name: String,

    /// Initial starting amount.
    initial_amount: f32,

    /// Overdraft amount.
    minimum_amount: f32,

    /// Maximum total amount.
    maximum_amount: f32,

    /// User-provided notes.
    notes: String,

    /// Index of the group this account belongs to, if any.
    group_idx: Option<usize>,

    /// Last reconciled date for [`Transaction`s][crate::transaction::transaction::Transaction] associated with this account.
    reconciled_date: NaiveDate,
}

impl Account {
    pub fn empty() -> Self {
        Self {
            key: 0,
            flags: 0,
            pos: 0,
            atype: AccountType::None,
            currency_idx: 0,
            name: "".to_string(),
            bank_name: "".to_string(),
            initial_amount: 0.0,
            minimum_amount: 0.0,
            maximum_amount: 0.0,
            notes: "".to_string(),
            group_idx: Some(0),
            reconciled_date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
        }
    }

    /// Retrieve the `Account` key
    pub(crate) fn key(&self) -> usize {
        self.key
    }

    /// Retrieve the account name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Retrieve the account type
    pub fn atype(&self) -> &AccountType {
        &self.atype
    }

    /// Retrieve the account's group index
    pub fn group(&self) -> Option<usize> {
        self.group_idx
    }

    /// Retrieve the name of the account's financial institution
    pub fn institution(&self) -> &str {
        &self.bank_name
    }
}

impl Default for Account {
    fn default() -> Self {
        Self::empty()
    }
}

impl TryFrom<Vec<OwnedAttribute>> for Account {
    type Error = AccountError;

    fn try_from(v: Vec<OwnedAttribute>) -> Result<Self, Self::Error> {
        let mut acct = Self::default();

        for i in v {
            match i.name.local_name.as_str() {
                "name" => {
                    acct.name = i.value.to_string();
                }
                "bankname" => {
                    acct.bank_name = i.value.to_string();
                }
                "notes" => {
                    acct.notes = i.value.to_string();
                }
                "key" => {
                    acct.key = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(AccountError::InvalidKey),
                    }
                }
                "flags" => {
                    acct.flags = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(AccountError::InvalidFlags),
                    }
                }
                "pos" => {
                    acct.pos = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(AccountError::InvalidPosition),
                    }
                }
                "type" => {
                    acct.atype = match usize::from_str(&i.value) {
                        Ok(idx) => match AccountType::try_from(idx) {
                            Ok(atype) => atype,
                            Err(e) => return Err(e),
                        },
                        Err(_) => return Err(AccountError::InvalidType),
                    }
                }
                "curr" => {
                    acct.currency_idx = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(AccountError::InvalidCurrency),
                    }
                }
                "initial" => {
                    acct.initial_amount = match f32::from_str(&i.value) {
                        Ok(a) => a,
                        Err(_) => return Err(AccountError::InvalidInitialAmount),
                    }
                }
                "minimum" => {
                    acct.minimum_amount = match f32::from_str(&i.value) {
                        Ok(a) => a,
                        Err(_) => return Err(AccountError::InvalidMinimumAmount),
                    }
                }
                "maximum" => {
                    acct.maximum_amount = match f32::from_str(&i.value) {
                        Ok(a) => a,
                        Err(_) => return Err(AccountError::InvalidMaximumAmount),
                    }
                }
                "grp" => {
                    acct.group_idx = match usize::from_str(&i.value) {
                        Ok(idx) => Some(idx),
                        Err(_) => return Err(AccountError::InvalidGroup),
                    }
                }
                "rdate" => {
                    acct.reconciled_date = match u32::from_str(&i.value) {
                        Ok(d) => julian_date_from_u32(d),
                        Err(_) => return Err(AccountError::InvalidReconcileDate),
                    }
                }
                _ => {}
            }
        }
        Ok(acct)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(2 + 2, result);
    }
}