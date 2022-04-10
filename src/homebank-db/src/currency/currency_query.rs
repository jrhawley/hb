use regex::Regex;
use structopt::StructOpt;

use crate::{Currency, HomeBankDb, Query};

#[derive(Debug, StructOpt)]
#[structopt(name = "currencies", alias = "C", about = "Query currencies used")]
pub struct QueryCurrencies {
    #[structopt(help = "Name of the currency", value_name = "regex")]
    name: Option<Regex>,
}

impl QueryCurrencies {
    /// Retrieve the regular expression for the payee name
    pub fn name(&self) -> &Option<Regex> {
        &self.name
    }
}

impl Query for QueryCurrencies {
    type T = Currency;

    fn exec<'a>(&self, db: &'a HomeBankDb) -> Vec<&'a Currency> {
        let filt_payees = db
            .currencies()
            .values()
            // filter out currencies that don't match the regex
            .filter(|&p| match self.name() {
                Some(re) => re.is_match(p.name()),
                None => true,
            })
            .collect();

        filt_payees
    }
}
