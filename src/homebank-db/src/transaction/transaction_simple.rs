//! A simple `Transaction` that only belongs to a single `Category`.

#[derive(Debug, PartialEq, Eq)]
pub struct SimpleTransaction {
    /// Which category does this transaction fall under
    category: Option<usize>,
}
