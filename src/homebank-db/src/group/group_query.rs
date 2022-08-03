use clap::Parser;
use regex::Regex;

use crate::{Group, HomeBankDb, Query};

#[derive(Debug, Parser)]
#[clap(name = "groups", visible_alias = "g", about = "Query account groups")]
pub struct QueryGroups {
    #[clap(
        help = "Include groups whose names match this regular expression",
        value_name = "regex"
    )]
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
