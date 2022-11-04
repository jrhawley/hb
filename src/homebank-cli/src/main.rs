//! A command line interface for [HomeBank](https://homebank.free.fr).
//! 
//! The interface allows you to query the HomeBank database by using the [`homebank_db`][homebank_db] crate.


use anyhow::Context;
use clap::Parser;
use cli::{budget::budget_pbar, CliOpts, SubCommand};
use config::Config;
use homebank_db::{transaction::sum_transactions, HomeBankDb, Query, QueryType};

pub mod cli;
pub mod config;

/// Run the command line interface.
fn main() -> Result<(), anyhow::Error> {
    let cli_opts = CliOpts::parse();

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

                for cat in filt_categories {
                    println!("{}", cat.full_name(&db));
                }
            }
            QueryType::Accounts(query) => {
                let filt_accounts = query.exec(&db);

                println!("{:#?}", filt_accounts);
            }
            QueryType::Groups(query) => {
                let filt_groups = query.exec(&db);

                println!("{:#?}", filt_groups);
            }
        },
        // QueryType::Templates(query) => {
        //     let filt_templates = query.exec(&db);

        //     println!("{:#?}", filt_templates);
        // }
        Some(SubCommand::Sum(query)) => {
            let filt_transactions = query.exec(&db);
            let sum = sum_transactions(&filt_transactions);
            println!("{sum:.2}");
        }
        Some(SubCommand::Budget(query)) => {
            let filt_budget = query.exec(&db);

            if filt_budget.is_empty() {
                eprintln!("No budget(s) set for the matching categories.");
            }

            for summary in filt_budget {
                let pbar = budget_pbar(summary);
                pbar.abandon();
            }
        }
        None => {}
    }

    Ok(())
}
