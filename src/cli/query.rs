//! Query the HomeBank database from the command line.

use chrono::NaiveDate;
use homebank_db::{PayMode, TransactionStatus, TransactionType};
use regex::Regex;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "query", alias = "q", about = "Query the database")]
pub struct QueryOpts {
    #[structopt(subcommand)]
    query_type: QueryType,
}

impl QueryOpts {
    /// Retrieve the type of query being made
    pub fn qtype(&self) -> &QueryType {
        &self.query_type
    }
}

#[derive(Debug, StructOpt)]
pub enum QueryType {
    Accounts(QueryAccounts),
    Categories(QueryCategories),
    Currencies(QueryCurrencies),
    Favourites(QueryFavourites),
    Groups(QueryGroups),
    Payees(QueryPayees),
    Transactions(QueryTransactions),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "accounts", alias = "a", about = "Query accounts")]
pub struct QueryAccounts {}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "categories",
    alias = "c",
    about = "Query transaction categories"
)]
pub struct QueryCategories {}

#[derive(Debug, StructOpt)]
#[structopt(name = "currencies", alias = "C", about = "Query currencies used")]
pub struct QueryCurrencies {}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "favourites",
    aliases = &["f", "favorites"],
    about = "Query templates and scheduled transactions"
)]
pub struct QueryFavourites {}

#[derive(Debug, StructOpt)]
#[structopt(name = "groups", alias = "g", about = "Query account groups")]
pub struct QueryGroups {}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "payees",
    alias = "p",
    about = "Query transaction payees, to and from"
)]
pub struct QueryPayees {}

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
        short = "s",
        help = "Include transactions with a certain status",
        value_name = "status"
    )]
    status: Option<Vec<TransactionStatus>>,

    #[structopt(
        short = "M",
        help = "Include transactions with a certain payment method",
        value_name = "method"
    )]
    paymode: Option<Vec<PayMode>>,

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

    /// Select the status(es) for including in the query
    pub fn status(&self) -> &Option<Vec<TransactionStatus>> {
        &self.status
    }

    /// Select the payment method(s) for including in the query
    pub fn paymode(&self) -> &Option<Vec<PayMode>> {
        &self.paymode
    }

    /// Select the memo regex for including in the query
    pub fn memo(&self) -> &Option<Regex> {
        &self.memo
    }

    /// Select the info regex for including in the query
    pub fn info(&self) -> &Option<Regex> {
        &self.info
    }

    /// Select the transaction type for including in the query
    pub fn ttype(&self) -> &Option<Vec<TransactionType>> {
        &self.transaction_type
    }
}
