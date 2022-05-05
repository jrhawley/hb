//! Render the `BudgetSummary` into a visual element in the terminal.

use homebank_db::category::budget_query::BudgetSummary;
use indicatif::{ProgressBar, ProgressStyle};

/// Create a `ProgressBar` out of a `BudgetSummary`
pub fn budget_pbar(summary: BudgetSummary) -> ProgressBar {
    if let (Some(val), Some(frac)) = (summary.allotment_rounded(), summary.progress_frac()) {
        let pbar = ProgressBar::new(val);
        let bar_colour: &str;

        if *frac > 1.0 {
            bar_colour = "red";
        } else if *frac > 0.5 {
            bar_colour = "yellow";
        } else {
            bar_colour = "white";
        }

        let template = format!(
            "{{msg:<30.{bar_colour}}} {{wide_bar:.{bar_colour}}} {{pos:>6.{bar_colour}}}/{{len:>6}} ({{percent:>3.{bar_colour}}} %)"
        );

        pbar.set_message(format!("{}", summary.name()));
        pbar.set_style(ProgressStyle::default_bar().template(&template));

        pbar.set_position(summary.progress_rounded());

        pbar
    } else {
        let pbar = ProgressBar::new(u64::MAX);
        let bar_colour = "white";

        let template = format!(
            "{{msg:<30.{bar_colour}}} {{wide_bar:.{bar_colour}}} {{pos:>6.{bar_colour}}}/  None ({{percent:>3.{bar_colour}}} %)"
        );

        pbar.set_message(format!("{}", summary.name()));
        pbar.set_style(ProgressStyle::default_bar().template(&template));

        pbar.set_position(summary.progress_rounded());

        pbar
    }
}
