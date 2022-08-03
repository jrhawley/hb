//! Query the budget in your HomeBank database.

use crate::{transaction::sum_transactions, Category, HomeBankDb, Query, QueryTransactions};
use chrono::{Datelike, Local, NaiveDate};
use clap::Parser;
use kronos::{Grain, Grains, NthOf, TimeSequence};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    pub static ref TODAY: NaiveDate = Local::today().naive_local();
    pub static ref TODAY_FIRST_OF_MONTH: NaiveDate =
        NaiveDate::from_ymd(TODAY.year(), TODAY.month(), 1);
    pub static ref TODAY_FIRST_OF_MONTH_STR: String =
        TODAY_FIRST_OF_MONTH.format("%Y-%m-%d").to_string();
    pub static ref FIRST_OF_NEXT_MONTH: NaiveDate = {
        let first_of_month = NthOf(1, Grains(Grain::Day), Grains(Grain::Month));
        let mut date_iter = first_of_month.future(&TODAY_FIRST_OF_MONTH.and_hms(0, 0, 0));

        // skip the first month
        date_iter.next();

        // save the next month
        let first_of_next_month = date_iter
            .next()
            .unwrap()
            .start
            .date();

        first_of_next_month
    };
    pub static ref FIRST_OF_NEXT_MONTH_STR: String =
        FIRST_OF_NEXT_MONTH.format("%Y-%m-%d").to_string();
}

#[derive(Debug, Parser)]
pub struct QueryBudget {
    #[clap(help = "Name of the category", value_name = "regex")]
    name: Option<Regex>,

    #[clap(
        short = 'd',
        long = "date-from",
        help = "Consider the budget from the month including this date",
        default_value = &TODAY_FIRST_OF_MONTH_STR,
        parse(try_from_str = NaiveDate::from_str),
        value_name = "date"
    )]
    date_from: NaiveDate,

    #[clap(
        short = 'D',
        long = "date-to",
        help = "Consider the budget from the month up to and excluding this date",
        default_value = &FIRST_OF_NEXT_MONTH_STR,
        parse(try_from_str = NaiveDate::from_str),
        value_name = "date"
    )]
    date_to: NaiveDate,
}

impl QueryBudget {
    /// Create a new query for budgets
    pub fn new(name: Option<Regex>, date_from: NaiveDate, date_to: NaiveDate) -> Self {
        Self {
            name,
            date_from,
            date_to,
        }
    }

    /// Retrieve the regular expression for the `Category` name
    fn name(&self) -> &Option<Regex> {
        &self.name
    }

    /// Retrieve the earliest date that the budget is including
    fn date_from(&self) -> &NaiveDate {
        &self.date_from
    }

    /// Retrieve the latest date that the budget is including
    fn date_to(&self) -> &NaiveDate {
        &self.date_to
    }
}

pub struct BudgetSummary {
    name: String,
    progress: f32,
    allotment: Option<f32>,
    progress_frac: Option<f32>,
}

impl BudgetSummary {
    /// Create a new budget summary
    pub fn new(name: &str, progress: f32, allotment: Option<f32>) -> Self {
        Self {
            name: name.to_string(),
            progress,
            allotment,
            progress_frac: match allotment {
                Some(val) => Some(progress / val),
                None => None,
            },
        }
    }

    /// Retrieve the name of the `Category` to which the budget applies
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Retrieve the progress of the budget
    pub fn progress(&self) -> f32 {
        self.progress
    }

    /// Retrieve the progress of the budget, made positive, and rounded to the nearest integer
    pub fn progress_rounded(&self) -> u64 {
        self.progress.abs() as u64
    }

    /// Retrieve the progress of the budget
    pub fn progress_frac(&self) -> &Option<f32> {
        &self.progress_frac
    }

    /// Retrieve the allotment for the budget
    pub fn allotment(&self) -> Option<f32> {
        self.allotment
    }

    /// Retrieve the allotment for the budget, made positive, and rounded to the nearest integer
    pub fn allotment_rounded(&self) -> Option<u64> {
        match self.allotment {
            Some(val) => Some(val.abs() as u64),
            None => None,
        }
    }

    /// Helper function to determine if there is a budget or not
    pub fn has_allotment(&self) -> bool {
        self.allotment.is_some()
    }
}

impl Query for QueryBudget {
    type T = BudgetSummary;

    fn exec(&self, db: &HomeBankDb) -> Vec<Self::T> {
        let mut filt_categories: Vec<Category> = db
            .categories()
            .values()
            // filter out categories that don't match the regex
            .filter(|&cat| match self.name() {
                Some(re) => re.is_match(&cat.full_name(db)),
                None => true,
            })
            // filter out categories that don't have a budget
            .filter(|&cat| cat.has_budget())
            .map(|cat| cat.clone())
            .collect();

        filt_categories.sort_by(|a, b| a.full_name(db).cmp(&b.full_name(db)));

        let budget_spent: Vec<BudgetSummary> = filt_categories
            .iter()
            .map(|cat| {
                let cat_name_re = Regex::new(&cat.full_name(db)).unwrap();
                let transaction_query = QueryTransactions::new(
                    &Some(*self.date_from()),
                    &Some(*self.date_to()),
                    &None,
                    &None,
                    &None,
                    &Some(cat_name_re),
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
                let allotment = cat.budget_amount_over_interval(*self.date_from(), *self.date_to());

                BudgetSummary::new(&cat.full_name(db), sum, allotment)
            })
            .collect();

        budget_spent
    }
}
