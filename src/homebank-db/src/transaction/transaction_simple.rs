//! A simple `Transaction` that only belongs to a single `Category`.

#[derive(Debug, PartialEq, Eq)]
pub struct SimpleTransaction<'a> {
    /// Which category does this transaction fall under
    category: Option<usize>,

    /// The amount of the parent `Transaction`.
    /// Using a reference instead of a value to avoid duplicating data
    amount: &'a f32,

    /// The memo of the parent `Transaction`.
    /// Using a reference instead of a value to avoid duplicating data
    memo: &'a Option<String>,
}

impl<'a> SimpleTransaction<'a> {
    /// Create and empty `SimpleTransaction`
    pub fn empty() -> Self {
        Self { category: None }
    }

    /// Retrieve the `Category` for the `Transaction`
    pub fn category(&self) -> &Option<usize> {
        &self.category
    }

    /// Retrieve the amount of the `Transaction`
    pub fn amount(&self) -> &f32 {
        &self.amount
    }

    /// Retrieve the memo for the `Transaction`
    pub fn memo(&self) -> &Option<String> {
        &self.memo
    }
}

impl<'a> Default for SimpleTransaction<'a> {
    fn default() -> Self {
        Self::empty()
    }
}
