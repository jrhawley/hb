use cli::CliOpts;
use structopt::StructOpt;

pub mod cli;
pub mod config;

fn main() {
    let cli_opts = CliOpts::from_args();
}
