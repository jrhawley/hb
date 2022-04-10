//! Transactions

use super::{julian_date_from_u32, parse_split_values, TransactionStatus, TransactionType};
use crate::{HomeBankDb, PayMode, TransactionError};
use chrono::NaiveDate;
use std::str::FromStr;
use xml::attribute::OwnedAttribute;

#[derive(Debug, PartialEq)]
pub struct Transaction {
    /// Date on which the transaction took place
    date: NaiveDate,
    /// Net sum of the transaction (including any split amounts)
    amount: f32,
    /// Which account the transaction applied to
    account: usize,
    /// Payment method transacted
    pay_mode: PayMode,
    /// Review status of the transaction
    status: TransactionStatus,
    /// Any flags
    flags: Option<usize>,
    /// Which payee was involved with the transaction
    payee: Option<usize>,
    /// Which category does this transaction fall under
    category: Option<usize>,
    /// Short form text expanding on what the transaction was about
    memo: Option<String>,
    /// Any info related to the transaction, such as a reference number
    info: Option<String>,
    /// Tags for the transaction
    tags: Option<Vec<String>>,
    /// What type of transaction was it? 'Expense', 'Income', or 'Transfer'?
    transaction_type: TransactionType,
    /// If this is a transfer, what is the corresponding destination account
    destination_account_idx: Option<usize>,
    /// If this is a transfer, unique identifier for the transfer
    transfer_key: Option<usize>,
    /// If this transaction is split, how many sub-transactions is it split into
    num_splits: usize,
    /// If this transaction is split, what are the categories for the sub-transactions
    split_categories: Option<Vec<usize>>,
    /// If this transaction is split, what are the amounts for the sub-transactions
    split_amounts: Option<Vec<f32>>,
    /// If this transaction is split, what are the memos for the sub-transactions
    split_memos: Option<Vec<String>>,
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

    /// Retrieve the account where the `Transaction` takes place
    pub fn account(&self) -> usize {
        self.account
    }

    /// Retrieve the account name
    pub fn account_name(&self, db: &HomeBankDb) -> Option<String> {
        if let Some(acct) = db.accounts().get(&self.account()) {
            Some(acct.name().to_string())
        } else {
            None
        }
    }

    /// Retrieve the status of the `Transaction`
    pub fn status(&self) -> &TransactionStatus {
        &self.status
    }

    /// Retrieve the category of the `Transaction`
    pub fn category(&self) -> &Option<usize> {
        &self.category
    }

