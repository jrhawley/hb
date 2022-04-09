use super::Query;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "categories",
    alias = "c",
    about = "Query transaction categories"
)]
pub struct QueryCategories {}
