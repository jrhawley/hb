use crate::{HomeBankDb, Payee, Query};
use regex::Regex;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "payees",
    alias = "p",
    about = "Query transaction payees, to and from"
)]
pub struct QueryPayees {
    #[structopt(help = "Name of the payee", value_name = "regex")]
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

    fn exec<'a>(&self, db: &'a HomeBankDb) -> Vec<&'a Payee> {
        let filt_payees = db
            .payees()
            .values()
            // filter out payees that don't match the regex
            .filter(|&p| match self.name() {
                Some(re) => re.is_match(p.name()),
                None => true,
            })
            .collect();

        filt_payees
    }
}
