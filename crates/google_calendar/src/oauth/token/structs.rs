use serde::Deserialize;

use crate::oauth::credentials::GoogleCredentials;

pub enum RefreshOutcome {
    Refreshed(GoogleCredentials),
    ReauthorizationRequired,
}

pub enum TokenRequestOutcome {
    Succeeded(String),
    InvalidGrant,
}

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub(crate) access_token: String,

    pub(crate) expires_in: u64,

    #[serde(default)]
    pub(crate) refresh_token: Option<String>,

    #[serde(default)]
    pub(crate) scope: Option<String>,
}

impl TokenResponse {
    pub fn into_credentials(self, refresh_token: String, received_at: u64) -> GoogleCredentials {
        let scopes = self
            .scope
            .as_deref()
            .unwrap_or_default()
            .split_whitespace()
            .map(ToString::to_string)
            .collect();

        GoogleCredentials {
            access_token: self.access_token,
            refresh_token,
            expires_at: received_at.saturating_add(self.expires_in),
            scopes,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TokenErrorResponse {
    pub(crate) error: String,

    #[serde(default)]
    pub(crate) error_description: Option<String>,
}
