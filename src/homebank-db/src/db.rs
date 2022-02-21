//! Data structure for the HomeBank database.

use std::path::Path;

use crate::{
    currency::Currency, Account, Category, Favourite, Group, HomeBankDbError, Payee, Transaction,
};
use semver::Version;

#[derive(Debug, PartialEq)]
pub struct HomeBankDbProperties {
    title: String,
    currency_idx: usize,
    car_category_idx: usize,
    auto_smode: usize,
    auto_weekday: usize,
}

impl HomeBankDbProperties {
    /// Create an empty, default set of properties
    fn empty() -> Self {
        Self {
            title: String::from(""),
            currency_idx: 1,
            car_category_idx: 1,
            auto_smode: 1,
            auto_weekday: 1,
        }
    }
}

impl Default for HomeBankDbProperties {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, PartialEq)]
pub struct HomeBankDb {
    xml_schema: Version,
    homebank_version: Version,
    properties: HomeBankDbProperties,
    currencies: Vec<Currency>,
    groups: Vec<Group>,
    accounts: Vec<Account>,
    payees: Vec<Payee>,
    categories: Vec<Category>,
    favourites: Vec<Favourite>,
    transactions: Vec<Transaction>,
}

impl HomeBankDb {
    /// Create an empty, default, HomeBank database
    fn empty() -> Self {
        Self {
            xml_schema: Version::new(1, 0, 0),
            homebank_version: Version::new(1, 4, 0),
            properties: HomeBankDbProperties::empty(),
            currencies: vec![],
            groups: vec![],
            accounts: vec![],
            payees: vec![],
            categories: vec![],
            favourites: vec![],
            transactions: vec![],
        }
    }
}

impl TryFrom<&Path> for HomeBankDb {
    type Error = HomeBankDbError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        if !path.exists() {
            return Err(HomeBankDbError::DoesNotExist(path.to_path_buf()));
        }

        Ok(Self::empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_hdb_props() {
        let observed = HomeBankDbProperties::empty();
        let expected = HomeBankDbProperties {
            title: String::from(""),
            currency_idx: 1,
            car_category_idx: 1,
            auto_smode: 1,
            auto_weekday: 1,
        };

        assert_eq!(expected, observed);
    }

    #[test]
    fn empty_hdb() {
        let observed = HomeBankDb::empty();
        let expected = HomeBankDb {
            xml_schema: Version::new(1, 0, 0),
            homebank_version: Version::new(1, 4, 0),
            properties: HomeBankDbProperties::empty(),
            currencies: vec![],
            groups: vec![],
            accounts: vec![],
            payees: vec![],
            categories: vec![],
            favourites: vec![],
            transactions: vec![],
        };

        assert_eq!(expected, observed);
    }

    #[test]
    fn minimal_db() {
        let path = Path::new("tests/minimal.xhb");
        let observed = HomeBankDb::try_from(path).unwrap();
        let expected = HomeBankDb::empty();

        assert_eq!(expected, observed);
    }
}
