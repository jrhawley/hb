//! User-provided groups that an [`Account`][crate::account::account::Account] belongs to.

pub mod group;
pub mod group_error;
pub mod group_query;

pub use group::Group;
pub use group_error::GroupError;
pub use group_query::QueryGroups;
