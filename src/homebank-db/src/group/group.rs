//! The group that an [`Account`][crate::account::account::Account] belongs to.
//! 
//! Either `Active` or `Archived`.

use super::GroupError;
use std::str::FromStr;
use xml::attribute::OwnedAttribute;

#[derive(Debug, PartialEq, Clone)]
pub struct Group {
    key: usize,
    name: String,
}

impl Group {
    /// Create the empty, default `Group`
    pub fn empty() -> Self {
        Self {
            key: 0,
            name: "".to_string(),
        }
    }

    /// Create a new `Group`
    pub fn new(key: usize, name: &str) -> Self {
        Self {
            key,
            name: name.to_string(),
        }
    }

    /// Retrieve the key for the `Group`
    pub(crate) fn key(&self) -> usize {
        self.key
    }

    /// Retrieve the name of the `Group`
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for Group {
    fn default() -> Self {
        Self::empty()
    }
}

impl TryFrom<Vec<OwnedAttribute>> for Group {
    type Error = GroupError;

    fn try_from(v: Vec<OwnedAttribute>) -> Result<Self, Self::Error> {
        let mut grp = Self::default();

        for i in v {
            match i.name.local_name.as_str() {
                "key" => {
                    grp.key = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(GroupError::InvalidKey),
                    }
                }
                "name" => {
                    grp.name = i.value.as_str().to_string();
                }
                _ => {}
            }
        }
        Ok(grp)
    }
}
