//! Transactions

use super::{PayMode, TransactionStatus, TransactionType};
use chrono::{Duration, NaiveDate};
use std::str::FromStr;
use thiserror::Error;
use xml::attribute::OwnedAttribute;

#[derive(Debug, Error, PartialEq)]
pub enum TransactionError {
    #[error("Missing account from transaction.")]
    MissingAccount,
    #[error("Missing amount from transaction.")]
    MissingAmount,
    #[error("Missing date from transaction.")]
    MissingDate,
    #[error("Missing pay mode from transaction.")]
    MissingPayMode,
    #[error("Missing payee from transaction.")]
    MissingPayee,
    #[error("Invalid transaction status. Must be 0-4 or the status name 'None', 'Cleared', 'Reconciled', 'Remind', or 'Void'.")]
    InvalidStatus,
    #[error("Invalid transaction payment method. Must be 0-10, 'None', 'CreditCard', 'Cheque', 'Cash', 'BankTransfer', 'DebitCard', 'StandingOrder', 'ElectronicPayment', 'Deposit', 'FIFee', or 'DirectDebit'.")]
    InvalidPayMode,
    #[error("Invalid transaction type. Must be 'Expense', 'Income', or 'Transfer'.")]
    InvalidType,
}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    date: NaiveDate,
    amount: f32,
    account: usize,
    pay_mode: PayMode,
    status: TransactionStatus,
    flags: Option<usize>,
    payee: usize,
    category: Option<usize>,
    memo: Option<String>,
    info: Option<String>,
    tags: Option<Vec<String>>,
    transaction_type: TransactionType,
    // destination_account_idx: Option<usize>,
    // // I don't know what this is
    // kxfer: Option<usize>,
    // // I don't know what this is
    // scat: Option<String>,
    // // I don't know what this is
    // samt: Option<String>,
    // // I don't know what this is
    // smem: Option<String>,
}

impl Transaction {
    /// Retrieve the date of the `Transaction`
    pub fn date(&self) -> &NaiveDate {
        &self.date
    }

    /// Retrieve the amount of the `Transaction`
    pub fn amount(&self) -> &f32 {
        &self.amount
    }

    /// Retrieve the status of the `Transaction`
    pub fn status(&self) -> &TransactionStatus {
        &self.status
    }

    /// Retrieve the category of the `Transaction`
    pub fn category(&self) -> &Option<usize> {
        &self.category
    }

    /// Retrieve the payment method of the `Transaction`
    pub fn pay_mode(&self) -> &PayMode {
        &self.pay_mode
    }

    /// Retrieve the memo for the `Transaction`
    pub fn memo(&self) -> &Option<String> {
        &self.memo
    }

    /// Retrieve the info field for the `Transaction`
    pub fn info(&self) -> &Option<String> {
        &self.info
    }

    /// Retrieve the tags for the `Transaction`
    pub fn tags(&self) -> &Option<Vec<String>> {
        &self.tags
    }

