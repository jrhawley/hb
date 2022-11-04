//! Properties for the entire HomeBank database.

use std::str::FromStr;
use thiserror::Error;
use xml::attribute::OwnedAttribute;

/// Properties for the entire HomeBank database.
#[derive(Debug, PartialEq)]
pub struct HomeBankDbProperties {
    /// Title for the database.
    title: String,

    /// Key specifying the base [`Currency`][crate::currency::currency::Currency] that all conversion rates are calculated against.
    currency_key: usize,

    /// Key specifying the [`Category`][crate::category::category::Category] that contains [`Transaction`s][crate::transaction::transaction::Transaction] about your vehicle's (or vehicles') mileage and fuel consumption.
    car_category_key: usize,

    auto_smode: u8,
    auto_weekday: u8,
    auto_nbdays: u8,

    // Mode for how automatically scheduled transactions should be added.
    // sched_mode: ScheduleMode,
}

impl HomeBankDbProperties {
    /// Create an empty, default set of properties
    pub fn empty() -> Self {
        Self {
            title: String::from(""),
            currency_key: 1,
            car_category_key: 1,
            // sched_mode: ScheduleMode::NotCurrentlySet,
            auto_smode: 1,
            auto_weekday: 1,
            auto_nbdays: 0,
        }
    }

    /// Create a new properties object
    pub fn new(
        title: &str,
        currency: usize,
        car_category: usize,
        auto_smode: u8,
        auto_weekday: u8,
        auto_nbdays: u8,
    ) -> Self {
        Self {
            title: title.to_string(),
            currency_key: currency,
            car_category_key: car_category,
            auto_smode,
            auto_weekday,
            auto_nbdays,
        }
    }
}

impl Default for HomeBankDbProperties {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HomeBankDbPropertiesError {
    #[error("Invalid database title.")]
    InvalidTitle,
    #[error("Invalid database currency.")]
    InvalidCurrency,
    #[error("Invalid database vehicle category.")]
    InvalidVehicleCategory,
    #[error("Invalid database S mode.")]
    InvalidSMode,
    #[error("Invalid database week starting day.")]
    InvalidStartingWeekday,
}

impl TryFrom<Vec<OwnedAttribute>> for HomeBankDbProperties {
    type Error = HomeBankDbPropertiesError;

    fn try_from(v: Vec<OwnedAttribute>) -> Result<Self, Self::Error> {
        let mut props = Self::default();

        for i in v {
            match i.name.local_name.as_str() {
                "title" => {
                    props.title = i.value.to_string();
                }
                "curr" => {
                    props.currency_key = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(HomeBankDbPropertiesError::InvalidCurrency),
                    }
                }
                "car_category" => {
                    props.car_category_key = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(HomeBankDbPropertiesError::InvalidVehicleCategory),
                    }
                }
                "auto_smode" => {
                    props.auto_smode = match u8::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(HomeBankDbPropertiesError::InvalidSMode),
                    }
                }
                "auto_weekday" => {
                    props.auto_weekday = match u8::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(HomeBankDbPropertiesError::InvalidStartingWeekday),
                    }
                }
                "auto_nbdays" => {
                    props.auto_weekday = match u8::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(HomeBankDbPropertiesError::InvalidStartingWeekday),
                    }
                }
                _ => {}
            }
        }
        Ok(props)
    }
}

/// Default setting for how scheduled [`Transaction`][crate::transaction::transaction::Transaction] dates should be calculated.
pub enum ScheduleMode {
    /// Not currently set.
    /// This is used when creating a new HomeBank database, or when parsing the XML file for the first time.
    NotCurrentlySet,

    /// Add this many days in advance of the current date.
    Add(u8),

    /// Add days until you reach this day of each month (excluded).
    AddUntil(u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use xml::{reader::XmlEvent, EventReader};

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(2 + 2, result);
    }

    #[track_caller]
    fn check_try_from_single_str(input: &str, expected: Result<HomeBankDbProperties, HomeBankDbPropertiesError>) {
        // set up the reader from the input string
        let mut reader = EventReader::from_str(input);

        // skip the XML starting header and parse the first event
        let (_start, first) = (reader.next(), reader.next());

        // get the first event
        if let Ok(XmlEvent::StartElement {
            name, attributes, ..
        }) = first
        {
            if "properties" == name.local_name.as_str() {
                let observed = HomeBankDbProperties::try_from(attributes);
                assert_eq!(expected, observed);
            } else {
                panic!(
                    "Incorrect transaction string passed into check. Expected `ope`, found `{:#?}`",
                    name.local_name.as_str()
                );
            }
        } else {
            panic!("Incorrect string passed into check. `{:#?}`", first);
        }
    }

    #[test]
    fn check_title() {
        let input = r#"<properties title="DbTitle">"#;
        let expected = Ok(HomeBankDbProperties {
            title: "DbTitle".to_string(),
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }
    
    #[test]
    fn check_currency() {
        let input = r#"<properties curr="2">"#;
        let expected = Ok(HomeBankDbProperties {
            currency_key: 2,
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }
    
    #[test]
    fn check_car_cat() {
        let input = r#"<properties car_category="3">"#;
        let expected = Ok(HomeBankDbProperties {
            car_category_key: 3,
            ..Default::default()
        });

        check_try_from_single_str(input, expected);
    }
}