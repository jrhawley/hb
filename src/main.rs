use cli::CliOpts;
use config::Config;
use structopt::StructOpt;

pub mod cli;
pub mod config;

fn main() -> Result<(), anyhow::Error> {
    let cli_opts = CliOpts::from_args();
    let cfg = Config::try_from(cli_opts)?;

    Ok(())
}
