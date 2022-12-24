//! Top level CLI command

use crate::config::default_cfg_file;
use clap::Parser;
use homebank_db::{category::{QueryBudget, QueryReview}, QueryOpts, QueryTransactions};
use lazy_static::lazy_static;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref DEFAULT_CFG: String = default_cfg_file().to_str().unwrap().to_string();
}

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct CliOpts {
    /// Path to `hb` (not HomeBank) configuration file
    #[clap(
        short = 'c',
        long = "config",
        default_value = &DEFAULT_CFG
    )]
    pub path: PathBuf,

    /// Optional subcommand
    #[clap(subcommand)]
    pub subcmd: Option<SubCommand>,
}

impl CliOpts {
    /// Create a new `CliOpts`
    pub fn new(path: &Path, subcmd: Option<SubCommand>) -> Self {
        Self {
            path: path.to_path_buf(),
            subcmd,
        }
    }

    /// Retrieve the CLI config path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Retrieve the subcommand given, if any
    pub fn subcommand(&self) -> Option<&SubCommand> {
        match &self.subcmd {
            Some(sc) => Some(sc),
            None => None,
        }
    }
}

impl Default for CliOpts {
    fn default() -> Self {
        CliOpts {
            path: default_cfg_file(),
            subcmd: None,
        }
    }
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    /// Perform a query on the HomeBank database.
    #[clap(visible_alias = "q")]
    Query(QueryOpts),

    /// Calculate a sum of transactions in a query.
    #[clap(visible_alias = "s")]
    Sum(QueryTransactions),

    /// Print a tab-separated table of each category and the sum of its transactions.
    #[clap(visible_alias = "r")]
    Review(QueryReview),

    /// Look at your category budgets.
    #[clap(visible_alias = "b")]
    Budget(QueryBudget),
}
