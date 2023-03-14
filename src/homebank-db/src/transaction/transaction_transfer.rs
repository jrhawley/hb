//! [`Transaction`s][crate::transaction::transaction::Transaction] that transfer amounts between [`Account`s][crate::account::account_struct::Account].

/// [`Transaction`s][crate::transaction::transaction::Transaction] that transfer amounts between [`Account`s][crate::account::account_struct::Account].
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Transfer {
    /// Unique identifier for the transfer.
    transfer_key: usize,

    /// The key for the corresponding destination [`Account`][crate::account::account_struct::Account].
    destination_account_idx: usize,
}

impl Transfer {
    /// Create an empty [`Transfer`] that doesn't link to any [`Account`s][crate::account::account_struct::Account].
    pub fn empty() -> Self {
        Self {
            destination_account_idx: 0,
            transfer_key: 0,
        }
    }

    /// Create a new [`Transfer`].
    pub fn new(key: usize, acct: usize) -> Self {
        Self {
            transfer_key: key,
            destination_account_idx: acct,
        }
    }

    /// Retrieve the destination [`Account`s][crate::account::account_struct::Account] key in the [`HomeBankDb`][crate::db::db_struct::HomeBankDb].
    pub fn destination(&self) -> &usize {
        &self.destination_account_idx
    }

    /// Retrieve the mutable destination [`Account`][crate::account::account_struct::Account] index.
    pub fn mut_destination(&mut self) -> &mut usize {
        &mut self.destination_account_idx
    }

    /// Retrieve the [`Transfer`] key.
    pub fn transfer_key(&self) -> &usize {
        &self.transfer_key
    }

    /// Retrieve the mutable [`Transfer`] key.
    pub fn mut_transfer_key(&mut self) -> &mut usize {
        &mut self.transfer_key
    }
}

impl Default for Transfer {
    fn default() -> Self {
        Self::empty()
    }
}
