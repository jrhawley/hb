//! CLI argument parsing and configuration

pub mod command;
pub mod query;

pub use command::{CliOpts, SubCommand};
pub use query::{
    Query, QueryAccounts, QueryCategories, QueryCurrencies, QueryGroups, QueryOpts, QueryPayees,
    QueryTemplates, QueryTransactions, QueryType,
};
