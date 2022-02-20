//! CLI argument parsing and configuration

use crate::config::default_cfg_file;
use lazy_static::lazy_static;
use std::path::PathBuf;
use structopt::StructOpt;

lazy_static! {
    static ref DEFAULT_CFG: String = default_cfg_file().to_str().unwrap().to_string();
}

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
pub struct CliOpts {
    #[structopt(short = "c", long = "config", help = "Path to hb configuration file", default_value = &DEFAULT_CFG)]
    path: PathBuf,
}
