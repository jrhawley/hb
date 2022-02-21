pub mod account;
pub mod category;
pub mod currency;
pub mod db;
pub mod favourite;
pub mod group;
pub mod payee;
pub mod transaction;

pub use account::Account;
pub use category::Category;
pub use currency::Currency;
pub use db::{HomeBankDB, HomeBankDBProperties};
pub use favourite::Favourite;
pub use group::Group;
pub use payee::Payee;
pub use transaction::Transaction;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
