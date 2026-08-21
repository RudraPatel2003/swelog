use std::{
    io::Error as IoError,
    path::PathBuf,
};

use miette::Diagnostic;
use thiserror::Error;

/// Every variant is swallowed before it reaches the user.
///
/// The update check is a convenience, so failing to read the cache or reach the
/// registry must never interrupt the command the user actually ran.
#[derive(Debug, Diagnostic, Error)]
pub enum UpdateCheckError {
    #[error("unable to determine the cache directory")]
    #[diagnostic(code(swelog::updates::unavailable_cache_directory))]
    UnavailableCacheDirectory,

    #[error("failed to read the version cache at {cache_file_path}")]
    #[diagnostic(code(swelog::updates::failed_to_read_version_cache))]
    FailedToReadVersionCache {
        cache_file_path: PathBuf,
        #[source]
        source: IoError,
    },

    #[error("failed to write the version cache at {cache_file_path}")]
    #[diagnostic(code(swelog::updates::failed_to_write_version_cache))]
    FailedToWriteVersionCache {
        cache_file_path: PathBuf,
        #[source]
        source: IoError,
    },

    #[error("failed to parse the version cache")]
    #[diagnostic(code(swelog::updates::failed_to_parse_version_cache))]
    FailedToParseVersionCache {
        #[source]
        source: serde_json::Error,
    },

    #[error("failed to serialize the version cache")]
    #[diagnostic(code(swelog::updates::failed_to_serialize_version_cache))]
    FailedToSerializeVersionCache {
        #[source]
        source: serde_json::Error,
    },

    #[error("failed to fetch the latest published version")]
    #[diagnostic(code(swelog::updates::failed_to_fetch_latest_version))]
    FailedToFetchLatestVersion {
        #[source]
        source: reqwest::Error,
    },

    #[error("failed to parse the npm package manifest")]
    #[diagnostic(code(swelog::updates::failed_to_parse_npm_package_manifest))]
    FailedToParseNpmPackageManifest {
        #[source]
        source: serde_json::Error,
    },
}
