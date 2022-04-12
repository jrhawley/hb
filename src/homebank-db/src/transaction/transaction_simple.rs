//! A simple `Transaction` that only belongs to a single `Category`.

#[derive(Debug, PartialEq, Eq)]
pub struct SimpleTransaction {
    /// Which category does this transaction fall under
    category: Option<usize>,
}

impl SimpleTransaction {
    /// Create and empty `SimpleTransaction`
    pub fn empty() -> Self {
        Self { category: None }
    }
    /// Retrieve the `Category` for the `Transaction`
    pub fn category(&self) -> &Option<usize> {
        &self.category
    }
}

impl Default for SimpleTransaction {
    fn default() -> Self {
        Self::empty()
    }
}
