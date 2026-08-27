mod github;
mod google_calendar;
mod linear;

use clap::{
    Args,
    Subcommand,
};
use github::GithubArgs;
use google_calendar::GoogleCalendarArgs;
use linear::LinearArgs;
use miette::Result;

#[derive(Debug, Args)]
pub struct FetchArgs {
    #[command(subcommand)]
    command: FetchCommands,
}

#[derive(Debug, Subcommand)]
enum FetchCommands {
    /// Fetch the PRs you opened and merged on a date in GitHub
    Github(GithubArgs),

    /// Fetch the Linear issues assigned to your configured Linear username
    Linear(LinearArgs),

    /// Fetch the meetings on your Google Calendar for a date
    GoogleCalendar(GoogleCalendarArgs),
}

impl FetchArgs {
    pub async fn run(self) -> Result<()> {
        match self.command {
            FetchCommands::Github(github_args) => github_args.run().await,
            FetchCommands::Linear(linear_args) => linear_args.run().await,

            FetchCommands::GoogleCalendar(google_calendar_args) => google_calendar_args.run().await,
        }
    }
}
