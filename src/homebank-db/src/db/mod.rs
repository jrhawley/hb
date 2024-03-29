//! Data structure for the HomeBank database.

pub mod db_error;
pub mod db_properties;
pub mod db_struct;
pub mod db_version;

pub use db_struct::HomeBankDb;
pub use db_error::HomeBankDbError;
pub use db_properties::HomeBankDbProperties;
pub use db_version::HomeBankDbSchema;
