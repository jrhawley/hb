//! Query the HomeBank database from the command line.

use crate::{
    currency::QueryCurrencies, group::QueryGroups, payee::QueryPayees,
    transaction::QueryTransactions, HomeBankDb, QueryAccounts, QueryCategories,
};
use clap::Parser;

/// A common way to execute queries of different data types in the HomeBank database.
pub trait Query {
    type T;

    /// Execute the query
    fn exec(&self, db: &HomeBankDb) -> Vec<Self::T>;
}

/// A subcommand to query the database from the CLI.
#[derive(Debug, Parser)]
pub struct QueryOpts {
    #[clap(subcommand)]
    query_type: QueryType,
}

impl QueryOpts {
    /// Retrieve the type of query being made
    pub fn qtype(&self) -> &QueryType {
        &self.query_type
    }
}

/// Differentiate between the different query types from the CLI
#[derive(Debug, Parser)]
pub enum QueryType {
    Accounts(QueryAccounts),
    Categories(QueryCategories),
    Currencies(QueryCurrencies),
    Groups(QueryGroups),
    Payees(QueryPayees),
    // Templates(QueryTemplates),
    Transactions(QueryTransactions),
}
