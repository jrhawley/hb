//! The donor or recipient of a [`Transaction`][crate::transaction::transaction::Transaction].

pub mod payee;
pub mod payee_error;
pub mod payee_query;

pub use payee::Payee;
pub use payee_error::PayeeError;
pub use payee_query::QueryPayees;
