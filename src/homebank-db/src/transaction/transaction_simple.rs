//! A simple [`Transaction`][crate::transaction::transaction_struct::Transaction] that only belongs to a single [`Category`][crate::category::category_struct::Category].

/// A simple [`Transaction`][crate::transaction::transaction_struct::Transaction] that only belongs to a single [`Category`][crate::category::category_struct::Category].
#[derive(Debug, PartialEq, Clone)]
pub struct SimpleTransaction {
    /// The [`Category`][crate::category::category_struct::Category] this [`Transaction`][crate::transaction::transaction_struct::Transaction] falls under.
    category: Option<usize>,

    /// The amount of the parent [`Transaction`][crate::transaction::transaction_struct::Transaction].
    /// This will duplicate data, but this impacts the code base much less
    /// than using pointers and introducing lifetimes everywhere.
    amount: f32,

    /// The memo of the parent [`Transaction`][crate::transaction::transaction_struct::Transaction].
    /// This will duplicate data, but this impacts the code base much less
    /// than using pointers and introducing lifetimes everywhere.
    memo: Option<String>,
}

impl SimpleTransaction {
    /// Create an empty [`SimpleTransaction`]
    pub fn empty() -> Self {
        Self {
            category: None,
            amount: 0.0,
            memo: None,
        }
    }

    /// Create an new [`SimpleTransaction`]
    pub fn new(category: Option<usize>, amount: f32, memo: Option<String>) -> Self {
        Self {
            category,
            amount,
            memo,
        }
    }

    /// Retrieve the [`Category`][crate::category::category_struct::Category] for the [`Transaction`][crate::transaction::transaction_struct::Transaction].
    pub fn category(&self) -> &Option<usize> {
        &self.category
    }

    /// Retrieve the mutable [`Category`][crate::category::category_struct::Category] for the [`Transaction`][crate::transaction::transaction_struct::Transaction].
    pub fn mut_category(&mut self) -> &mut Option<usize> {
        &mut self.category
    }

    /// Retrieve the amount of the [`Transaction`][crate::transaction::transaction_struct::Transaction].
    pub fn amount(&self) -> &f32 {
        &self.amount
    }

    /// Retrieve the mutable amount of the [`Transaction`][crate::transaction::transaction_struct::Transaction].
    pub fn mut_amount(&mut self) -> &mut f32 {
        &mut self.amount
    }

    /// Retrieve the memo for the [`Transaction`][crate::transaction::transaction_struct::Transaction].
    pub fn memo(&self) -> &Option<String> {
        &self.memo
    }

    /// Retrieve the mutable memo for the [`Transaction`][crate::transaction::transaction_struct::Transaction].
    pub fn mut_memo(&mut self) -> &mut Option<String> {
        &mut self.memo
    }
}

impl Default for SimpleTransaction {
    fn default() -> Self {
        Self::empty()
    }
}
