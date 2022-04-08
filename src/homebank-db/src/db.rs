//! Data structure for the HomeBank database.

use crate::{HomeBankDbError, HomeBankDbProperties, Transaction};
use std::{fs::File, io::BufReader, path::Path};
use xml::{reader::XmlEvent, EventReader};

#[derive(Debug, PartialEq)]
pub struct HomeBankDb {
    // #[serde(rename = "homebank")]
    // homebank_version: HomeBankDbVersion,
    properties: HomeBankDbProperties,
    // currencies: Vec<Currency>,
    // groups: Vec<Group>,
    // accounts: Vec<Account>,
    // payees: Vec<Payee>,
    // categories: Vec<Category>,
    // favourites: Vec<Favourite>,
    transactions: Vec<Transaction>,
}

impl HomeBankDb {
    /// Create an empty, default, HomeBank database
    fn empty() -> Self {
        Self {
            // homebank_version: HomeBankDbVersion::empty(),
            properties: HomeBankDbProperties::empty(),
            // currencies: vec![],
            // groups: vec![],
            // accounts: vec![],
            // payees: vec![],
            // categories: vec![],
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
                            "cur" => {}
                            "grp" => {}
                            "account" => {}
                            "pay" => {}
                            "cat" => {}
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
            // currencies: vec![],
            // groups: vec![],
            // accounts: vec![],
            // payees: vec![],
            // categories: vec![],
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
