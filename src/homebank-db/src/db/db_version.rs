//! Version information for the HomeBank database

use super::HomeBankDbError;
use crate::transaction::julian_date_from_u32;
use chrono::NaiveDate;
use std::str::FromStr;
use xml::attribute::OwnedAttribute;

#[derive(Debug, PartialEq)]
pub struct HomeBankDbVersion {
    version: String,
    date: NaiveDate,
}

impl HomeBankDbVersion {
    /// Create an empty, default set of properties
    pub fn empty() -> Self {
        Self {
            // version: Version::new(1, 4, 0),
            version: String::from("1.3999999999999999"),
            date: julian_date_from_u32(050504),
        }
    }
}

impl Default for HomeBankDbVersion {
    fn default() -> Self {
        Self::empty()
    }
}

impl TryFrom<Vec<OwnedAttribute>> for HomeBankDbVersion {
    type Error = HomeBankDbError;

    fn try_from(v: Vec<OwnedAttribute>) -> Result<Self, Self::Error> {
        let mut db_ver = Self::default();

        for i in v {
            match i.name.local_name.as_str() {
                "v" => db_ver.version = i.value.to_string(),
                "d" => match u32::from_str(&i.value) {
                    Ok(d) => db_ver.date = julian_date_from_u32(d),
                    Err(_) => return Err(HomeBankDbError::InvalidDate),
                },
                _ => {}
            }
        }

        Ok(db_ver)
    }
}
