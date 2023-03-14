//! Currencies used within a HomeBank database.

use super::CurrencyError;
use crate::transaction::julian_date_from_u32;
use std::str::FromStr;
use chrono::NaiveDate;
use xml::attribute::OwnedAttribute;

/// Currencies used within a HomeBank database.
#[derive(Debug, PartialEq, Clone)]
pub struct Currency {
    /// The unique key for a currency in the database.
    key: usize,

    /// Flags on the currency.
    flags: usize,

    /// [ISO Currency Code](https://www.iso.org/iso-4217-currency-codes.html) for this currency.
    iso: String,

    /// The common name for this currency.
    name: String,

    /// The monetary symbol used for this currency, like `$` for the dollar or `€` for the Euro.
    symbol: char,

    /// Does the currency symbol prefix the amount?
    syprf: bool,
    
    /// What character separates the ordinal amount from its fractions?
    /// This is typically `.` or `,`, but may vary by locale.
    decimal_separator: char,

    /// What character provides a visual break between thousands digits?
    /// This is typically `,` or `.`, but may vary by locale.
    thousands_separator: char,

    /// How many digits should be displayed after the `decimal_separator`?
    decimal_len: usize,
    
    /// Conversion rate from this currency to the base currency specified in the [`HomeBankDbProperties`][crate::db::db_properties::HomeBankDbProperties].
    /// `conversion_rate` = `value in base currency` / `value in this currency`.
    conversion_rate: f32,

    /// The date when this currency's exchange rates were last updated.
    mdate: NaiveDate,
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
            syprf: false,
            decimal_separator: '.',
            thousands_separator: ' ',
            decimal_len: 2,
            conversion_rate: 1.0,
            mdate: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
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
                        Ok(0) => false,
                        Ok(1) => true,
                        Ok(_) => return Err(CurrencyError::InvalidSymbolPrefix),
                        Err(_) => return Err(CurrencyError::InvalidSymbolPrefix),
                    }
                }
                "frac" => {
                    curr.decimal_len = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(CurrencyError::InvalidDecimalLength),
                    }
                }
                "mdate" => {
                    curr.mdate = match u32::from_str(&i.value) {
                        Ok(d) => julian_date_from_u32(d),
                        Err(_) => return Err(CurrencyError::InvalidMDate),
                    };
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
