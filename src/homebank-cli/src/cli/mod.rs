//! CLI argument parsing and configuration

pub mod budget;
pub mod command;

pub use budget::budget_pbar;
pub use command::{CliOpts, SubCommand};
