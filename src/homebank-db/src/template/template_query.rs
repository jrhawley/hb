use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = "templates",
    visible_alias = "T",
    about = "Query templates and scheduled transactions"
)]
pub struct QueryTemplates {}