    /// Retrieve the complete category name.
    /// This includes the parent category, if one exists.
    pub fn category_name(&self, db: &HomeBankDb) -> Option<String> {
        match self.category() {
            Some(idx) => {
                if let Some(cat) = db.categories().get(idx) {
                    Some(cat.full_name(db))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Retrieve the payee for the `Transaction`
    pub fn payee(&self) -> &Option<usize> {
        &self.payee
    }

    /// Retrieve the payee name.
    pub fn payee_name(&self, db: &HomeBankDb) -> Option<String> {
        match self.payee() {
            Some(idx) => {
                if let Some(payee) = db.payees().get(idx) {
                    Some(payee.name().to_string())
                } else {
                    None
                }
            }
            None => None,
        }
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

    /// Check if the `Transaction` is a transfer or not
    pub fn is_transfer(&self) -> bool {
        self.transfer_key.is_some()
    }

    /// Retrieve the transfer key for the `Transaction`
    pub fn transfer_key(&self) -> &Option<usize> {
        &self.transfer_key
    }

    /// Retrieve the destination account key for the transfer
    pub fn transfer_destination_account_key(&self) -> &Option<usize> {
        &self.destination_account_idx
    }

    /// Check if the `Transaction` is a split transaction or not
    pub fn is_split(&self) -> bool {
        self.num_splits > 0
    }

    /// Retrieve the number of splits the `Transaction` is divided into
    pub fn num_splits(&self) -> usize {
        self.num_splits
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
            payee: None,
            category: None,
            memo: None,
            info: None,
            tags: None,
            transaction_type: TransactionType::Expense,
            destination_account_idx: None,
            transfer_key: None,
            num_splits: 0,
            split_amounts: None,
            split_categories: None,
            split_memos: None,
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
                            // if it's not a transfer then it's an expense if the amount is negative, otherwise an income
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
                        Err(_) => {
                            return Err(TransactionError::InvalidCategory(i.value.to_string()))
                        }
                    }
                }
                "date" => {
                    tr.date = match u32::from_str(&i.value) {
                        Ok(d) => julian_date_from_u32(d),
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
                "st" => {
                    tr.status = match usize::from_str(&i.value) {
                        Ok(st) => match TransactionStatus::try_from(st) {
                            Ok(t_stat) => t_stat,
                            Err(e) => return Err(e),
                        },
                        Err(_) => return Err(TransactionError::InvalidStatus),
                    }
                }
                "flags" => {
                    tr.flags = match usize::from_str(&i.value) {
                        Ok(f) => Some(f),
                        Err(_) => return Err(TransactionError::InvalidFlags),
                    }
                }
                "payee" => {
                    tr.payee = match usize::from_str(&i.value) {
                        Ok(p) => Some(p),
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
                // handle split categories
                "scat" => {}
                // handle split amounts
                "samt" => {}
                // handle split memos
                "smem" => {}
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
                value: "737494".to_string(),
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
            payee: Some(1),
            status: TransactionStatus::None,
            transaction_type: TransactionType::Income,
            destination_account_idx: None,
            transfer_key: None,
            num_splits: 0,
            split_amounts: None,
            split_categories: None,
            split_memos: None,
        });

        check_try_from_vec_ownedatt(input, expected)
    }

    #[track_caller]
    fn check_try_from_single_str(input: &str, expected: Result<Transaction, TransactionError>) {
        // set up the reader from the input string
        let mut reader = EventReader::from_str(input);

        // skip the XML starting header and parse the first event
        let (_start, first) = (reader.next(), reader.next());

        // get the first event
        if let Ok(XmlEvent::StartElement {
            name, attributes, ..
        }) = first
        {
            if "ope" == name.local_name.as_str() {
                let observed = Transaction::try_from(attributes);
                assert_eq!(expected, observed);
            } else {
                panic!(
                    "Incorrect transaction string passed into check. Expected `ope`, found `{:#?}`",
                    name.local_name.as_str()
                );
            }
        } else {
            panic!("Incorrect string passed into check. `{:#?}`", first);
        }
    }

    #[test]
    fn parse_account() {
        let input = r#"<ope account="1">"#;
        let expected = Ok(Transaction {
            account: 1,
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }

    #[test]
    fn parse_positive_amount() {
        let input = r#"<ope amount="1">"#;
        let expected = Ok(Transaction {
            amount: 1.0,
            transaction_type: TransactionType::Income,
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }

    #[test]
    fn parse_negative_amount() {
        let input = r#"<ope amount="-1">"#;
        let expected = Ok(Transaction {
            amount: -1.0,
            transaction_type: TransactionType::Expense,
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }
    #[test]
    fn parse_good_category() {
        let input = r#"<ope category="1">"#;
        let expected = Ok(Transaction {
            category: Some(1),
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }

    #[test]
    #[should_panic]
    fn parse_bad_category() {
        let input = r#"<ope category="-1">"#;
        let expected = Ok(Transaction {
            category: Some(1),
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }

    #[test]
    fn parse_good_date() {
        let input = r#"<ope date="737494">"#;
        let expected = Ok(Transaction {
            date: NaiveDate::from_ymd(2020, 03, 11),
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }

    #[test]
    #[should_panic]
    fn parse_bad_date() {
        let input = r#"<ope category="5.028s">"#;
        let expected = Ok(Transaction {
            category: Some(1),
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }

    /// Check all valid pay modes at the same time
    #[test]
    fn parse_paymode_good() {
        let inputs = vec![
            PayMode::None,
            PayMode::CreditCard,
            PayMode::Cheque,
            PayMode::Cash,
            PayMode::BankTransfer,
            PayMode::DebitCard,
            PayMode::StandingOrder,
            PayMode::ElectronicPayment,
            PayMode::Deposit,
            PayMode::FinancialInstitutionFee,
            PayMode::DirectDebit,
        ];

        // iterate over the pay modes
        for (i, pay_mode) in inputs.into_iter().enumerate() {
            // fill in the raw string with the index that matches the pay mode
            let input = format!(r#"<ope paymode="{}">"#, i);
            let expected = Ok(Transaction {
                pay_mode,
                ..Default::default()
            });
            // perform the check
            check_try_from_single_str(&input, expected);
        }
    }

    #[test]
    #[should_panic]
    fn parse_paymode_bad() {
        // use a string that should work in the `from_str` method to make sure that there
        // isn't confusion between the two parsing methods
        let input = r#"<ope paymode="none">"#;
        let expected = Ok(Transaction {
            pay_mode: PayMode::None,
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }

    /// Check all valid pay modes at the same time
    #[test]
    fn parse_status_good() {
        let inputs = vec![
            TransactionStatus::None,
            TransactionStatus::Cleared,
            TransactionStatus::Reconciled,
            TransactionStatus::Remind,
            TransactionStatus::Void,
        ];

        // iterate over the statuses
        for (i, status) in inputs.into_iter().enumerate() {
            // fill in the raw string with the index that matches the status
            let input = format!(r#"<ope st="{}">"#, i);
            let expected = Ok(Transaction {
                status,
                ..Default::default()
            });
            // perform the check
            check_try_from_single_str(&input, expected);
        }
    }

    #[test]
    #[should_panic]
    fn parse_status_bad() {
        // use a string that should work in the `from_str` method to make sure that there
        // isn't confusion between the two parsing methods
        let input = r#"<ope st="none">"#;
        let expected = Ok(Transaction {
            status: TransactionStatus::None,
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }

    #[test]
    fn parse_good_flag() {
        let input = r#"<ope flags="1">"#;
        let expected = Ok(Transaction {
            flags: Some(1),
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }

    #[test]
    #[should_panic]
    fn parse_bad_flags() {
        let input = r#"<ope flags="somethingelse">"#;
        let expected = Ok(Transaction {
            flags: None,
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }

    #[test]
    fn parse_good_payee() {
        let input = r#"<ope payee="5">"#;
        let expected = Ok(Transaction {
            payee: Some(5),
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }

    #[test]
    #[should_panic]
    fn parse_bad_payee() {
        let input = r#"<ope payee="something-other-payee">"#;
        let expected = Ok(Transaction {
            payee: None,
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }

    #[test]
    fn parse_empty_memo() {
        let input = r#"<ope wording="">"#;
        let expected = Ok(Transaction {
            memo: None,
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }

    #[test]
    fn parse_simple_memo() {
        let input = r#"<ope wording="Simple memo">"#;
        let expected = Ok(Transaction {
            memo: Some(String::from("Simple memo")),
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }

    #[test]
    fn parse_memo_with_nontrivial_chars() {
        let input = r#"<ope wording="This &amp; that shouldn't cause a problem, right?">"#;
        let expected = Ok(Transaction {
            memo: Some(String::from(
                "This & that shouldn't cause a problem, right?",
            )),
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }
}
