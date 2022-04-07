//! The type of a `Transaction`

#[derive(Debug, PartialEq, Eq)]
pub enum TransactionType {
    Expense,
    Income,
    Transfer,
}
