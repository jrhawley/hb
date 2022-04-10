use regex::Regex;
use structopt::StructOpt;

use crate::{Group, HomeBankDb, Query};

#[derive(Debug, StructOpt)]
#[structopt(name = "groups", alias = "g", about = "Query account groups")]
pub struct QueryGroups {
    #[structopt(
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

    fn exec<'a>(&self, db: &'a HomeBankDb) -> Vec<&'a Self::T> {
        let filt_groups = db
            .groups()
            .values()
            .filter(|&grp| match self.name() {
                Some(re) => re.is_match(grp.name()),
                None => true,
            })
            .collect();

        filt_groups
    }
}
