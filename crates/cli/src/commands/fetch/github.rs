use clap::Args;
use miette::Result;

#[derive(Debug, Args)]
pub struct GithubArgs {}

impl GithubArgs {
    pub async fn run(self) -> Result<()> {
        println!("Fetching data from GitHub...");

        Ok(())
    }
}
