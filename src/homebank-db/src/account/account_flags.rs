//! Flags that can be placed on an [`Account`][crate::account::account::Account].

use crate::AccountError;
use std::ops::BitAnd;
 
/// Flags that can be placed on an [`Account`][crate::account::account::Account].
#[derive(Debug, PartialEq, Eq)]
pub enum AccountFlag {
    /// No flags on the [`Account`][crate::account::account::Account].
    None,

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

impl TryFrom<u16> for AccountFlag {
    type Error = AccountError;

    fn try_from(val: u16) -> Result<AccountFlag, Self::Error> {
        match val {
            0 => Ok(AccountFlag::None),
            1 => Ok(AccountFlag::None),
            2 => Ok(AccountFlag::Closed),
            // this was previously used for `Added` in HomeBank v < 5.5.3
            4 => Ok(AccountFlag::None),
            // this was previously used for `Changed` in HomeBank v < 5.5.3
            8 => Ok(AccountFlag::None),
            16 => Ok(AccountFlag::NoSummary),
            32 => Ok(AccountFlag::NoBudget),
            64 => Ok(AccountFlag::NoReport),
            512 => Ok(AccountFlag::Added),
            1024 => Ok(AccountFlag::Changed),
            _ => Err(AccountError::InvalidFlags),
        }
    }
}

impl From<AccountFlag> for u16 {
    fn from(val: AccountFlag) -> Self {
        match val {
            AccountFlag::None => 0,
            AccountFlag::Closed => 2,
            AccountFlag::NoSummary => 16,
            AccountFlag::NoBudget => 32,
            AccountFlag::NoReport => 64,
            AccountFlag::Added => 512,
            AccountFlag::Changed => 1024,
        }
    }
}

/// The set of flags that can be placed on any [`Account`][crate::account::account::Account], stored efficiently.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccountFlags(pub u16);

impl AccountFlags {
    pub fn is_closed(&self) -> bool {
        self & AccountFlag::Closed
    }

    pub fn is_excluded_from_summary(&self) -> bool {
        !(self & AccountFlag::NoSummary)
    }

    pub fn is_excluded_from_budget(&self) -> bool {
        !(self & AccountFlag::NoBudget)
    }

    pub fn is_excluded_from_reports(&self) -> bool {
        !(self & AccountFlag::NoReport)
    }

    pub fn is_added(&self) -> bool {
        self & AccountFlag::Added
    }

    pub fn is_changed(&self) -> bool {
        self & AccountFlag::Changed
    }
}

impl From<u16> for AccountFlags {
    fn from(val: u16) -> Self {
        Self(val)
    }
}

impl From<AccountFlags> for u16 {
    fn from(val: AccountFlags) -> Self {
        val.0
    }
}

impl BitAnd<AccountFlag> for &AccountFlags {
    type Output = bool;

    fn bitand(self, rhs: AccountFlag) -> Self::Output {
        (self.0 & Into::<u16>::into(rhs)) == 0
    }
}
