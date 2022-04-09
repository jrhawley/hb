use super::Query;
use chrono::NaiveDate;
use homebank_db::{HomeBankDb, PayMode, Transaction, TransactionStatus, TransactionType};
use regex::Regex;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "transactions", alias = "t", about = "Query transactions")]
pub struct QueryTransactions {
    #[structopt(
        short = "d",
        help = "Include transactions starting from (and including) this date",
        parse(try_from_str = NaiveDate::from_str),
        value_name = "date"
    )]
    date_from: Option<NaiveDate>,

    #[structopt(
        short = "D",
        help = "Include transactions up to (and excluding) this date",
        parse(try_from_str = NaiveDate::from_str),
        value_name = "date"
    )]
    date_to: Option<NaiveDate>,

    #[structopt(
        short = "l",
        help = "Include transactions greater than (and including) this amount",
        value_name = "amount"
    )]
    amount_from: Option<f32>,

    #[structopt(
        short = "u",
        help = "Include transactions less than (and excluding) this amount",
        value_name = "amount"
    )]
    amount_to: Option<f32>,

    #[structopt(
        short = "s",
        help = "Include transactions with a certain status",
        value_name = "status"
    )]
    status: Option<Vec<TransactionStatus>>,

    #[structopt(
        short = "c",
        help = "Include transactions with categories that match the regular expression",
        value_name = "regex"
    )]
    category: Option<Regex>,

    #[structopt(
        short = "M",
        help = "Include transactions with a certain payment method",
        value_name = "method"
    )]
    pay_mode: Option<Vec<PayMode>>,

    #[structopt(
        short = "m",
        help = "Include transactions whose memos match this regular expression",
        value_name = "regex"
    )]
    memo: Option<Regex>,

    #[structopt(
        short = "i",
        help = "Include transactions whose info fields match this regular expression",
        value_name = "regex"
    )]
    info: Option<Regex>,

    #[structopt(
        short = "t",
        help = "Include transactions whose tags match this regular expression",
        value_name = "regex"
    )]
    tags: Option<Regex>,

    #[structopt(
        short = "T",
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

    fn exec<'a>(&self, db: &'a HomeBankDb) -> Vec<&'a Self::T> {
        let filt_transactions: Vec<&Transaction> = db
            .transactions()
            .iter()
            // filter out dates before the given date
            .filter(|&t| match self.date_from() {
                Some(d) => t.date() >= d,
                None => true,
            })
            // filter out dates on or after the given date
            .filter(|&t| match self.date_to() {
                Some(d) => t.date() < d,
                None => true,
            })
            // filter out amounts less than the lower bound
            .filter(|&t| match self.amount_from() {
                Some(a) => t.amount() >= a,
                None => true,
            })
            // filter out amounts greater than the upper bound
            .filter(|&t| match self.amount_to() {
                Some(a) => t.amount() < a,
                None => true,
            })
            // filter out certain statuses
            .filter(|&t| match self.status() {
                Some(v) => v.contains(t.status()),
                None => true,
            })
            // filter out certain payment methods
            .filter(|&t| match self.pay_mode() {
                Some(v) => v.contains(t.pay_mode()),
                None => true,
            })
            // filter out transaction types
            .filter(|&t| match self.ttype() {
                Some(v) => v.contains(t.ttype()),
                None => true,
            })
            // filter out the memo regex
            .filter(|&t| match (self.memo(), t.memo()) {
                (Some(re), Some(memo)) => re.is_match(memo),
                (Some(_), None) => false,
                (None, _) => true,
            })
            // filter out tags that don't match the regex
            .filter(|&t| match (self.tags(), t.tags()) {
                (Some(re), Some(tags)) => {
                    // combine all the tags back into a single string to perform a single regex match
                    // this avoids performing the costly match multiple times
                    let combined_tr_tags = tags.join(",");
                    re.is_match(&combined_tr_tags)
                }
                (Some(_), None) => false,
                (None, _) => true,
            })
            // filter out the info regex
            .filter(|&t| match (self.info(), t.info()) {
                (Some(re), Some(info)) => re.is_match(info),
                (Some(_), None) => false,
                (None, _) => true,
            })
            .collect();

        filt_transactions
    }
}
