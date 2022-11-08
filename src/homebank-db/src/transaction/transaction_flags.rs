//! Flags that can be placed on a [`Transaction`][crate::transaction::transaction::Transaction].

/// Flags that can be placed on a [`Transaction`][crate::transaction::transaction::Transaction].
#[derive(Debug, PartialEq, Eq)]
pub enum TransactionFlag {
    /// Equivalent to `OF_INCOME`.
    Income,
    
    /// A scheduled [`Transaction`][crate::transaction::transaction::Transaction].
    /// Equivalent to `OF_AUTO`.
    Scheduled,
    
    /// Equivalent to `OF_INTXFER`.
    Transfer,
    
    /// Equivalent to `OF_CHEQ2`.
    Cheque,
    
    /// Equivalent to `OF_LIMIT`.
    ScheduledLimit,
    
    /// The [`Transaction`][crate::transaction::transaction::Transaction] is a [`SplitTransaction`][crate::transaction::transaction_split::SplitTransaction].
    /// Equivalent to `OF_SPLIT`.
    Split,
    
    /// The [`Transaction`][crate::transaction::transaction::Transaction] was recently added and has not been saved.
    /// Equivalent to `OF_ADDED`.
    Added,

    /// The [`Transaction`][crate::transaction::transaction::Transaction] has been changed but not saved.
    /// Equivalent to `AF_CHANGED`.
    Changed,
}

/// The set of flags that can be placed on a [`Transaction`][crate::transaction::transaction::Transaction], stored efficiently.
pub struct TransactionFlags(u8);
