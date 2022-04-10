use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum TransactionError {
    #[error("Missing account from transaction.")]
    MissingAccount,
    #[error("Missing amount from transaction.")]
    MissingAmount,
    #[error("Missing date from transaction.")]
    MissingDate,
    #[error("Missing pay mode from transaction.")]
    MissingPayMode,
    #[error("Missing payee from transaction.")]
    MissingPayee,
    #[error("Invalid transaction status. Must be 0-4 or the status name 'None', 'Cleared', 'Reconciled', 'Remind', or 'Void'.")]
    InvalidStatus,
    #[error("Invalid transaction payment method. Must be 0-10, 'None', 'CreditCard', 'Cheque', 'Cash', 'BankTransfer', 'DebitCard', 'StandingOrder', 'ElectronicPayment', 'Deposit', 'FIFee', or 'DirectDebit'.")]
    InvalidPayMode,
    #[error("Invalid transaction type. Must be 'Expense', 'Income', or 'Transfer'.")]
    InvalidType,
    #[error("Invalid category `{0}`. Must be a `usize` type.")]
    InvalidCategory(String),
    #[error("Mismatched number of splits. Expected {0}, found {1}.")]
    MismatchedSplitNumber(usize, usize),
}
