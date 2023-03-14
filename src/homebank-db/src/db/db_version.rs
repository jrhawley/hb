//! Version information for the HomeBank database.

use super::HomeBankDbError;
use crate::transaction::{julian_date_from_u32, transaction_date::unclamped_julian_date_from_u32};
use chrono::NaiveDate;
use semver::Version;
use std::str::FromStr;
use xml::attribute::OwnedAttribute;

/// Version information for the HomeBank database.
#[derive(Debug, PartialEq)]
pub struct HomeBankDbSchema {
    version: Version,
    date: NaiveDate,
}

impl HomeBankDbSchema {
    /// Create an empty, default set of properties
    pub fn empty() -> Self {
        Self {
            version: Version::new(0, 0, 1),
            date: julian_date_from_u32(050504),
        }
    }
}

impl Default for HomeBankDbSchema {
    fn default() -> Self {
        Self::empty()
    }
}

impl TryFrom<Vec<OwnedAttribute>> for HomeBankDbSchema {
    type Error = HomeBankDbError;

    fn try_from(v: Vec<OwnedAttribute>) -> Result<Self, Self::Error> {
        let mut db_ver = Self::default();

        for i in v {
            match i.name.local_name.as_str() {
                "v" => {
                    // The version is stored internally as a f32 type, but when
                    // it's written out in text, it carries all the floating points.
                    // This leads to a not nicely-formatted value that needs to be parsed manually.
                    db_ver.version = match parse_version_string(&i.value) {
                        Ok(ver) => ver,
                        Err(e) => return Err(e),
                    }
                }
                "d" => match u32::from_str(&i.value) {
                    Ok(d) => db_ver.date = unclamped_julian_date_from_u32(d),
                    Err(_) => return Err(HomeBankDbError::InvalidDate),
                },
                _ => {}
            }
        }

        Ok(db_ver)
    }
}

fn parse_version_string(s: &str) -> Result<Version, HomeBankDbError> {
    match f32::from_str(s) {
        Ok(f) => {
            let major = f.floor() as u64;
            // note: this won't work if the minor version is > 10
            // since the version is stored as a float, I don't thin this will be a problem
            let minor = (10.0 * f.fract()).round() as u64;

            Ok(Version::new(major, minor, 0))
        }
        Err(_) => Err(HomeBankDbError::InvalidVersion),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xml::{reader::XmlEvent, EventReader};

    #[track_caller]
    fn check_try_from_single_str(input: &str, expected: Result<HomeBankDbSchema, HomeBankDbError>) {
        // set up the reader from the input string
        let mut reader = EventReader::from_str(input);

        // skip the XML starting header and parse the first event
        let (_start, first) = (reader.next(), reader.next());

        // get the first event
        if let Ok(XmlEvent::StartElement {
            name, attributes, ..
        }) = first
        {
            if "homebank" == name.local_name.as_str() {
                let observed = HomeBankDbSchema::try_from(attributes);
                assert_eq!(expected, observed);
            } else {
                panic!(
                    "Incorrect HomeBank string passed into check. Expected `homebank`, found `{:#?}`",
                    name.local_name.as_str()
                );
            }
        } else {
            panic!("Incorrect string passed into check. `{:#?}`", first);
        }
    }

    #[test]
    fn parse_version() {
        let input = r#"<homebank v="1.3999999999999999">"#;
        let expected = Ok(HomeBankDbSchema {
            version: Version::new(1, 4, 0),
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }
}
