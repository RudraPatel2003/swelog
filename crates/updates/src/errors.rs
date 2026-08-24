use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("unable to determine the cache directory")]
#[diagnostic(code(swelog::updates::unavailable_cache_directory))]
pub struct UnavailableCacheDirectory;

#[derive(Debug, Diagnostic, Error)]
#[error("failed to fetch the latest published version")]
#[diagnostic(code(swelog::updates::failed_to_fetch_latest_version))]
pub struct FailedToFetchLatestVersion;
