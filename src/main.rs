use crate::cli::{Query, QueryType};
use anyhow::Context;
use cli::{CliOpts, SubCommand};
use config::Config;
use homebank_db::HomeBankDb;
use structopt::StructOpt;

pub mod cli;
pub mod config;

fn main() -> Result<(), anyhow::Error> {
    let cli_opts = CliOpts::from_args();

    let cfg = Config::try_from(&cli_opts)?;
    let db = match HomeBankDb::try_from(cfg.path()) {
        Ok(db) => db,
        Err(e) => return Err(e).with_context(|| "Error parsing HomeBank file."),
    };

    println!("{:#?}", db.groups());
    println!("{:#?}", db.payees());
    println!("{:#?}", db.categories());
    println!("{:#?}", db.accounts());

    match &cli_opts.subcommand() {
        Some(SubCommand::Query(q_opts)) => match q_opts.qtype() {
            QueryType::Transactions(query) => {
                let filt_transactions = query.exec(&db);

                println!("{:#?}", filt_transactions);
            }
            _ => {}
        },
        None => {}
    }

    Ok(())
}
