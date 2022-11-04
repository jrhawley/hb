//! Library implementation of HomeBank that is heavily inspired by [HomeBank's C implementation](https://code.launchpad.net/~mdoyen/homebank/).

pub mod account;
pub mod category;
pub mod currency;
pub mod db;
pub mod group;
pub mod payee;
pub mod paymode;
pub mod query;
// pub mod template;
pub mod transaction;

pub use account::{Account, AccountError, AccountType, QueryAccounts};
pub use category::{Category, CategoryError, QueryCategories};
pub use currency::{Currency, CurrencyError, QueryCurrencies};
pub use db::{HomeBankDb, HomeBankDbProperties, HomeBankDbSchema};
pub use group::{Group, QueryGroups};
pub use payee::{Payee, PayeeError, QueryPayees};
pub use paymode::PayMode;
pub use query::{Query, QueryOpts, QueryType};
// pub use template::{QueryTemplates, Template};
pub use transaction::{
    QueryTransactions, Transaction, TransactionError, TransactionStatus, TransactionType,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
