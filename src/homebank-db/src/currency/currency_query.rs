//! Options for filtering [`Currencies`][crate::currency::currency_struct::Currency] from the [`HomeBankDb`].

use crate::{Currency, HomeBankDb, Query};
use clap::Parser;
use regex::Regex;

/// Options for filtering [`Currencies`][crate::currency::currency_struct::Currency] from the [`HomeBankDb`].
#[derive(Debug, Parser)]
#[clap(
    name = "currencies",
    visible_alias = "C",
    about = "Query currencies used"
)]
pub struct QueryCurrencies {
    /// Name of the currency.
    #[clap(value_name = "regex")]
    name: Option<Regex>,
}

impl QueryCurrencies {
    /// Retrieve the regular expression for the payee name
    fn name(&self) -> &Option<Regex> {
        &self.name
    }
}

impl Query for QueryCurrencies {
    type T = Currency;

    fn exec(&self, db: &HomeBankDb) -> Vec<Self::T> {
        let filt_payees = db
            .currencies()
            .values()
            // filter out currencies that don't match the regex
            .filter(|&p| match self.name() {
                Some(re) => re.is_match(p.name()),
                None => true,
            })
            .cloned()
            .collect();

        filt_payees
    }
}
