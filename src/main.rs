use anyhow::Context;
use cli::CliOpts;
use config::Config;
use homebank_db::HomeBankDb;
use structopt::StructOpt;

pub mod cli;
pub mod config;

fn main() -> Result<(), anyhow::Error> {
    let cli_opts = CliOpts::from_args();
    let cfg = Config::try_from(cli_opts)?;
    let db = match HomeBankDb::try_from(cfg.path()) {
        Ok(db) => db,
        Err(e) => return Err(e).with_context(|| "Error parsing HomeBank file."),
    };

    println!("{:#?}", db);

    Ok(())
}

