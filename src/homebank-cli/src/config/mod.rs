//! Configuration for the application

pub mod cfg;
pub mod error;
pub mod parse;

pub use cfg::{default_cfg_file, Config};
pub use error::ConfigError;
