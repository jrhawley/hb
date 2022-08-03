use super::{TransactionStatus, TransactionType};
use crate::{HomeBankDb, PayMode, Query, Transaction};
use chrono::NaiveDate;
use clap::Parser;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[clap(
    name = "transactions",
    visible_alias = "t",
    about = "Query transactions"
)]
pub struct QueryTransactions {
    #[clap(
        short = 'd',
        long = "date-from",
        help = "Include transactions starting from (and including) this date",
        parse(try_from_str = NaiveDate::from_str),
        value_name = "date"
    )]
    date_from: Option<NaiveDate>,

    #[clap(
        short = 'D',
        long = "date-to",
        help = "Include transactions up to (and excluding) this date",
        parse(try_from_str = NaiveDate::from_str),
        value_name = "date"
    )]
    date_to: Option<NaiveDate>,

    #[clap(
        short = 'l',
        long = "amount-lower",
        help = "Include transactions greater than (and including) this amount",
        value_name = "amount"
    )]
    amount_from: Option<f32>,

    #[clap(
        short = 'u',
        long = "amount-upper",
        help = "Include transactions less than (and excluding) this amount",
        value_name = "amount"
    )]
    amount_to: Option<f32>,

    #[clap(
        short = 's',
        long = "status",
        help = "Include transactions with a certain status",
        value_name = "status"
    )]
    status: Option<Vec<TransactionStatus>>,

    #[clap(
        short = 'c',
        long = "category",
        help = "Include transactions with categories that match the regular expression",
        value_name = "regex"
    )]
    category: Option<Regex>,

    #[clap(
        short = 'p',
        long = "payee",
        help = "Include transactions involving payees that match the regular expression",
        value_name = "regex"
    )]
    payee: Option<Regex>,

    #[clap(
        short = 'a',
        long = "account",
        help = "Include transactions involving accounts that match the regular expression",
        value_name = "regex"
    )]
    account: Option<Regex>,

    #[clap(
        short = 'M',
        long = "method",
        help = "Include transactions with a certain payment method",
        value_name = "method"
    )]
    pay_mode: Option<Vec<PayMode>>,

    #[clap(
        short = 'm',
        long = "memo",
        help = "Include transactions whose memos match this regular expression",
        value_name = "regex"
    )]
    memo: Option<Regex>,

    #[clap(
        short = 'i',
        long = "info",
        help = "Include transactions whose info fields match this regular expression",
        value_name = "regex"
    )]
    info: Option<Regex>,

    #[clap(
        short = 't',
        long = "tag",
        help = "Include transactions whose tags match this regular expression",
        value_name = "regex"
    )]
    tags: Option<Regex>,

    #[clap(
        short = 'T',
        long = "type",
        help = "Include 'Expense', 'Income', or 'Transfer' transactions",
        value_name = "type"
    )]
    transaction_type: Option<Vec<TransactionType>>,
}

impl QueryTransactions {
    /// Create a new query for `Transaction`s
    pub fn new(
        date_from: &Option<NaiveDate>,
        date_to: &Option<NaiveDate>,
        amount_from: &Option<f32>,
        amount_to: &Option<f32>,
        status: &Option<Vec<TransactionStatus>>,
        category: &Option<Regex>,
        payee: &Option<Regex>,
        account: &Option<Regex>,
        pay_mode: &Option<Vec<PayMode>>,
        memo: &Option<Regex>,
        info: &Option<Regex>,
        tags: &Option<Regex>,
        transaction_type: &Option<Vec<TransactionType>>,
    ) -> Self {
        Self {
            date_from: date_from.clone(),
            date_to: date_to.clone(),
            amount_from: amount_from.clone(),
            amount_to: amount_to.clone(),
            status: status.clone(),
            category: category.clone(),
            payee: payee.clone(),
            account: account.clone(),
            pay_mode: pay_mode.clone(),
            memo: memo.clone(),
            info: info.clone(),
            tags: tags.clone(),
            transaction_type: transaction_type.clone(),
        }
    }

    /// Select the lower bound date for querying
    pub fn date_from(&self) -> &Option<NaiveDate> {
        &self.date_from
    }

    /// Select the upper bound date for querying
    pub fn date_to(&self) -> &Option<NaiveDate> {
        &self.date_to
    }

    /// Select the lower bound amount for querying
    pub fn amount_from(&self) -> &Option<f32> {
        &self.amount_from
    }

    /// Select the upper bound amount for querying
    pub fn amount_to(&self) -> &Option<f32> {
        &self.amount_to
    }

    /// Select the status(es) for including in the query
    pub fn status(&self) -> &Option<Vec<TransactionStatus>> {
        &self.status
    }

    /// Select the category regex for including in the query
    pub fn category(&self) -> &Option<Regex> {
        &self.category
    }

    /// Select the payee regex for including in the query
    pub fn payee(&self) -> &Option<Regex> {
        &self.payee
    }

    /// Select the account regex for including in the query
    pub fn account(&self) -> &Option<Regex> {
        &self.account
    }

    /// Select the payment method(s) for including in the query
    pub fn pay_mode(&self) -> &Option<Vec<PayMode>> {
        &self.pay_mode
    }

    /// Select the memo regex for including in the query
    pub fn memo(&self) -> &Option<Regex> {
        &self.memo
    }

    /// Select the info regex for including in the query
    pub fn info(&self) -> &Option<Regex> {
        &self.info
    }

