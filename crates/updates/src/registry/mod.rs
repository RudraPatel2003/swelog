use std::time::Duration;

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use reqwest::{
    Client,
    header::USER_AGENT,
};
use serde::Deserialize;

use crate::errors::FailedToFetchLatestVersion;

const NPM_REGISTRY_LATEST_URL: &str = "https://registry.npmjs.org/swelog-cli/latest";

const SWELOG_USER_AGENT: &str = "RudraPatel2003/swelog-cli";

const VERSION_CHECK_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Deserialize)]
struct NpmPackageManifest {
    version: String,
}

pub async fn fetch_latest_version() -> Result<String> {
    let client = Client::builder()
        .timeout(VERSION_CHECK_REQUEST_TIMEOUT)
        .build()
        .into_diagnostic()
        .wrap_err_with(|| FailedToFetchLatestVersion)?;

    let response = client
        .get(NPM_REGISTRY_LATEST_URL)
        .header(USER_AGENT, SWELOG_USER_AGENT)
        .send()
        .await
        .into_diagnostic()
        .wrap_err_with(|| FailedToFetchLatestVersion)?;

    let response_text = response
        .error_for_status()
        .into_diagnostic()
        .wrap_err_with(|| FailedToFetchLatestVersion)?
        .text()
        .await
        .into_diagnostic()
        .wrap_err_with(|| FailedToFetchLatestVersion)?;

    parse_npm_package_manifest(&response_text)
}

fn parse_npm_package_manifest(response_text: &str) -> Result<String> {
    let npm_package_manifest: NpmPackageManifest = serde_json::from_str(response_text)
        .into_diagnostic()
        .wrap_err("failed to parse the npm package manifest")?;

    let latest_version = npm_package_manifest.version;

    Ok(latest_version)
}

#[cfg(test)]
mod tests;
