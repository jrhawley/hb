//! Errors when parsing to [`Transaction`s][crate::transaction::transaction::Transaction] from the HomeBank XML file.

use thiserror::Error;

/// Errors when parsing to [`Transaction`s][crate::transaction::transaction::Transaction] from the HomeBank XML file.
#[derive(Debug, Error, PartialEq)]
pub enum TransactionError {
    /// When the account key is invalid or not found within the database.
    #[error("Invalid account identifier from transaction. Must be of type `usize`.")]
    InvalidAccount,

    /// When the transaction amount is invalid or missing.
    #[error("Missing transaction amount.")]
    InvalidAmount,
    
    /// When the transaction date is invalid or missing.
    #[error("Invalid transaction date.")]
    InvalidDate,
    
    /// When the payee key is invalid or not found within the database.
    #[error("Missing payee from transaction.")]
    InvalidPayee,

    /// When the status is invalid.
    #[error("Invalid transaction status. Must be 0-4 or the status name 'None', 'Cleared', 'Reconciled', 'Remind', or 'Void'.")]
    InvalidStatus,

    /// When the pay mode is invalid.
    #[error("Invalid transaction payment method. Must be 0-10, 'None', 'CreditCard', 'Cheque', 'Cash', 'BankTransfer', 'DebitCard', 'StandingOrder', 'ElectronicPayment', 'Deposit', 'FIFee', or 'DirectDebit'.")]
    InvalidPayMode,

    /// When the type of transaction is invalid.
    #[error("Invalid transaction type. Must be 'Expense', 'Income', or 'Transfer'.")]
    InvalidType,

    /// When the category key is invalid or is not found within the database.
    #[error("Invalid category `{0}`. Must be a `usize` type.")]
    InvalidCategory(String),

    /// When a [`SplitTransaction`][crate::transaction::transaction_split::SplitTransaction] contains the wrong number of splits.
    #[error("Mismatched number of splits. Expected {0}, found {1}.")]
    MismatchedSplitNumber(usize, usize),

    /// When the flags on a transaction are invalid.
    #[error("Invalid transaction flags. Must be a `usize` type.")]
    InvalidFlags,

    /// When a transfer's destination account is invalid or not found within the database.
    #[error("Invalid destination account identifier from transfer. Must be of type `usize` and cannot be 0.")]
    InvalidDestinationAccount,

    /// When the transfer key is invalid.
    #[error("Invalid transfer key. Must be of type `usize` and cannot be 0.")]
    InvalidTransferKey,
    
    /// When the category, memo, or other fields in a transaction are incompatible with either a [`SimpleTransaction`][crate::transaction::transaction_simple::SimpleTransaction] or a [`SplitTransaction`][crate::transaction::transaction_split::SplitTransaction].
    #[error("Transactions must be `SimpleTransaction` or `SplitTransaction`, but not both. `SplitTransaction`s cannot have a global category and `SimpleTransaction`s cannot have multiple memos or amounts.")]
    ConflictingInfoSimpleSplitTransaction,
}
