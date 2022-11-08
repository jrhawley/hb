//! Categories for each [`Transaction`][crate::transaction::transaction::Transaction].

pub mod budget_query;
pub mod category;
pub mod category_budget;
pub mod category_error;
pub mod category_flags;
pub mod category_query;

pub use budget_query::QueryBudget;
pub use category::Category;
pub use category_budget::CategoryBudget;
pub use category_error::CategoryError;
pub use category_flags::{CategoryFlag, CategoryFlags};
pub use category_query::QueryCategories;
