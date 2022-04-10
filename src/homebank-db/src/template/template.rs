//! Favourites

use crate::Transaction;
use chrono::NaiveDate;
use kronos::Shim;

pub struct Template<'a> {
    transaction: Transaction,
    next_date: NaiveDate,
    period: Shim<'a>,
    weekend_shift: usize,
}
