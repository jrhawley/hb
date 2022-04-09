//! Query the accounts

use super::Query;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "accounts", alias = "a", about = "Query accounts")]
pub struct QueryAccounts {}
