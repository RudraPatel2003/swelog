use clap::Args;
use config::utils::read_config_file;
use github::{
    github_token::get_github_token,
    issues::{
        get_merged_prs,
        get_opened_prs,
    },
    users::get_github_username,
};
use miette::Result;

#[derive(Debug, Args)]
pub struct GithubArgs {}

impl GithubArgs {
    pub async fn run(self) -> Result<()> {
        let swelog_config = read_config_file()?;

        let github_token = get_github_token()?;

        let github_username = get_github_username(&github_token).await?;

        let (opened_prs, merged_prs) = tokio::try_join!(
            get_opened_prs(&github_token, &github_username),
            get_merged_prs(&github_token, &github_username),
        )?;

        println!("{}", github_username);
        println!("{:?}", opened_prs);
        println!("{:?}", merged_prs);

        Ok(())
    }
}
