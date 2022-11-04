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

    /// Mode for how automatically scheduled transactions should be added.
    sched_mode: ScheduleMode,
}

impl HomeBankDbProperties {
    /// Create an empty, default set of properties
    pub fn empty() -> Self {
        Self {
            title: String::from(""),
            currency_key: 1,
            car_category_key: 1,
            sched_mode: ScheduleMode::NotCurrentlySet(None, None),
        }
    }

    /// Create a new properties object
    pub fn new(
        title: &str,
        currency: usize,
        car_category: usize,
        sched_mode: ScheduleMode,
    ) -> Self {
        Self {
            title: title.to_string(),
            currency_key: currency,
            car_category_key: car_category,
            sched_mode,
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
    #[error("Invalid default scheduling mode.")]
    InvalidDefaultSchedulingMode,
    #[error("Invalid default scheduling mode week day.")]
    InvalidDefaultSchedulingModeWeekday,
    #[error("Invalid number of days in advance for default scheduling mode.")]
    InvalidDefaultSchedulingModeDaysInAdvance,
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
                    match (u8::from_str(&i.value), props.sched_mode) {
                        (Ok(0), ScheduleMode::NotCurrentlySet(None, _)) => {
                            // setting the default `AddUntil` time to 1 (first day of the month)
                            props.sched_mode = ScheduleMode::AddUntil(1);
                        },
                        (Ok(0), ScheduleMode::NotCurrentlySet(Some(adduntil_val), _)) => {
                            props.sched_mode = ScheduleMode::AddUntil(adduntil_val);
                        },
                        (Ok(1), ScheduleMode::NotCurrentlySet(_, None)) => {
                            // setting the default `Add` time to 0
                            props.sched_mode = ScheduleMode::Add(0);
                        },
                        (Ok(1), ScheduleMode::NotCurrentlySet(_, Some(add_val))) => {
                            props.sched_mode = ScheduleMode::Add(add_val);
                        },
                        _ => {
                            // any other value that's stored here should be considered incorrect
                            return Err(HomeBankDbPropertiesError::InvalidDefaultSchedulingMode)
                        },
                    }
                }
                "auto_weekday" => {
                    match (u8::from_str(&i.value), props.sched_mode) {
                        (Ok(idx), ScheduleMode::NotCurrentlySet(_, add_val)) => {
                            // store the value temporarily until you potentially run across the `auto_smode`
                            props.sched_mode = ScheduleMode::NotCurrentlySet(Some(idx), add_val);
                        },
                        (Ok(idx), ScheduleMode::AddUntil(_)) => {
                            // update the value
                            props.sched_mode = ScheduleMode::AddUntil(idx);
                        },
                        (Ok(_), ScheduleMode::Add(add_val)) => {
                            // don't overwrite the value, since the other mode has already been specified
                            props.sched_mode = ScheduleMode::Add(add_val);
                        },
                        (Err(_), _) => return Err(HomeBankDbPropertiesError::InvalidDefaultSchedulingModeWeekday),
                    }
                }
                "auto_nbdays" => {
                    match (u8::from_str(&i.value), props.sched_mode) {
                        (Ok(idx), ScheduleMode::NotCurrentlySet(adduntil_val, _)) => {
                            // store the value temporarily until you potentially run across the `auto_smode`
                            props.sched_mode = ScheduleMode::NotCurrentlySet(adduntil_val, Some(idx));
                        },
                        (Ok(_), ScheduleMode::AddUntil(adduntil_val)) => {
                            // don't overwrite the value, since the other mode has already been specified
                            props.sched_mode = ScheduleMode::AddUntil(adduntil_val);
                        },
                        (Ok(idx), ScheduleMode::Add(_)) => {
                            // update the value
                            props.sched_mode = ScheduleMode::Add(idx);
                        },
                        (Err(_), _) => return Err(HomeBankDbPropertiesError::InvalidDefaultSchedulingModeDaysInAdvance),
                    }
                }
                _ => {}
            }
        }
        Ok(props)
    }
}

/// Default setting for how scheduled [`Transaction`][crate::transaction::transaction::Transaction] dates should be calculated.
#[derive(Debug, PartialEq, Eq)]
pub enum ScheduleMode {
    /// Not currently set.
    /// This is used when creating a new HomeBank database, or when parsing the XML file for the first time.
    /// The first element stores the `auto_weekday` value that is used in `AddUntil` variant.
    /// The second element stores the `auto_nbdays` value that is used in `Add` variant.
    NotCurrentlySet(Option<u8>, Option<u8>),

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
    fn check_try_from_single_str(input: &str, expected: &Result<HomeBankDbProperties, HomeBankDbPropertiesError>) {
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
                assert_eq!(*expected, observed);
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

        check_try_from_single_str(input, &expected);
    }
    
    #[test]
    fn check_currency() {
        let input = r#"<properties curr="2">"#;
        let expected = Ok(HomeBankDbProperties {
            currency_key: 2,
            ..Default::default()
        });

        check_try_from_single_str(input, &expected);
    }
    
    #[test]
    fn check_car_cat() {
        let input = r#"<properties car_category="3">"#;
        let expected = Ok(HomeBankDbProperties {
            car_category_key: 3,
            ..Default::default()
        });

        check_try_from_single_str(input, &expected);
    }

    #[test]
    fn check_smode_add_0() {
        // each of these inputs should produce the same result
        let inputs = vec![
            r#"<properties auto_smode="1">"#,
            r#"<properties auto_smode="1" auto_nbdays="0">"#,
            r#"<properties auto_smode="1" auto_weekday="1">"#,
            r#"<properties auto_smode="1" auto_weekday="1" auto_nbdays="0">"#,
        ];
        let expected = Ok(HomeBankDbProperties {
            sched_mode: ScheduleMode::Add(0),
            ..Default::default()
        });

        for input in inputs {
            check_try_from_single_str(input, &expected);
        }

    }

    #[test]
    fn check_smode_add_2() {
        // each of these inputs should produce the same result
        let inputs = vec![
            r#"<properties auto_smode="1" auto_nbdays="2">"#,
            r#"<properties auto_smode="1" auto_weekday="1" auto_nbdays="2">"#,
        ];
        let expected = Ok(HomeBankDbProperties {
            sched_mode: ScheduleMode::Add(2),
            ..Default::default()
        });

        for input in inputs {
            check_try_from_single_str(input, &expected);
        }
    }

    #[test]
    fn check_smode_adduntil_1() {
        // each of these inputs should produce the same result
        let inputs = vec![
            r#"<properties auto_smode="0" auto_weekday="1">"#,
            r#"<properties auto_smode="0" auto_weekday="1" auto_nbdays="3">"#,
        ];
        let expected = Ok(HomeBankDbProperties {
            sched_mode: ScheduleMode::AddUntil(1),
            ..Default::default()
        });

        for input in inputs {
            check_try_from_single_str(input, &expected);
        }
    }
}
