use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("failed to fetch the latest published version")]
#[diagnostic(code(swelog::updates::failed_to_fetch_latest_version))]
pub struct FailedToFetchLatestVersion;
