use super::Query;
use homebank_db::{HomeBankDb, Payee};
use regex::Regex;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "payees",
    alias = "p",
    about = "Query transaction payees, to and from"
)]
pub struct QueryPayees {
    #[structopt(about = "Name of the payee", value_name = "regex")]
    name: Regex,
}

impl QueryPayees {
    /// Retrieve the regular expression for the payee name
    pub fn name(&self) -> &Regex {
        &self.name
    }
}

impl Query for QueryPayees {
    type T = Payee;

    fn exec<'a>(&self, db: &'a HomeBankDb) -> Vec<&'a Payee> {
        let filt_payees = db
            .payees()
            .values()
            // filter out payees that don't match the regex
            .filter(|&p| self.name().is_match(p.name()))
            .collect();

        filt_payees
    }
}
