//! CLI argument parsing and configuration

pub mod command;
pub mod query;

pub use command::{CliOpts, SubCommand};
pub use query::{
    QueryAccounts, QueryCategories, QueryCurrencies, QueryFavourites, QueryGroups, QueryOpts,
    QueryPayees, QueryTransactions, QueryType,
};
