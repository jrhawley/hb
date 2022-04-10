//! Helper functions to handle the processing of dates

use chrono::{Duration, NaiveDate};

pub(crate) fn julian_date_from_u32(d: u32) -> NaiveDate {
    // dates are stored as Julian dates, starting from 0001-01-01
    let julian_zero = NaiveDate::from_ymd(1, 1, 1);
    julian_zero + Duration::days(d.into())
}
