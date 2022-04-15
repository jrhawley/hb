//! Top level CLI command

use crate::config::default_cfg_file;
use homebank_db::{QueryOpts, QueryTransactions};
use lazy_static::lazy_static;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

lazy_static! {
    static ref DEFAULT_CFG: String = default_cfg_file().to_str().unwrap().to_string();
}

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
pub struct CliOpts {
    #[structopt(
        short = "c",
        long = "config",
        help = "Path to hb configuration file",
        default_value = &DEFAULT_CFG
    )]
    pub path: PathBuf,

    // make optional subcommands
    #[structopt(subcommand)]
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

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    #[structopt(
        about = "Perform a query on the HomeBank database",
        visible_alias = "q"
    )]
    Query(QueryOpts),
    #[structopt(
        about = "Calculate a sum of transactions in a query",
        visible_alias = "s"
    )]
    Sum(QueryTransactions),
}
