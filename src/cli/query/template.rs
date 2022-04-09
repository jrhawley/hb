use super::Query;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "templates",
    alias = "T",
    about = "Query templates and scheduled transactions"
)]
pub struct QueryTemplates {}
