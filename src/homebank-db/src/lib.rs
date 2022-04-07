pub mod account;
pub mod category;
pub mod currency;
pub mod db;
pub mod error;
pub mod favourite;
pub mod group;
pub mod payee;
pub mod paymode;
pub mod transaction;
pub mod transaction_status;

pub use account::Account;
pub use category::Category;
pub use currency::Currency;
pub use db::{HomeBankDb, HomeBankDbProperties};
pub use error::HomeBankDbError;
pub use favourite::Favourite;
pub use group::Group;
pub use payee::Payee;
pub use paymode::PayMode;
pub use transaction::{Transaction, TransactionError};
pub use transaction_status::TransactionStatus;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
