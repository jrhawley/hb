//! Query the HomeBank database from the command line.

mod account;
mod category;
mod currency;
mod group;
mod payee;
mod template;
mod transaction;

pub use account::QueryAccounts;
pub use category::QueryCategories;
pub use currency::QueryCurrencies;
pub use group::QueryGroups;
pub use payee::QueryPayees;
pub use template::QueryTemplates;
pub use transaction::QueryTransactions;

use homebank_db::HomeBankDb;
use structopt::StructOpt;

/// A common way to execute queries of different data types in the HomeBank database.
pub trait Query {
    type T;

    /// Execute the query
    fn exec<'a>(&self, db: &'a HomeBankDb) -> Vec<&'a Self::T>;
}

/// A subcommand to query the database from the CLI.
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

/// Differentiate between the different query types from the CLI
#[derive(Debug, StructOpt)]
pub enum QueryType {
    Accounts(QueryAccounts),
    Categories(QueryCategories),
    Currencies(QueryCurrencies),
    Groups(QueryGroups),
    Payees(QueryPayees),
    Templates(QueryTemplates),
    Transactions(QueryTransactions),
}
