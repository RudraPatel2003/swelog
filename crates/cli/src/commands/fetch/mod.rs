mod github;
mod linear;

use clap::{
    Args,
    Subcommand,
};
use github::GithubArgs;
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

    /// Fetch active Linear issues assigned to your configured Linear username
    Linear(LinearArgs),
}

impl FetchArgs {
    pub async fn run(self) -> Result<()> {
        match self.command {
            FetchCommands::Github(github_args) => github_args.run().await,
            FetchCommands::Linear(linear_args) => linear_args.run().await,
        }
    }
}
