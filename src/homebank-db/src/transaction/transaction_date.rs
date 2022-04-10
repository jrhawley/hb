//! Helper functions to handle the processing of dates

use chrono::{Duration, NaiveDate};

pub(crate) fn julian_date_from_u32(d: u32) -> NaiveDate {
    // dates are stored as Julian dates, starting from 0001-01-01
    let julian_zero = NaiveDate::from_ymd(1, 1, 1);
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
    fn unix_epoch_beginning() {
        let input = 719163;
        let expected = NaiveDate::from_ymd(1970, 01, 01);

        check_date_conversion(input, expected);
    }
}
