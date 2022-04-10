//! Accounts

use chrono::{Duration, NaiveDate};
use std::str::FromStr;
use xml::attribute::OwnedAttribute;

use super::{AccountError, AccountType};

#[derive(Debug, PartialEq)]
pub struct Account {
    key: usize,
    flags: usize,
    pos: usize,
    atype: AccountType,
    currency_idx: usize,
    name: String,
    bank_name: String,
    initial_amount: f32,
    minimum_amount: f32,
    maximum_amount: f32,
    notes: String,
    group_idx: usize,
    rdate: NaiveDate,
}

impl Account {
    pub(crate) fn empty() -> Self {
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
            group_idx: 0,
            rdate: NaiveDate::from_ymd(2000, 01, 01),
        }
    }

    pub(crate) fn new(
        key: usize,
        flags: usize,
        pos: usize,
        atype: AccountType,
        currency_idx: usize,
        name: &str,
        bank_name: &str,
        init: f32,
        min: f32,
        max: f32,
        notes: &str,
        group_idx: usize,
        rdate: &NaiveDate,
    ) -> Self {
        Self {
            key,
            flags,
            pos,
            atype,
            currency_idx,
            name: name.to_string(),
            bank_name: bank_name.to_string(),
            initial_amount: init,
            minimum_amount: min,
            maximum_amount: max,
            notes: notes.to_string(),
            group_idx,
            rdate: rdate.clone(),
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
    pub fn group(&self) -> &usize {
        &self.group_idx
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
                        Ok(idx) => idx,
                        Err(_) => return Err(AccountError::InvalidGroup),
                    };
                }
                "rdate" => {
                    acct.rdate = match u32::from_str(&i.value) {
                        Ok(d) => {
                            // dates are stored as Julian dates
                            let zero = NaiveDate::from_ymd(0, 1, 1);
                            zero + Duration::days(d.into())
                        }
                        Err(_) => return Err(AccountError::InvalidRDate),
                    }
                }
                _ => {}
            }
        }
        Ok(acct)
    }
}
