use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "templates",
    visible_alias = "T",
    about = "Query templates and scheduled transactions"
)]
pub struct QueryTemplates {}
