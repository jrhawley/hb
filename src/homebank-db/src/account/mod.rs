//! Chequing, savings, and other types of financial accounts.

pub mod account_error;
pub mod account_query;
pub mod account_struct;
pub mod account_type;

pub use account_struct::Account;
pub use account_error::AccountError;
pub use account_query::QueryAccounts;
pub use account_type::AccountType;