    /// Retrieve the type for the `Transaction`
    pub fn ttype(&self) -> &TransactionType {
        &self.transaction_type
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            date: NaiveDate::from_ymd(2000, 1, 1),
            amount: 0.0,
            account: 0,
            pay_mode: PayMode::None,
            status: TransactionStatus::None,
            flags: None,
            payee: 0,
            category: None,
            memo: None,
            info: None,
            tags: None,
            transaction_type: TransactionType::Expense,
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
                    match f32::from_str(&i.value) {
                        Ok(a) => {
                            tr.amount = a;
                            // if the transaction already appears to be a transfer, then leave the type alone
                            // if it's not a transfer then it's an expense positive if the amount is negative, otherwise an income
                            if *tr.ttype() != TransactionType::Transfer {
                                if a > 0.0 {
                                    tr.transaction_type = TransactionType::Income;
                                } else {
                                    tr.transaction_type = TransactionType::Expense;
                                }
                            }
                        }
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
                    tr.pay_mode = match usize::from_str(&i.value) {
                        Ok(pm) => match PayMode::try_from(pm) {
                            Ok(t_pm) => t_pm,
                            Err(e) => return Err(e),
                        },
                        Err(_) => return Err(TransactionError::MissingPayMode),
                    }
                }
                "status" => {
                    tr.status = match usize::from_str(&i.value) {
                        Ok(st) => match TransactionStatus::try_from(st) {
                            Ok(t_stat) => t_stat,
                            Err(e) => return Err(e),
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
                "tags" => {
                    // split the tags string by commas
                    let tags: Vec<String> =
                        i.value.as_str().split(',').map(|s| s.to_string()).collect();
                    tr.tags = Some(tags);
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
    use xml::name::OwnedName;

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
        let observed = Transaction::try_from(input);

        assert_eq!(expected, observed);
    }

    /// Create a template `Vec<OwnedAttribute>` quickly for less repetition
    #[track_caller]
    fn template_vec_ownedatt() -> Vec<OwnedAttribute> {
        vec![
            OwnedAttribute {
                name: OwnedName {
                    local_name: "account".to_string(),
                    namespace: None,
                    prefix: None,
                },
                value: "1".to_string(),
            },
            OwnedAttribute {
                name: OwnedName {
                    local_name: "amount".to_string(),
                    namespace: None,
                    prefix: None,
                },
                value: "1".to_string(),
            },
            OwnedAttribute {
                name: OwnedName {
                    local_name: "date".to_string(),
                    namespace: None,
                    prefix: None,
                },
                // corresponds to 2020-03-11
                value: "737860".to_string(),
            },
            OwnedAttribute {
                name: OwnedName {
                    local_name: "payee".to_string(),
                    namespace: None,
                    prefix: None,
                },
                value: "1".to_string(),
            },
            OwnedAttribute {
                name: OwnedName {
                    local_name: "paymode".to_string(),
                    namespace: None,
                    prefix: None,
                },
                value: "0".to_string(),
            },
            OwnedAttribute {
                name: OwnedName {
                    local_name: "st".to_string(),
                    namespace: None,
                    prefix: None,
                },
                value: "0".to_string(),
            },
        ]
    }

    /// Create a template `Vec<OwnedAttribute>` that is missing one element
    #[track_caller]
    fn template_all_but(i: usize) -> Vec<OwnedAttribute> {
        template_vec_ownedatt()
            .iter()
            .enumerate()
            .filter(|&(j, _)| i != j)
            .map(|(_, v)| v.clone())
            .collect()
    }

    #[test]
    #[should_panic]
    fn try_from_empty() {
        let input = vec![];
        let expected = Err(TransactionError::MissingAccount);

        check_try_from_vec_ownedatt(input, expected)
    }

    #[test]
    #[should_panic]
    fn try_from_missing_acct() {
        // drop the account from the template
        let input = template_all_but(0);
        let expected = Err(TransactionError::MissingAccount);

        check_try_from_vec_ownedatt(input, expected)
    }

    #[test]
    #[should_panic]
    fn try_from_missing_amount() {
        // drop the account from the template
        let input = template_all_but(1);
        let expected = Err(TransactionError::MissingAmount);

        check_try_from_vec_ownedatt(input, expected)
    }

    #[test]
    #[should_panic]
    fn try_from_missing_date() {
        // drop the account from the template
        let input = template_all_but(2);
        let expected = Err(TransactionError::MissingDate);

        check_try_from_vec_ownedatt(input, expected)
    }

    #[test]
    #[should_panic]
    fn try_from_missing_paymode() {
        // drop the account from the template
        let input = template_all_but(3);
        let expected = Err(TransactionError::MissingPayMode);

        check_try_from_vec_ownedatt(input, expected)
    }

    #[test]
    #[should_panic]
    fn try_from_missing_payee() {
        // drop the account from the template
        let input = template_all_but(4);
        let expected = Err(TransactionError::MissingPayee);

        check_try_from_vec_ownedatt(input, expected)
    }

    #[test]
    #[should_panic]
    fn try_from_missing_status() {
        // drop the account from the template
        let input = template_all_but(5);
        let expected = Err(TransactionError::InvalidStatus);

        check_try_from_vec_ownedatt(input, expected)
    }

    #[test]
    fn try_from_template() {
        let input = template_vec_ownedatt();
        let expected = Ok(Transaction {
            account: 1,
            amount: 1.0,
            category: None,
            date: NaiveDate::from_ymd(2020, 03, 11),
            flags: None,
            info: None,
            memo: None,
            tags: None,
            pay_mode: PayMode::None,
            payee: 1,
            status: TransactionStatus::None,
            transaction_type: TransactionType::Income,
        });

        check_try_from_vec_ownedatt(input, expected)
    }
}
