use super::Query;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "groups", alias = "g", about = "Query account groups")]
pub struct QueryGroups {}
