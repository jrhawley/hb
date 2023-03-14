//! Categories for each [`Transaction`][crate::transaction::transaction_struct::Transaction].

pub mod budget_query;
pub mod category_struct;
pub mod category_budget;
pub mod category_error;
pub mod category_query;
pub mod review_query;

pub use budget_query::QueryBudget;
pub use category_struct::Category;
pub use category_budget::CategoryBudget;
pub use category_error::CategoryError;
pub use category_query::QueryCategories;
pub use review_query::QueryReview;

use chrono::{Datelike, Local, NaiveDate};
use kronos::{Grain, Grains, NthOf, TimeSequence};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TODAY: NaiveDate = Local::now().date_naive();
    pub static ref TODAY_FIRST_OF_MONTH: NaiveDate =
        NaiveDate::from_ymd_opt(TODAY.year(), TODAY.month(), 1).unwrap();
    pub static ref TODAY_FIRST_OF_MONTH_STR: String =
        TODAY_FIRST_OF_MONTH.format("%Y-%m-%d").to_string();
    pub static ref FIRST_OF_NEXT_MONTH: NaiveDate = {
        let first_of_month = NthOf(1, Grains(Grain::Day), Grains(Grain::Month));
        let mut date_iter = first_of_month.future(&TODAY_FIRST_OF_MONTH.and_hms_opt(0, 0, 0).unwrap());

        // skip the first month
        date_iter.next();

        // save the next month
         date_iter.next().unwrap().start.date()
    };
    pub static ref FIRST_OF_NEXT_MONTH_STR: String =
        FIRST_OF_NEXT_MONTH.format("%Y-%m-%d").to_string();
}
