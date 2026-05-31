mod day;

use clap::{
    Args,
    Subcommand,
};
use day::DailySummaryArgs;
use miette::Result;

#[derive(Debug, Args)]
pub struct SummarizeArgs {
    #[command(flatten)]
    daily_summary_args: DailySummaryArgs,

    #[command(subcommand)]
    command: Option<SummarizeCommands>,
}

#[derive(Debug, Subcommand)]
enum SummarizeCommands {
    /// Summarize your configured work file and log it into the daily folder.
    Day(DailySummaryArgs),
}

impl SummarizeArgs {
    pub async fn run(self) -> Result<()> {
        match self.command {
            Some(SummarizeCommands::Day(daily_summary_args)) => {
                daily_summary_args.run().await?;
            }

            None => {
                self.daily_summary_args.run().await?;
            }
        }

        Ok(())
    }
}
