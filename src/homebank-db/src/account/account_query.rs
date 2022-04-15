//! Query the accounts

use crate::{db::HomeBankDb, query::Query, Account, AccountType};
use regex::Regex;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "accounts", visible_alias = "a", about = "Query accounts")]
pub struct QueryAccounts {
    #[structopt(
        short = "T",
        long = "type",
        help = "Include accounts of a certain type. Options are 'None', 'Bank', 'Cash', 'Asset', 'CreditCard', 'Liability', 'Chequing', or 'Savings'.",
        value_name = "type"
    )]
    acct_type: Option<Vec<AccountType>>,

    #[structopt(
        short = "g",
        long = "group",
        help = "Include accounts in group(s) that match the regular expression",
        value_name = "regex"
    )]
    group: Option<Regex>,

    #[structopt(
        short = "i",
        long = "institution",
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

    fn exec(&self, db: &HomeBankDb) -> Vec<Self::T> {
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
            .map(|acct| acct.clone())
            .collect();

        filt_accounts
    }
}
