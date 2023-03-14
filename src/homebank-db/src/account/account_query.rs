//! Options for filtering [`Account`s][crate::account::account_struct::Account] from the [`HomeBankDb`].

use crate::{db::HomeBankDb, query::Query, Account, AccountType};
use clap::Parser;
use regex::Regex;

/// Options for filtering the [`Account`s][crate::account::account_struct::Account]
#[derive(Debug, Parser)]
#[clap(name = "accounts", visible_alias = "a", about = "Query accounts")]
pub struct QueryAccounts {
    /// Include accounts of a certain type. Options are 'None', 'Bank', 'Cash', 'Asset', 'CreditCard', 'Liability', 'Chequing', or 'Savings'.
    #[clap(short = 'T', long = "type", value_name = "type")]
    acct_type: Option<Vec<AccountType>>,

    /// Include accounts in group(s) that match the regular expression.
    #[clap(short = 'g', long = "group", value_name = "regex")]
    group: Option<Regex>,

    /// Include accounts whose institutions match the regular expression.
    #[clap(short = 'i', long = "institution", value_name = "regex")]
    institution: Option<Regex>,
}

impl QueryAccounts {
    /// Retrieve the filter for [`AccountType`][crate::account::account_type::AccountType].
    fn account_type(&self) -> &Option<Vec<AccountType>> {
        &self.acct_type
    }

    /// Retrieve the filter for [`Account`][crate::account::account_struct::Account] group.
    fn group(&self) -> &Option<Regex> {
        &self.group
    }

    /// Retrieve the filter for [`Account`][crate::account::account_struct::Account] institution.
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
            .filter(|&acct| match (self.group(), acct.group()) {
                (Some(re), Some(grp_idx)) => {
                    // lookup the group index in the database
                    match db.groups().get(&grp_idx) {
                        Some(grp) => re.is_match(grp.name()),
                        None => false,
                    }
                }
                (Some(_), None) => false,
                (None, _) => true,
            })
            // filter the account institution
            .filter(|&acct| match self.institution() {
                Some(re) => re.is_match(acct.institution()),
                None => true,
            })
            .cloned()
            .collect();

        filt_accounts
    }
}
