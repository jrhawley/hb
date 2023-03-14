//! Review the sums across each (sub)category in your HomeBank database.

use crate::{transaction::sum_transactions, HomeBankDb, Query, QueryTransactions};
use super::{TODAY_FIRST_OF_MONTH_STR, FIRST_OF_NEXT_MONTH_STR};

use chrono::NaiveDate;
use clap::Parser;
use regex::Regex;
use std::str::FromStr;


/// Query the budget in your HomeBank database.
#[derive(Debug, Parser)]
pub struct QueryReview {
    /// Consider the budget from the month including this date.
    #[clap(
        short = 'd',
        long = "date-from",
        default_value = &TODAY_FIRST_OF_MONTH_STR,
        parse(try_from_str = NaiveDate::from_str),
        value_name = "date"
    )]
    date_from: NaiveDate,

    /// Consider the budget from the month up to and excluding this date.
    #[clap(
        short = 'D',
        long = "date-to",
        default_value = &FIRST_OF_NEXT_MONTH_STR,
        parse(try_from_str = NaiveDate::from_str),
        value_name = "date"
    )]
    date_to: NaiveDate,

    /// Exclude any (sub)categories that have no transactions.
    #[clap(short = 'x')]
    exclude_none: bool
}

impl QueryReview {
    /// Create a new query for budgets
    pub fn new(date_from: NaiveDate, date_to: NaiveDate, exclude_none: bool) -> Self {
        Self {
            date_from,
            date_to,
            exclude_none,
        }
    }

    /// Retrieve the earliest date that the budget is including
    fn date_from(&self) -> &NaiveDate {
        &self.date_from
    }

    /// Retrieve the latest date that the budget is including
    fn date_to(&self) -> &NaiveDate {
        &self.date_to
    }

    /// Retrieve whether some 
    fn excluded_none(&self) -> bool {
        self.exclude_none
    }
}

impl Query for QueryReview {
    type T = (String, Option<String>, f32);

    fn exec(&self, db: &HomeBankDb) -> Vec<Self::T> {
        let mut vals: Vec<(String, Option<String>, usize, f32)> = db.categories()
            .values()
            .map(|cat| {
                // create a regex from the category name (match the name exactly to exclude subcategories)
                let re_str = format!("^{}$", &cat.full_name(db));
                let re = Regex::from_str(&re_str).unwrap();

                // get all the transactions for that category
                let transaction_query = QueryTransactions::new(
                    &Some(*self.date_from()),
                    &Some(*self.date_to()),
                    &None,
                    &None,
                    &None,
                    &Some(re),
                    &None,
                    &None,
                    &None,
                    &None,
                    &None,
                    &None,
                    &None,
                );

                let filt_transactions = transaction_query.exec(db);
                let sum = sum_transactions(&filt_transactions);
                let cat_name = cat.name().to_string();

                let val = match cat.parent_name(db){
                    Some(parent_name) => {
                        (parent_name.to_string(), Some(cat_name), filt_transactions.len(), sum)
                    },
                    None => {
                        (cat_name, None, filt_transactions.len(), sum)
                    }
                };

                val
            })
            .collect();

        // sort by category name, then by subcategory name
        vals.sort_by(|a, b| if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        });

        // filter out any 0 categories, if desired
        if self.excluded_none() {
            vals.iter()
                .filter_map(|v| if v.2 == 0 {
                    None
                } else {
                    Some((v.0.clone(), v.1.clone(), v.3))
                })
                .collect()
        } else {
            vals.iter()
                .map(|v| (v.0.clone(), v.1.clone(), v.3))
                .collect()
        }
    }
}
