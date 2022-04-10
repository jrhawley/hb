//! Query the accounts

use super::Query;
use homebank_db::{Account, AccountType, HomeBankDb};
use regex::Regex;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "accounts", alias = "a", about = "Query accounts")]
pub struct QueryAccounts {
    #[structopt(
        short = "T",
        help = "Include accounts of a certain type. Options are 'None', 'Bank', 'Cash', 'Asset', 'CreditCard', 'Liability', 'Chequing', or 'Savings'.",
        value_name = "type"
    )]
    acct_type: Option<Vec<AccountType>>,

    #[structopt(
        short = "g",
        help = "Include accounts in group(s) that match the regular expression",
        value_name = "regex"
    )]
    group: Option<Regex>,

    #[structopt(
        short = "i",
        help = "Include accounts whose institutions match the regular expression",
        value_name = "regex"
    )]
    institution: Option<Regex>,
}

impl QueryAccounts {
    fn account_type(&self) -> &Option<Vec<AccountType>> {
        &self.acct_type
    }

    fn group(&self) -> &Option<Regex> {
        &self.group
    }

    fn institution(&self) -> &Option<Regex> {
        &self.institution
    }
}

impl Query for QueryAccounts {
    type T = Account;

    fn exec<'a>(&self, db: &'a HomeBankDb) -> Vec<&'a Self::T> {
        let filt_accounts = db
            .accounts()
            .values()
            // filter the account types
            .filter(|&acct| match self.account_type() {
                Some(v) => v.contains(acct.atype()),
                None => true,
            })
            // filter the account group
            .filter(
                |&acct| match (self.group(), db.groups().get(acct.group())) {
                    (Some(re), Some(grp)) => re.is_match(grp.name()),
                    (Some(_), None) => false,
                    (None, _) => true,
                },
            )
            // filter the account institution
            .filter(|&acct| match self.institution() {
                Some(re) => re.is_match(acct.institution()),
                None => true,
            })
            .collect();

        filt_accounts
    }
}
