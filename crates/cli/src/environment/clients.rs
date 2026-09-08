use credentials::{
    credential::Credential,
    resolution::get_or_prompt_for_credential,
};
use github::client::GitHubClient;
use google_calendar::{
    client::GoogleCalendarClient,
    oauth::{
        GoogleAuthorization,
        application::get_oauth_application,
    },
};
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

    pub fn build_google_calendar_client(&self) -> Result<GoogleCalendarClient> {
        let application = get_oauth_application(&self.google_oauth_application_overrides)?;

        let google_token = self.endpoints.google_token.clone();

        let google_calendar_api_endpoint = self.endpoints.google_calendar_api.clone();

        let authorization = GoogleAuthorization::new(application, google_token);

        let google_calendar_client =
            GoogleCalendarClient::new(google_calendar_api_endpoint, authorization);

        Ok(google_calendar_client)
    }

    pub fn build_language_model(
        &self,
        summarization_settings: &SummarizationSettings,
    ) -> Result<Box<dyn LanguageModel>> {
        let language_model_endpoints = self.endpoints.language_model_endpoints();

        get_language_model(summarization_settings, &language_model_endpoints)
    }
}
