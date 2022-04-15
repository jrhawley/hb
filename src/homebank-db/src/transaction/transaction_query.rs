use super::{TransactionStatus, TransactionType};
use crate::{HomeBankDb, PayMode, Query, Transaction};
use chrono::NaiveDate;
use regex::Regex;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "transactions",
    visible_alias = "t",
    about = "Query transactions"
)]
pub struct QueryTransactions {
    #[structopt(
        short = "d",
        long = "date-from",
        help = "Include transactions starting from (and including) this date",
        parse(try_from_str = NaiveDate::from_str),
        value_name = "date"
    )]
    date_from: Option<NaiveDate>,

    #[structopt(
        short = "D",
        long = "date-to",
        help = "Include transactions up to (and excluding) this date",
        parse(try_from_str = NaiveDate::from_str),
        value_name = "date"
    )]
    date_to: Option<NaiveDate>,

    #[structopt(
        short = "l",
        long = "amount-lower",
        help = "Include transactions greater than (and including) this amount",
        value_name = "amount"
    )]
    amount_from: Option<f32>,

    #[structopt(
        short = "u",
        long = "amount-upper",
        help = "Include transactions less than (and excluding) this amount",
        value_name = "amount"
    )]
    amount_to: Option<f32>,

    #[structopt(
        short = "s",
        long = "status",
        help = "Include transactions with a certain status",
        value_name = "status"
    )]
    status: Option<Vec<TransactionStatus>>,

    #[structopt(
        short = "c",
        long = "category",
        help = "Include transactions with categories that match the regular expression",
        value_name = "regex"
    )]
    category: Option<Regex>,

    #[structopt(
        short = "p",
        long = "payee",
        help = "Include transactions involving payees that match the regular expression",
        value_name = "regex"
    )]
    payee: Option<Regex>,

    #[structopt(
        short = "a",
        long = "account",
        help = "Include transactions involving accounts that match the regular expression",
        value_name = "regex"
    )]
    account: Option<Regex>,

    #[structopt(
        short = "M",
        long = "method",
        help = "Include transactions with a certain payment method",
        value_name = "method"
    )]
    pay_mode: Option<Vec<PayMode>>,

    #[structopt(
        short = "m",
        long = "memo",
        help = "Include transactions whose memos match this regular expression",
        value_name = "regex"
    )]
    memo: Option<Regex>,

    #[structopt(
        short = "i",
        long = "info",
        help = "Include transactions whose info fields match this regular expression",
        value_name = "regex"
    )]
    info: Option<Regex>,

    #[structopt(
        short = "t",
        long = "tag",
        help = "Include transactions whose tags match this regular expression",
        value_name = "regex"
    )]
    tags: Option<Regex>,

    #[structopt(
        short = "T",
        long = "type",
        help = "Include 'Expense', 'Income', or 'Transfer' transactions",
        value_name = "type"
    )]
    transaction_type: Option<Vec<TransactionType>>,
}

impl QueryTransactions {
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
}

impl Query for QueryTransactions {
    type T = Transaction;

    fn exec(&self, db: &HomeBankDb) -> Vec<Self::T> {
        let filt_transactions: Vec<Transaction> = db
            .transactions()
            .iter()
            // filter out dates before the given date
            .filter(|&tr| match self.date_from() {
                Some(d) => tr.date() >= d,
                None => true,
            })
            // filter out dates on or after the given date
            .filter(|&tr| match self.date_to() {
                Some(d) => tr.date() < d,
                None => true,
            })
            // filter out amounts less than the lower bound
            .filter(|&tr| match self.amount_from() {
                Some(a) => tr.total() >= a,
                None => true,
            })
            // filter out amounts greater than the upper bound
            .filter(|&tr| match self.amount_to() {
                Some(a) => tr.total() < a,
                None => true,
            })
            // filter out certain statuses
            .filter(|&tr| match self.status() {
                Some(v) => v.contains(tr.status()),
                None => true,
            })
            // filter out certain payees
            .filter(|&tr| match (self.payee(), tr.payee_name(db)) {
                // if there is a regex and there is a category name
                (Some(re), Some(t_payee_name)) => re.is_match(&t_payee_name),
                // if there is a regex but no category
                (Some(_), None) => false,
                // if there is no regex
                (None, _) => true,
            })
            // filter out certain accounts
            .filter(|&tr| match (self.payee(), tr.account_name(db)) {
                // if there is a regex and there is a category name
                (Some(re), Some(tr_account_name)) => re.is_match(&tr_account_name),
                // if there is a regex but no category
                (Some(_), None) => false,
                // if there is no regex
                (None, _) => true,
            })
            // filter out certain payment methods
            .filter(|&tr| match self.pay_mode() {
                Some(v) => v.contains(tr.pay_mode()),
                None => true,
            })
            // filter out transaction types
            .filter(|&tr| match self.ttype() {
                Some(v) => v
                    .iter()
                    // check transaction types without explicitly checking the values
                    .any(|queried_type| queried_type.is_similar_to(tr.ttype())),
                None => true,
            })
            // filter out tags that don't match the regex
            .filter(|&tr| match (self.tags(), tr.tags()) {
                (Some(re), Some(tags)) => {
                    // combine all the tags back into a single string to perform a single regex match
                    // this avoids performing the costly match multiple times
                    let combined_tr_tags = tags.join(",");
                    re.is_match(&combined_tr_tags)
                }
                (Some(_), None) => false,
                (None, _) => true,
            })
            // filter out the memo regex
            .filter(|&tr| match (self.memo(), tr.memo()) {
                (Some(re), Some(memo)) => re.is_match(memo),
                (Some(_), None) => false,
                (None, _) => true,
            })
            // filter out the info regex
            .filter(|&tr| match (self.info(), tr.info()) {
                (Some(re), Some(info)) => re.is_match(info),
                (Some(_), None) => false,
                (None, _) => true,
            })
            // filter out certain categories
            .filter_map(|tr| {
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
                    None => None,
                }
            })
            .collect();

        filt_transactions
    }
}