    /// Select the tags regex for including in the query
    pub fn tags(&self) -> &Option<Regex> {
        &self.tags
    }

    /// Select the transaction type for including in the query
    pub fn ttype(&self) -> &Option<Vec<TransactionType>> {
        &self.transaction_type
    }

    /// Filter out dates occurring before the query date
    pub fn filter_date_from(&self, tr: &Transaction) -> bool {
        match self.date_from() {
            Some(d) => tr.date() >= d,
            None => true,
        }
    }

    /// Filter out dates occurring after the query date
    pub fn filter_date_to(&self, tr: &Transaction) -> bool {
        match self.date_to() {
            Some(d) => tr.date() < d,
            None => true,
        }
    }

    /// Filter out amounts below the query amount lower bound
    pub fn filter_amount_from(&self, tr: &Transaction) -> bool {
        match self.amount_from() {
            Some(a) => tr.total() >= a,
            None => true,
        }
    }

    /// Filter out amounts above the query amount upper
    pub fn filter_amount_to(&self, tr: &Transaction) -> bool {
        match self.amount_to() {
            Some(a) => tr.total() < a,
            None => true,
        }
    }

    /// Filter out by status
    pub fn filter_status(&self, tr: &Transaction) -> bool {
        match self.status() {
            Some(v) => v.contains(tr.status()),
            None => true,
        }
    }

    /// Filter by payee names
    pub fn filter_payee(&self, tr: &Transaction, db: &HomeBankDb) -> bool {
        match (self.payee(), tr.payee_name(db)) {
            // if there is a regex and there is a category name
            (Some(re), Some(t_payee_name)) => re.is_match(&t_payee_name),
            // if there is a regex but no category
            (Some(_), None) => false,
            // if there is no regex
            (None, _) => true,
        }
    }

    /// Filer by account name
    pub fn filter_account(&self, tr: &Transaction, db: &HomeBankDb) -> bool {
        match (self.payee(), tr.account_name(db)) {
            // if there is a regex and there is a category name
            (Some(re), Some(tr_account_name)) => re.is_match(&tr_account_name),
            // if there is a regex but no category
            (Some(_), None) => false,
            // if there is no regex
            (None, _) => true,
        }
    }

    /// Filter by payment method
    pub fn filter_paymode(&self, tr: &Transaction) -> bool {
        match self.pay_mode() {
            Some(v) => v.contains(tr.pay_mode()),
            None => true,
        }
    }

    /// Filter by `TransactionType`
    pub fn filter_ttype(&self, tr: &Transaction) -> bool {
        match self.ttype() {
            Some(v) => v
                .iter()
                // check transaction types without explicitly checking the values
                .any(|queried_type| queried_type.is_similar_to(tr.ttype())),
            None => true,
        }
    }

    /// Filter by tags
    pub fn filter_tags(&self, tr: &Transaction) -> bool {
        match (self.tags(), tr.tags()) {
            (Some(re), Some(tags)) => {
                // combine all the tags back into a single string to perform a single regex match
                // this avoids performing the costly match multiple times
                let combined_tr_tags = tags.join(",");
                re.is_match(&combined_tr_tags)
            }
            (Some(_), None) => false,
            (None, _) => true,
        }
    }

    /// Filter by memo
    pub fn filter_memo(&self, tr: &Transaction) -> bool {
        match (self.memo(), tr.memo()) {
            (Some(re), Some(memo)) => re.is_match(memo),
            (Some(_), None) => false,
            (None, _) => true,
        }
    }

    /// Filter by info
    pub fn filter_info(&self, tr: &Transaction) -> bool {
        match (self.info(), tr.info()) {
            (Some(re), Some(info)) => re.is_match(info),
            (Some(_), None) => false,
            (None, _) => true,
        }
    }

    /// Filter map the `Transaction` by the `Category`
    pub fn filter_category(&self, tr: &Transaction, db: &HomeBankDb) -> Option<Transaction> {
        match self.category() {
            Some(re) => {
                let matching_idx: Vec<usize> = tr
                    .category_names(db)
                    .iter()
                    .enumerate()
                    .filter_map(|(i, cat)| match cat {
                        Some(u) => {
                            if re.is_match(u) {
                                Some(i)
                            } else {
                                None
                            }
                        }
                        None => None,
                    })
                    .collect();

                // return the subset of the `Transaction` that matches the category query
                tr.subset(&matching_idx)
            }
            None => Some(tr.clone()),
        }
    }
}

impl Query for QueryTransactions {
    type T = Transaction;

    fn exec(&self, db: &HomeBankDb) -> Vec<Self::T> {
        let filt_transactions: Vec<Transaction> = db
            .transactions()
            .iter()
            .filter(|&tr| self.filter_date_from(tr))
            .filter(|&tr| self.filter_date_to(tr))
            .filter(|&tr| self.filter_amount_from(tr))
            .filter(|&tr| self.filter_amount_to(tr))
            .filter(|&tr| self.filter_status(tr))
            .filter(|&tr| self.filter_payee(tr, db))
            .filter(|&tr| self.filter_account(tr, db))
            .filter(|&tr| self.filter_paymode(tr))
            .filter(|&tr| self.filter_ttype(tr))
            .filter(|&tr| self.filter_tags(tr))
            .filter(|&tr| self.filter_memo(tr))
            .filter(|&tr| self.filter_info(tr))
            .filter_map(|tr| self.filter_category(tr, db))
            .collect();

        filt_transactions
    }
}
