//! Transactions that transfer amounts between accounts.

#[derive(Debug, PartialEq, Eq)]
pub struct Transfer {
    /// Unique identifier for the transfer
    transfer_key: usize,
    /// What is the corresponding destination account
    destination_account_idx: usize,
}

impl Transfer {
    /// Create an empty transfer that doesn't link to any accounts
    pub fn empty() -> Self {
        Self {
            destination_account_idx: 0,
            transfer_key: 0,
        }
    }

    /// Create a new `Transfer`
    pub fn new(key: usize, acct: usize) -> Self {
        Self {
            transfer_key: key,
            destination_account_idx: acct,
        }
    }

    /// Retrieve the destination account index
    pub fn destination(&self) -> &usize {
        &self.destination_account_idx
    }

    /// Retrieve the mutable destination account index
    pub fn mut_destination(&mut self) -> &mut usize {
        &mut self.destination_account_idx
    }

    /// Retrieve the transfer key
    pub fn transfer_key(&self) -> &usize {
        &self.transfer_key
    }

    /// Retrieve the mutable transfer key
    pub fn mut_transfer_key(&mut self) -> &mut usize {
        &mut self.transfer_key
    }
}

impl Default for Transfer {
    fn default() -> Self {
        Self::empty()
    }
}
