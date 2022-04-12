//! Transactions that transfer amounts between accounts.

#[derive(Debug, PartialEq, Eq)]
pub struct Transfer {
    /// What is the corresponding destination account
    destination_account_idx: usize,
    /// Unique identifier for the transfer
    transfer_key: usize,
}

impl Transfer {
    /// Create an empty transfer that doesn't link to any accounts
    pub fn empty() -> Self {
        Self {
            destination_account_idx: 0,
            transfer_key: 0,
        }
    }

    /// Retrieve the destination account index
    pub fn destination(&self) -> &usize {
        &self.destination_account_idx
    }

    /// Retrieve the transfer key
    pub fn transfer_key(&self) -> &usize {
        &self.transfer_key
    }
}

impl Default for Transfer {
    fn default() -> Self {
        Self::empty()
    }
}
