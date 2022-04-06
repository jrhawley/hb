//! Transactions

use super::TransactionStatus;
use chrono::{Duration, NaiveDate};
use std::str::FromStr;
use thiserror::Error;
use xml::attribute::OwnedAttribute;

#[derive(Debug, Error, PartialEq)]
pub enum TransactionError {
    #[error("Missing date from transaction.")]
    MissingDate,
    #[error("Missing amount from transaction.")]
    MissingAmount,
    #[error("Missing account from transaction.")]
    MissingAccount,
    #[error("Missing pay mode from transaction.")]
    MissingPayMode,
    #[error("Missing payee from transaction.")]
    MissingPayee,
    #[error("Invalid transaction status. Must be 0-4 or the status name.")]
    InvalidStatus,
}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    date: NaiveDate,
    amount: f32,
    account: usize,
    paymode: usize,
    status: TransactionStatus,
    flags: Option<usize>,
    payee: usize,
    category: Option<usize>,
    memo: Option<String>,
    info: Option<String>,
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            date: NaiveDate::from_ymd(2000, 1, 1),
            amount: 0.0,
            account: 0,
            paymode: 0,
            status: TransactionStatus::None,
            flags: None,
            payee: 0,
            category: None,
            memo: None,
            info: None,
        }
    }
}

impl TryFrom<Vec<OwnedAttribute>> for Transaction {
    type Error = TransactionError;

    fn try_from(v: Vec<OwnedAttribute>) -> Result<Self, Self::Error> {
        let mut tr = Self::default();

        for i in v {
            match i.name.local_name.as_str() {
                "account" => {
                    tr.account = match usize::from_str(&i.value) {
                        Ok(a) => a,
                        Err(_) => return Err(TransactionError::MissingAccount),
                    }
                }
                "amount" => {
                    tr.amount = match f32::from_str(&i.value) {
                        Ok(a) => a,
                        Err(_) => return Err(TransactionError::MissingAmount),
                    };
                }
                "category" => {
                    tr.category = match usize::from_str(&i.value) {
                        Ok(c) => Some(c),
                        Err(_) => None,
                    }
                }
                "date" => {
                    tr.date = match u32::from_str(&i.value) {
                        Ok(d) => {
                            // dates are stored as Julian dates
                            let zero = NaiveDate::from_ymd(0, 1, 1);
                            zero + Duration::days(d.into())
                        }
                        Err(_) => return Err(TransactionError::MissingDate),
                    }
                }
                "paymode" => {
                    tr.paymode = match usize::from_str(&i.value) {
                        Ok(pm) => pm,
                        Err(_) => return Err(TransactionError::MissingPayMode),
                    }
                }
                "status" => {
                    tr.status = match usize::from_str(&i.value) {
                        Ok(st) => match TransactionStatus::try_from(st) {
                            Ok(t_stat) => t_stat,
                            Err(e) => return Err(TransactionError::InvalidStatus),
                        },
                        Err(_) => TransactionStatus::None,
                    }
                }
                "flags" => {
                    tr.flags = match usize::from_str(&i.value) {
                        Ok(f) => Some(f),
                        Err(_) => None,
                    }
                }
                "payee" => {
                    tr.payee = match usize::from_str(&i.value) {
                        Ok(p) => p,
                        Err(_) => return Err(TransactionError::MissingPayee),
                    }
                }
                "wording" => {
                    tr.memo = match i.value.as_str() {
                        "" => None,
                        s => Some(s.to_string()),
                    }
                }
                _ => {}
            }
        }
        Ok(tr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(2 + 2, result);
    }

    #[track_caller]
    fn check_try_from_vec_ownedatt(
        input: Vec<OwnedAttribute>,
        expected: Result<Transaction, TransactionError>,
    ) {
        println!("{:#?}", input);
        let observed = Transaction::try_from(input);

        assert_eq!(expected, observed);
    }

    #[test]
    fn try_from_empty() {}
}
