use credentials::{
    credential::Credential,
    resolution::get_or_prompt_for_credential,
};
use github::client::GitHubClient;
use miette::Result;

use crate::environment::Environment;

impl Environment {
    pub fn build_github_client(&self) -> Result<GitHubClient> {
        let github_token = get_or_prompt_for_credential(Credential::Github)?;

        Ok(GitHubClient::new(self.endpoints.github_api.clone(), github_token))
    }
}
