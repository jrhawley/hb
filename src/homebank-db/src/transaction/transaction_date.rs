//! Helper functions to handle the processing of dates

use chrono::{Duration, NaiveDate};
use std::cmp::{max, min};

pub(crate) fn clamp_date(d: NaiveDate) -> NaiveDate {
    // from HomeBank source code
    // listed as 693596 aka 1900-01-01
    let hb_min_date: NaiveDate = NaiveDate::from_ymd(1900, 01, 01);
    // list as 803533 aka 2200-12-31
    let hb_max_date: NaiveDate = NaiveDate::from_ymd(2200, 12, 31);
    max(min(d, hb_max_date), hb_min_date)
}

pub(crate) fn julian_date_from_u32(d: u32) -> NaiveDate {
    // dates are stored as Julian dates, starting from 0001-01-01
    // i.e. 0001-01-01 is day 1, so we start from the previous day to avoid off-by-1 errors
    let julian_zero = NaiveDate::from_ymd(0000, 12, 31);
    clamp_date(julian_zero + Duration::days(d.into()))
}

pub(crate) fn unclamped_julian_date_from_u32(d: u32) -> NaiveDate {
    // dates are stored as Julian dates, starting from 0001-01-01
    // i.e. 0001-01-01 is day 1, so we start from the previous day to avoid off-by-1 errors
    let julian_zero = NaiveDate::from_ymd(0000, 12, 31);
    julian_zero + Duration::days(d.into())
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
        let expected = NaiveDate::from_ymd(1900, 01, 01);

        check_date_conversion(input, expected);
    }

    #[test]
    fn convert_max_date() {
        let input = 803533;
        let expected = NaiveDate::from_ymd(2200, 12, 31);

        check_date_conversion(input, expected);
    }

    #[test]
    fn convert_unix_epoch_beginning() {
        let input = 719163;
        let expected = NaiveDate::from_ymd(1970, 01, 01);

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
        let expected = NaiveDate::from_ymd(1900, 01, 01);

        check_clamp_date(input, expected);
    }

    #[test]
    fn convert_date_equal_to_min() {
        let input = 693596;
        let expected = NaiveDate::from_ymd(1900, 01, 01);

        check_clamp_date(input, expected);
    }

    #[test]
    fn convert_date_between_bounds() {
        let input = 693597;
        let expected = NaiveDate::from_ymd(1900, 01, 02);

        check_clamp_date(input, expected);
    }

    #[test]
    fn convert_date_equal_to_upper() {
        let input = 803533;
        let expected = NaiveDate::from_ymd(2200, 12, 31);

        check_clamp_date(input, expected);
    }

    #[test]
    fn convert_date_grater_than_upper() {
        let input = 803534;
        let expected = NaiveDate::from_ymd(2200, 12, 31);

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
        let expected = NaiveDate::from_ymd(1899, 09, 27);

        check_unclamped_date(input, expected);
    }

    #[test]
    fn convert_date_equal_to_min_unclamped() {
        let input = 693596;
        let expected = NaiveDate::from_ymd(1900, 01, 01);

        check_unclamped_date(input, expected);
    }

    #[test]
    fn convert_date_between_bounds_unclamped() {
        let input = 693597;
        let expected = NaiveDate::from_ymd(1900, 01, 02);

        check_unclamped_date(input, expected);
    }

    #[test]
    fn convert_date_equal_to_upper_unclamped() {
        let input = 803533;
        let expected = NaiveDate::from_ymd(2200, 12, 31);

        check_unclamped_date(input, expected);
    }

    #[test]
    fn convert_date_grater_than_upper_unclamped() {
        let input = 803534;
        let expected = NaiveDate::from_ymd(2201, 01, 01);

        check_unclamped_date(input, expected);
    }
}
