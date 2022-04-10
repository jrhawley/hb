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

    match &cli_opts.subcommand() {
        Some(SubCommand::Query(q_opts)) => match q_opts.qtype() {
            QueryType::Transactions(query) => {
                let filt_transactions = query.exec(&db);

                println!("{:#?}", filt_transactions);
            }
            QueryType::Payees(query) => {
                let filt_payees = query.exec(&db);

                println!("{:#?}", filt_payees);
            }
            QueryType::Currencies(query) => {
                let filt_currencies = query.exec(&db);

                println!("{:#?}", filt_currencies);
            }
            QueryType::Categories(query) => {
                let filt_categories = query.exec(&db);

                println!("{:#?}", filt_categories);
            }
            QueryType::Accounts(query) => {
                let filt_accounts = query.exec(&db);

                println!("{:#?}", filt_accounts);
            }
            _ => {}
        },
        None => {}
    }

    Ok(())
}
