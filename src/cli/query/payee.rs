use super::Query;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "payees",
    alias = "p",
    about = "Query transaction payees, to and from"
)]
pub struct QueryPayees {}
