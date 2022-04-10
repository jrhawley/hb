//! Query the accounts

use super::Query;
use homebank_db::{Account, AccountType, HomeBankDb};
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
}

impl QueryAccounts {
    fn account_type(&self) -> &Option<Vec<AccountType>> {
        &self.acct_type
    }
}

impl Query for QueryAccounts {
    type T = Account;

    fn exec<'a>(&self, db: &'a HomeBankDb) -> Vec<&'a Self::T> {
        let filt_accounts = db
            .accounts()
            .values()
            .filter(|&acct| match self.account_type() {
                Some(v) => v.contains(acct.atype()),
                None => true,
            })
            .collect();

        filt_accounts
    }
}
