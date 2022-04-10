//! Properties for the entire database

use std::str::FromStr;
use thiserror::Error;
use xml::attribute::OwnedAttribute;

#[derive(Debug, PartialEq)]
pub struct HomeBankDbProperties {
    title: String,
    currency_idx: usize,
    car_category_idx: usize,
    auto_smode: usize,
    auto_weekday: usize,
}

impl HomeBankDbProperties {
    /// Create an empty, default set of properties
    pub(crate) fn empty() -> Self {
        Self {
            title: String::from(""),
            currency_idx: 1,
            car_category_idx: 1,
            auto_smode: 1,
            auto_weekday: 1,
        }
    }

    /// Create a new properties object
    pub(crate) fn new(
        title: &str,
        currency: usize,
        car_category: usize,
        auto_smode: usize,
        auto_weekday: usize,
    ) -> Self {
        Self {
            title: title.to_string(),
            currency_idx: currency,
            car_category_idx: car_category,
            auto_smode,
            auto_weekday,
        }
    }
}

impl Default for HomeBankDbProperties {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Error)]
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
                    props.currency_idx = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(HomeBankDbPropertiesError::InvalidCurrency),
                    }
                }
                "car_category" => {
                    props.car_category_idx = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(HomeBankDbPropertiesError::InvalidVehicleCategory),
                    }
                }
                "auto_smode" => {
                    props.auto_smode = match usize::from_str(&i.value) {
                        Ok(idx) => idx,
                        Err(_) => return Err(HomeBankDbPropertiesError::InvalidSMode),
                    }
                }
                "auto_weekday" => {
                    props.auto_weekday = match usize::from_str(&i.value) {
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
