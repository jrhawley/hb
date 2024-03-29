//! Options for filtering [`Payee`s][crate::payee::payee_struct::Payee] from the [`HomeBankDb`].

use crate::{HomeBankDb, Payee, Query};
use clap::Parser;
use regex::Regex;

/// Options for filtering [`Payee`s][crate::payee::payee_struct::Payee] from the [`HomeBankDb`].
#[derive(Debug, Parser)]
#[clap(
    name = "payees",
    visible_alias = "p",
    about = "Query transaction payees, to and from"
)]
pub struct QueryPayees {
    /// Name of the payee.
    #[clap(value_name = "regex")]
    name: Option<Regex>,
}

impl QueryPayees {
    /// Retrieve the regular expression for the payee name
    fn name(&self) -> &Option<Regex> {
        &self.name
    }
}

impl Query for QueryPayees {
    type T = Payee;

    fn exec(&self, db: &HomeBankDb) -> Vec<Self::T> {
        let filt_payees = db
            .payees()
            .values()
            // filter out payees that don't match the regex
            .filter(|&p| match self.name() {
                Some(re) => re.is_match(p.name()),
                None => true,
            })
            .cloned()
            .collect();

        filt_payees
    }
}
