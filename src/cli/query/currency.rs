use super::Query;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "currencies", alias = "C", about = "Query currencies used")]
pub struct QueryCurrencies {}
