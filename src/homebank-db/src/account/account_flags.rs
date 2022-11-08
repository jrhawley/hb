//! Flags that can be placed on an [`Account`][crate::account::account::Account].
 
/// Flags that can be placed on an [`Account`][crate::account::account::Account].
#[derive(Debug, PartialEq, Eq)]
pub enum AccountFlag {
    /// The [`Account`][crate::account::account::Account] is marked as closed.
    /// Equivalent to `AF_CLOSED`.
    Closed,

    /// The [`Account`][crate::account::account::Account] is excluded from the account summary.
    /// Equivalent to `AF_NOSUMMARY`.
    NoSummary,

    /// The [`Account`][crate::account::account::Account] is excluded from the budget.
    /// Equivalent to `AF_NOBUDGET`.
    NoBudget,

    /// The [`Account`][crate::account::account::Account] is excluded from any reports.
    /// Equivalent to `AF_NOREPORT`.
    NoReport,

    /// The [`Account`][crate::account::account::Account] was recently added and has not been saved.
    /// Equivalent to `AF_ADDED`.
    Added,

    /// The [`Account`][crate::account::account::Account] has been changed but not saved.
    /// Equivalent to `AF_CHANGED`.
    Changed,
}

/// The set of flags that can be placed on any [`Account`][crate::account::account::Account], stored efficiently.
pub struct AccountFlags(u8);