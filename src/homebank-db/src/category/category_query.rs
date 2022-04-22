use super::Category;
use crate::{db::HomeBankDb, query::Query};
use regex::Regex;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "categories",
    visible_alias = "c",
    about = "Query transaction categories"
)]
pub struct QueryCategories {
    #[structopt(help = "Name of the category", value_name = "regex")]
    name: Option<Regex>,
}

impl QueryCategories {
    /// Retrieve the regular expression for the payee name
    fn name(&self) -> &Option<Regex> {
        &self.name
    }
}

impl Query for QueryCategories {
    type T = Category;

    fn exec(&self, db: &HomeBankDb) -> Vec<Self::T> {
        let filt_payees = db
            .categories()
            .values()
            // filter out categories that don't match the regex
            .filter(|&p| match self.name() {
                Some(re) => re.is_match(&p.full_name(db)),
                None => true,
            })
            .map(|cat| cat.clone())
            .collect();

        filt_payees
    }
}
