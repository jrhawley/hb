//! The donor or recipient of a [`Transaction`][crate::transaction::transaction_struct::Transaction].

pub mod payee_error;
pub mod payee_query;
pub mod payee_struct;

pub use payee_struct::Payee;
pub use payee_error::PayeeError;
pub use payee_query::QueryPayees;
