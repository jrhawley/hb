//! Helper functions to for processing dates.

use chrono::{Duration, NaiveDate};
use lazy_static::lazy_static;
use std::cmp::{max, min};

lazy_static!{
    /// The minimum supported date (from HomeBank source code).
    /// Equivalent to 1900-01-01 (and stored in the database XML as 693596).
    pub static ref HB_MIN_DATE: NaiveDate = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

    /// The maximum supported date (from HomeBank source code).
    /// Equivalent to 2200-12-31 (and stored in the database XML as 803533).
    pub static ref HB_MAX_DATE: NaiveDate = NaiveDate::from_ymd_opt(2200, 12, 31).unwrap();

    /// The Julian-encoded day 0.
    /// Dates in the [`HomeBankDb`][crate::db::db_struct::HomeBankDb] are stored as [Julian dates](https://en.wikipedia.org/wiki/Julian_calendar), with day 1 being 0001-01-01.
    /// We start from the previous day to avoid off-by-1 errors in calculations.
    pub static ref JULIAN_ZERO: NaiveDate = NaiveDate::from_ymd_opt(0000, 12, 31).unwrap();
}

/// Clamp a date between the minimum ([`struct@HB_MIN_DATE`]) and maximum ([`struct@HB_MAX_DATE`]) dates supported by HomeBank.
pub(crate) fn clamp_date(d: NaiveDate) -> NaiveDate {
    max(min(d, *HB_MAX_DATE), *HB_MIN_DATE)
}

/// Convert a date from the Julian format (encoded as days since [`struct@JULIAN_ZERO`]) into a [`NaiveDate`].
/// This will also clamp the date as described by [`clamp_date`].
pub(crate) fn julian_date_from_u32(d: u32) -> NaiveDate {
    clamp_date(*JULIAN_ZERO + Duration::days(d.into()))
}

/// Convert a date from the Julian format (encoded as days since [`struct@JULIAN_ZERO`]) into a [`NaiveDate`].
/// This date is unbounded and does not necessarily fall between [`struct@HB_MIN_DATE`] and [`struct@HB_MAX_DATE`].
pub(crate) fn unclamped_julian_date_from_u32(d: u32) -> NaiveDate {
    *JULIAN_ZERO + Duration::days(d.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(2 + 2, result);
    }

    #[track_caller]
    fn check_date_conversion(input: u32, expected: NaiveDate) {
        let observed = julian_date_from_u32(input);

        assert_eq!(expected, observed);
    }

    #[test]
    fn convert_min_date() {
        let input = 693596;
        let expected = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        check_date_conversion(input, expected);
    }

    #[test]
    fn convert_max_date() {
        let input = 803533;
        let expected = NaiveDate::from_ymd_opt(2200, 12, 31).unwrap();

        check_date_conversion(input, expected);
    }

    #[test]
    fn convert_unix_epoch_beginning() {
        let input = 719163;
        let expected = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

        check_date_conversion(input, expected);
    }

    #[track_caller]
    fn check_clamp_date(input: u32, expected: NaiveDate) {
        let observed = julian_date_from_u32(input);

        assert_eq!(expected, observed);
    }

    #[test]
    fn convert_date_prior_to_min() {
        let input = 693500;
        let expected = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        check_clamp_date(input, expected);
    }

    #[test]
    fn convert_date_equal_to_min() {
        let input = 693596;
        let expected = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        check_clamp_date(input, expected);
    }

    #[test]
    fn convert_date_between_bounds() {
        let input = 693597;
        let expected = NaiveDate::from_ymd_opt(1900, 1, 2).unwrap();

        check_clamp_date(input, expected);
    }

    #[test]
    fn convert_date_equal_to_upper() {
        let input = 803533;
        let expected = NaiveDate::from_ymd_opt(2200, 12, 31).unwrap();

        check_clamp_date(input, expected);
    }

    #[test]
    fn convert_date_grater_than_upper() {
        let input = 803534;
        let expected = NaiveDate::from_ymd_opt(2200, 12, 31).unwrap();

        check_clamp_date(input, expected);
    }

    #[track_caller]
    fn check_unclamped_date(input: u32, expected: NaiveDate) {
        let observed = unclamped_julian_date_from_u32(input);

        assert_eq!(expected, observed);
    }

    #[test]
    fn convert_date_prior_to_min_unclamped() {
        let input = 693500;
        let expected = NaiveDate::from_ymd_opt(1899, 9, 27).unwrap();

        check_unclamped_date(input, expected);
    }

    #[test]
    fn convert_date_equal_to_min_unclamped() {
        let input = 693596;
        let expected = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        check_unclamped_date(input, expected);
    }

    #[test]
    fn convert_date_between_bounds_unclamped() {
        let input = 693597;
        let expected = NaiveDate::from_ymd_opt(1900, 1, 2).unwrap();

        check_unclamped_date(input, expected);
    }

    #[test]
    fn convert_date_equal_to_upper_unclamped() {
        let input = 803533;
        let expected = NaiveDate::from_ymd_opt(2200, 12, 31).unwrap();

        check_unclamped_date(input, expected);
    }

    #[test]
    fn convert_date_grater_than_upper_unclamped() {
        let input = 803534;
        let expected = NaiveDate::from_ymd_opt(2201, 1, 1).unwrap();

        check_unclamped_date(input, expected);
    }
}
