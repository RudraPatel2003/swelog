use std::time::Duration;

use reqwest::{
    Client,
    header::USER_AGENT,
};
use serde::Deserialize;

use crate::errors::UpdateCheckError;

const NPM_REGISTRY_LATEST_URL: &str = "https://registry.npmjs.org/swelog-cli/latest";

/// Duplicated from the github crate rather than imported from it, because
/// depending on that crate for a single string would be the wrong dependency
/// direction.
const SWELOG_USER_AGENT: &str = "RudraPatel2003/swelog-cli";

const VERSION_CHECK_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

/// The registry returns the full manifest for the latest published version.
/// Every field other than the version is ignored.
#[derive(Debug, Deserialize)]
struct NpmPackageManifest {
    version: String,
}

/// npm is the install path swelog documents, so the version it reports is the
/// one `npm update -g swelog-cli` would actually deliver.
pub async fn fetch_latest_version() -> Result<String, UpdateCheckError> {
    let client = Client::builder()
        .timeout(VERSION_CHECK_REQUEST_TIMEOUT)
        .build()
        .map_err(|source| UpdateCheckError::FailedToFetchLatestVersion { source })?;

    let response = client
        .get(NPM_REGISTRY_LATEST_URL)
        .header(USER_AGENT, SWELOG_USER_AGENT)
        .send()
        .await
        .map_err(|source| UpdateCheckError::FailedToFetchLatestVersion { source })?;

    let response_text = response
        .error_for_status()
        .map_err(|source| UpdateCheckError::FailedToFetchLatestVersion { source })?
        .text()
        .await
        .map_err(|source| UpdateCheckError::FailedToFetchLatestVersion { source })?;

    parse_npm_package_manifest(&response_text)
}

/// Split out from the request so that it can be tested without the network.
fn parse_npm_package_manifest(response_text: &str) -> Result<String, UpdateCheckError> {
    let npm_package_manifest: NpmPackageManifest = serde_json::from_str(response_text)
        .map_err(|source| UpdateCheckError::FailedToParseNpmPackageManifest { source })?;

    Ok(npm_package_manifest.version)
}

#[cfg(test)]
mod tests;
