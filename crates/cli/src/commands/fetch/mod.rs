mod github;

use clap::{
    Args,
    Subcommand,
};
use github::GithubArgs;
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
}

impl FetchArgs {
    pub async fn run(self) -> Result<()> {
        match self.command {
            FetchCommands::Github(github_args) => github_args.run().await,
        }
    }
}
