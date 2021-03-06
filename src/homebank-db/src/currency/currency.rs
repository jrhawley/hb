//! Currencies

use std::str::FromStr;
use xml::attribute::OwnedAttribute;

use super::CurrencyError;

#[derive(Debug, PartialEq, Clone)]
pub struct Currency {
    key: usize,
    flags: usize,
    iso: String,
    name: String,
    symbol: char,
    // I don't know what this is
    syprf: usize,
    decimal_separator: char,
    thousands_separator: char,
    decimal_len: usize,
    conversion_rate: f32,
    // I don't know what this is
    mdate: u8,
}

impl Currency {
    /// Create an empty, default set of properties
    pub fn empty() -> Self {
        Self {
            key: 0,
            flags: 0,
            iso: "".to_string(),
            name: "".to_string(),
            symbol: '$',
            syprf: 0,
            decimal_separator: '.',
            thousands_separator: ' ',
            decimal_len: 2,
            conversion_rate: 1.0,
            mdate: 0,
        }
    }

    /// Create a new properties object
    pub fn new(
        key: usize,
        flags: usize,
        iso: &str,
        name: &str,
        symbol: char,
        syprf: usize,
        decimal_separator: char,
        thousands_separator: char,
        decimal_len: usize,
        conversion_rate: f32,
        mdate: u8,
    ) -> Self {
        Self {
            key,
            flags,
            iso: iso.to_string(),
            name: name.to_string(),
            symbol: symbol,
            syprf,
            decimal_separator,
            thousands_separator,
            decimal_len,
            conversion_rate,
            mdate,
        }
    }

    /// Retrieve the `Currency` key
    pub(crate) fn key(&self) -> usize {
        self.key
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for Currency {
    fn default() -> Self {
        Self::empty()
    }
}

impl TryFrom<Vec<OwnedAttribute>> for Currency {
    type Error = CurrencyError;

    fn try_from(v: Vec<OwnedAttribute>) -> Result<Self, Self::Error> {
        let mut curr = Self::default();

        for i in v {
            match i.name.local_name.as_str() {
                "name" => {
                    curr.name = i.value.to_string();
                }
                "key" => {
                    curr.key = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(CurrencyError::InvalidKey),
                    }
                }
                "flags" => {
                    curr.flags = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(CurrencyError::InvalidFlags),
                    }
                }
                "syprf" => {
                    curr.syprf = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(CurrencyError::InvalidSyprf),
                    }
                }
                "frac" => {
                    curr.decimal_len = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(CurrencyError::InvalidDecimalLength),
                    }
                }
                "mdate" => {
                    curr.mdate = match u8::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(CurrencyError::InvalidMDate),
                    }
                }
                "iso" => {
                    curr.iso = i.value.to_string();
                }
                "symb" => {
                    curr.symbol = match i.value.chars().next() {
                        Some(c) => c,
                        None => return Err(CurrencyError::InvalidSymbol),
                    };
                }
                "dchar" => {
                    curr.decimal_separator = match i.value.chars().next() {
                        Some(c) => c,
                        None => return Err(CurrencyError::InvalidDecimalSeparator),
                    };
                }
                "gchar" => {
                    curr.thousands_separator = match i.value.chars().next() {
                        Some(c) => c,
                        None => return Err(CurrencyError::InvalidThousandsSeparator),
                    };
                }
                "rate" => {
                    curr.conversion_rate = match f32::from_str(&i.value) {
                        Ok(f) => f,
                        Err(_) => return Err(CurrencyError::InvalidConversionRate),
                    }
                }
                _ => {}
            }
        }
        Ok(curr)
    }
}
