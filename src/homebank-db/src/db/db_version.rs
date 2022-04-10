//! Version information for the HomeBank database

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct HomeBankDbVersion {
    #[serde(rename = "v")]
    version: String,
    #[serde(rename = "d")]
    date: String,
}

impl HomeBankDbVersion {
    /// Create an empty, default set of properties
    pub fn empty() -> Self {
        Self {
            // version: Version::new(1, 4, 0),
            version: String::from("1.3999999999999999"),
            date: String::from("050504"),
        }
    }
}

impl Default for HomeBankDbVersion {
    fn default() -> Self {
        Self::empty()
    }
}
