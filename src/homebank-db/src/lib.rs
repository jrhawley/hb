pub mod account;
pub mod account_type;
pub mod category;
pub mod currency;
pub mod db;
pub mod db_properties;
pub mod db_version;
pub mod error;
pub mod group;
pub mod payee;
pub mod paymode;
pub mod template;
pub mod transaction;
pub mod transaction_status;
pub mod transaction_type;

pub use account::Account;
pub use account_type::AccountType;
pub use category::Category;
pub use currency::Currency;
pub use db::HomeBankDb;
pub use db_properties::HomeBankDbProperties;
pub use error::{AccountError, CategoryError, CurrencyError, HomeBankDbError};
pub use group::Group;
pub use payee::Payee;
pub use paymode::PayMode;
pub use template::Template;
pub use transaction::{Transaction, TransactionError};
pub use transaction_status::TransactionStatus;
pub use transaction_type::TransactionType;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
