//! Query the HomeBank database from the command line.

use chrono::NaiveDate;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "query", alias = "q", about = "Query the database")]
pub struct QueryOpts {
    #[structopt(subcommand)]
    query_type: QueryType,
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
        short = "f",
        help = "Include transactions starting from (and including) this date",
        parse(try_from_str = NaiveDate::from_str),
        value_name = "date"
    )]
    date_from: Option<NaiveDate>,

    #[structopt(
        short = "t",
        help = "Include transactions up to (and excluding) this date",
        parse(try_from_str = NaiveDate::from_str),
        value_name = "date"
    )]
    date_to: Option<NaiveDate>,
}
