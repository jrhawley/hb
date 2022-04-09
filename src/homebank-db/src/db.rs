//! Data structure for the HomeBank database.

use crate::{
    Account, Category, Currency, Group, HomeBankDbError, HomeBankDbProperties, Payee, Transaction,
};
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};
use xml::{reader::XmlEvent, EventReader};

#[derive(Debug, PartialEq)]
pub struct HomeBankDb {
    // #[serde(rename = "homebank")]
    // homebank_version: HomeBankDbVersion,
    properties: HomeBankDbProperties,
    currencies: HashMap<usize, Currency>,
    groups: HashMap<usize, Group>,
    accounts: HashMap<usize, Account>,
    payees: HashMap<usize, Payee>,
    categories: HashMap<usize, Category>,
    // favourites: Vec<Favourite>,
    transactions: Vec<Transaction>,
}

impl HomeBankDb {
    /// Create an empty, default, HomeBank database
    fn empty() -> Self {
        Self {
            // homebank_version: HomeBankDbVersion::empty(),
            properties: HomeBankDbProperties::empty(),
            currencies: HashMap::new(),
            groups: HashMap::new(),
            accounts: HashMap::new(),
            payees: HashMap::new(),
            categories: HashMap::new(),
            // favourites: vec![],
            transactions: vec![],
        }
    }

    /// Retrieve the database properties
    pub fn properties(&self) -> &HomeBankDbProperties {
        &self.properties
    }

    /// Retrieve the mutable transactions
    fn mut_properties(&mut self) -> &mut HomeBankDbProperties {
        &mut self.properties
    }

    /// Retrieve the database accounts
    pub fn accounts(&self) -> &HashMap<usize, Account> {
        &self.accounts
    }

    /// Retrieve the mutable accounts
    fn mut_accounts(&mut self) -> &mut HashMap<usize, Account> {
        &mut self.accounts
    }

    /// Retrieve the database properties
    pub fn currencies(&self) -> &HashMap<usize, Currency> {
        &self.currencies
    }

    /// Retrieve the mutable transactions
    fn mut_currencies(&mut self) -> &mut HashMap<usize, Currency> {
        &mut self.currencies
    }

    /// Retrieve the groups in the database
    pub fn groups(&self) -> &HashMap<usize, Group> {
        &self.groups
    }

    /// Retrieve the mutable currencies
    fn mut_groups(&mut self) -> &mut HashMap<usize, Group> {
        &mut self.groups
    }

    /// Retrieve the payees in the database
    pub fn payees(&self) -> &HashMap<usize, Payee> {
        &self.payees
    }

    /// Retrieve the mutable map of payees
    fn mut_payees(&mut self) -> &mut HashMap<usize, Payee> {
        &mut self.payees
    }

    /// Retrieve the payees in the database
    pub fn categories(&self) -> &HashMap<usize, Category> {
        &self.categories
    }

    /// Retrieve the mutable map of payees
    fn mut_categories(&mut self) -> &mut HashMap<usize, Category> {
        &mut self.categories
    }

    /// Retrieve the list of transactions
    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    /// Retrieve the mutable transactions
    fn mut_transactions(&mut self) -> &mut Vec<Transaction> {
        &mut self.transactions
    }
}

impl TryFrom<&Path> for HomeBankDb {
    type Error = HomeBankDbError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        if !path.exists() {
            return Err(HomeBankDbError::DoesNotExist(path.to_path_buf()));
        }

        let xhb_file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Err(HomeBankDbError::CouldNotOpen(path.to_path_buf())),
        };

        let xhb_buf = BufReader::new(xhb_file);
        let parser = EventReader::new(xhb_buf);

        // create the default HomeBankDb
        let mut db = HomeBankDb::empty();
        // check if the XML is parsing the HomeBank data or not
        let mut in_info = false;

        // using xml manual parsing to read in the file
        // not using some type of string parsing serde coersion because we
        // don't know how large the database is going to be
        for event in parser {
            match event {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.local_name == "homebank" {
                        in_info = true;
                    } else if in_info {
                        // only add data if we're within the `<homebank></homebank>` tags
                        match name.local_name.as_str() {
                            "properties" => {
                                if let Ok(props) = HomeBankDbProperties::try_from(attributes) {
                                    *db.mut_properties() = props;
                                }
                            }
                            "cur" => {
                                if let Ok(curr) = Currency::try_from(attributes) {
                                    db.mut_currencies().insert(curr.key(), curr);
                                }
                            }
                            "grp" => {
                                if let Ok(grp) = Group::try_from(attributes) {
                                    db.mut_groups().insert(grp.key(), grp);
                                }
                            }
                            "account" => {
                                if let Ok(acct) = Account::try_from(attributes) {
                                    db.mut_accounts().insert(acct.key(), acct);
                                }
                            }
                            "pay" => {
                                if let Ok(payee) = Payee::try_from(attributes) {
                                    db.mut_payees().insert(payee.key(), payee);
                                }
                            }
                            "cat" => {
                                if let Ok(cat) = Category::try_from(attributes) {
                                    db.mut_categories().insert(cat.key(), cat);
                                }
                            }
                            "fav" => {}
                            "ope" => {
                                if let Ok(tr) = Transaction::try_from(attributes) {
                                    db.mut_transactions().push(tr);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    if name.local_name == "homebank" {
                        in_info = false;
                    }
                }
                Ok(_) => {}
                Err(_) => {}
            }
        }

        Ok(db)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_hdb_props() {
        let observed = HomeBankDbProperties::empty();
        let expected = HomeBankDbProperties::new("", 1, 1, 1, 1);

        assert_eq!(expected, observed);
    }

    #[test]
    fn empty_hbdb_is_expected() {
        let observed = HomeBankDb::empty();
        let expected = HomeBankDb {
            // homebank_version: HomeBankDbVersion::empty(),
            properties: HomeBankDbProperties::empty(),
            currencies: HashMap::new(),
            groups: HashMap::new(),
            accounts: HashMap::new(),
            payees: HashMap::new(),
            categories: HashMap::new(),
            // favourites: vec![],
            transactions: vec![],
        };

        assert_eq!(expected, observed);
    }

    #[test]
    fn parse_empty_db() {
        let path = Path::new("tests/empty.xhb");
        let observed = HomeBankDb::try_from(path);
        let expected = HomeBankDb::empty();

        assert_eq!(Ok(expected), observed);
    }

    // #[test]
    // fn parse_minimal_db() {
    //     let path = Path::new("tests/minimal.xhb");
    //     let observed = HomeBankDb::try_from(path);
    //     let expected = HomeBankDb::empty();

    //     assert_eq!(Ok(expected), observed);
    // }
}
