//! Categories

use crate::CategoryError;
use std::str::FromStr;
use xml::attribute::OwnedAttribute;

#[derive(Debug, PartialEq)]
pub struct Category {
    key: usize,
    flags: usize,
    name: String,
    // I don't know what this is
    b: Vec<f32>,
    parent_key: Option<usize>,
}

impl Category {
    pub(crate) fn empty() -> Self {
        Self {
            key: 0,
            flags: 0,
            name: "".to_string(),
            b: vec![],
            parent_key: None,
        }
    }

    pub(crate) fn new(
        key: usize,
        flags: usize,
        name: &str,
        b: &Vec<f32>,
        parent_key: Option<usize>,
    ) -> Self {
        Self {
            key,
            flags,
            name: name.to_string(),
            b: b.clone(),
            parent_key,
        }
    }

    pub(crate) fn key(&self) -> usize {
        self.key
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn flags(&self) -> usize {
        self.flags
    }
}

impl Default for Category {
    fn default() -> Self {
        Self::empty()
    }
}

impl TryFrom<Vec<OwnedAttribute>> for Category {
    type Error = CategoryError;

    fn try_from(v: Vec<OwnedAttribute>) -> Result<Self, Self::Error> {
        let mut cat = Self::default();

        for i in v {
            match i.name.local_name.as_str() {
                "name" => {
                    cat.name = i.value.to_string();
                }
                "key" => {
                    cat.key = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(CategoryError::InvalidKey),
                    }
                }
                "flags" => {
                    cat.flags = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(CategoryError::InvalidFlags),
                    }
                }
                "parent" => {
                    cat.parent_key = match usize::from_str(&i.value) {
                        Ok(idx) => Some(idx),
                        Err(_) => return Err(CategoryError::InvalidParentKey),
                    }
                }
                _ => {}
            }
        }
        Ok(cat)
    }
}
