//! Options for filtering [`Categories`][crate::category::category::Category] from the [`HomeBankDb`].

use crate::{db::HomeBankDb, query::Query};
use super::Category;
use clap::Parser;
use regex::Regex;

/// Options for filtering [`Categories`][crate::category::category::Category] from the [`HomeBankDb`].
#[derive(Debug, Parser)]
#[clap(
    name = "categories",
    visible_alias = "c",
    about = "Query transaction categories"
)]
pub struct QueryCategories {
    /// Name of the [`Category`][crate::category::category::Category]
    #[clap(value_name = "regex")]
    name: Option<Regex>,
}

impl QueryCategories {
    /// Retrieve the regular expression for the `Category` name
    fn name(&self) -> &Option<Regex> {
        &self.name
    }
}

impl Query for QueryCategories {
    type T = Category;

    fn exec(&self, db: &HomeBankDb) -> Vec<Self::T> {
        let mut filt_categories: Vec<Category> = db
            .categories()
            .values()
            // filter out categories that don't match the regex
            .filter(|&p| match self.name() {
                Some(re) => re.is_match(&p.full_name(db)),
                None => true,
            })
            .cloned()
            .collect();

        filt_categories.sort_by_key(|a| a.full_name(db));

        filt_categories
    }
}
