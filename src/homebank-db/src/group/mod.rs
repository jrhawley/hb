//! User-provided groups that an [`Account`][crate::account::account_struct::Account] belongs to.

pub mod group_error;
pub mod group_query;
pub mod group_struct;

pub use group_struct::Group;
pub use group_error::GroupError;
pub use group_query::QueryGroups;
