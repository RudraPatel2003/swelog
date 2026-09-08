use credentials::{
    credential::Credential,
    resolution::get_or_prompt_for_credential,
};
use github::client::GitHubClient;
use llm::{
    language_model::LanguageModel,
    language_model_factory::get_language_model,
    summarization_settings::SummarizationSettings,
};
use miette::Result;

use crate::environment::Environment;

impl Environment {
    pub fn build_github_client(&self) -> Result<GitHubClient> {
        let github_token = get_or_prompt_for_credential(Credential::Github)?;

        let github_api_endpoint = self.endpoints.github_api.clone();

        let github_client = GitHubClient::new(github_api_endpoint, github_token);

        Ok(github_client)
    }

    pub fn build_language_model(
        &self,
        summarization_settings: &SummarizationSettings,
    ) -> Result<Box<dyn LanguageModel>> {
        get_language_model(summarization_settings, &self.endpoints.language_model_endpoints())
    }
}
