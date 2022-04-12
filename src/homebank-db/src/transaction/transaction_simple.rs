//! A simple `Transaction` that only belongs to a single `Category`.

#[derive(Debug, PartialEq)]
pub struct SimpleTransaction {
    /// Which category does this transaction fall under
    category: Option<usize>,

    /// The amount of the parent `Transaction`.
    /// This will duplicate data, but this impacts the code base much less
    /// than using pointers and introducing lifetimes everywhere.
    amount: f32,

    /// The memo of the parent `Transaction`.
    /// This will duplicate data, but this impacts the code base much less
    /// than using pointers and introducing lifetimes everywhere.
    memo: Option<String>,
}

impl SimpleTransaction {
    /// Create an empty `SimpleTransaction`
    pub fn empty() -> Self {
        Self {
            category: None,
            amount: 0.0,
            memo: None,
        }
    }

    /// Create an new `SimpleTransaction`
    pub fn new(category: Option<usize>, amount: f32, memo: Option<String>) -> Self {
        Self {
            category,
            amount,
            memo,
        }
    }

    /// Retrieve the `Category` for the `Transaction`
    pub fn category(&self) -> &Option<usize> {
        &self.category
    }

    /// Retrieve the mutable `Category` for the `Transaction`
    pub fn mut_category(&mut self) -> &mut Option<usize> {
        &mut self.category
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

impl Default for SimpleTransaction {
    fn default() -> Self {
        Self::empty()
    }
}
