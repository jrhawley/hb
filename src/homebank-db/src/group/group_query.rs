//! Filtering options when querying [`Group`s][crate::group::group::Group] from the database.

use crate::{Group, HomeBankDb, Query};
use clap::Parser;
use regex::Regex;

/// Filtering options when querying [`Group`s][crate::group::group::Group] from the database.
#[derive(Debug, Parser)]
#[clap(name = "groups", visible_alias = "g", about = "Query account groups")]
pub struct QueryGroups {
    /// Include groups whose names match this regular expression.
    #[clap(value_name = "regex")]
    name: Option<Regex>,
}

impl QueryGroups {
    fn name(&self) -> &Option<Regex> {
        &self.name
    }
}

impl Query for QueryGroups {
    type T = Group;

    fn exec(&self, db: &HomeBankDb) -> Vec<Self::T> {
        let filt_groups = db
            .groups()
            .values()
            .filter(|&grp| match self.name() {
                Some(re) => re.is_match(grp.name()),
                None => true,
            })
            .map(|grp| grp.clone())
            .collect();

        filt_groups
    }
}
