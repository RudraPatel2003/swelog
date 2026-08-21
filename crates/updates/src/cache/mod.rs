use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
    process,
};

use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::errors::UpdateCheckError;

const APP_NAME: &str = "swelog";
const VERSION_CACHE_FILE_NAME: &str = "version-check.json";

/// How long a cached version stays fresh before a background refresh is started.
const VERSION_CHECK_INTERVAL_SECONDS: i64 = 86_400;

/// The last known latest published version, and when swelog last looked for it.
#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct VersionCache {
    /// `None` until a fetch succeeds, so a check can record that it happened
    /// without inventing a version.
    pub latest_version: Option<String>,

    pub checked_at: DateTime<Utc>,
}

/// The file is regenerable, so it lives in the cache directory rather than
/// beside the config.
pub fn get_version_cache_file_path() -> Result<PathBuf, UpdateCheckError> {
    let cache_directory = dirs::cache_dir().ok_or(UpdateCheckError::UnavailableCacheDirectory)?;

    let cache_file_path = cache_directory.join(APP_NAME).join(VERSION_CACHE_FILE_NAME);

    Ok(cache_file_path)
}

pub fn read_version_cache(cache_file_path: &Path) -> Result<VersionCache, UpdateCheckError> {
    let cache_file_contents = fs::read_to_string(cache_file_path).map_err(|source| {
        UpdateCheckError::FailedToReadVersionCache {
            cache_file_path: cache_file_path.to_path_buf(),
            source,
        }
    })?;

    let version_cache = serde_json::from_str(&cache_file_contents)
        .map_err(|source| UpdateCheckError::FailedToParseVersionCache { source })?;

    Ok(version_cache)
}

/// Writes through a temporary file named for this process, so concurrent swelog
/// runs cannot read a half written cache or clobber each other.
pub fn write_version_cache(
    cache_file_path: &Path,
    version_cache: &VersionCache,
) -> Result<(), UpdateCheckError> {
    if let Some(parent) = cache_file_path.parent() {
        fs::create_dir_all(parent).map_err(|source| {
            UpdateCheckError::FailedToWriteVersionCache {
                cache_file_path: cache_file_path.to_path_buf(),
                source,
            }
        })?;
    }

    let json = serde_json::to_string(version_cache)
        .map_err(|source| UpdateCheckError::FailedToSerializeVersionCache { source })?;

    let temporary_file_path = cache_file_path.with_extension(format!("{}.tmp", process::id()));

    fs::write(&temporary_file_path, json).map_err(|source| {
        UpdateCheckError::FailedToWriteVersionCache {
            cache_file_path: temporary_file_path.clone(),
            source,
        }
    })?;

    fs::rename(&temporary_file_path, cache_file_path).map_err(|source| {
        UpdateCheckError::FailedToWriteVersionCache {
            cache_file_path: cache_file_path.to_path_buf(),
            source,
        }
    })?;

    Ok(())
}

/// Whether it is time to look for a newer version. A missing cache is due, so
/// the first run always checks.
#[must_use]
pub fn is_refresh_due(version_cache: Option<&VersionCache>, now: DateTime<Utc>) -> bool {
    version_cache.is_none_or(|version_cache| {
        let time_since_last_check =
            now.signed_duration_since(version_cache.checked_at).num_seconds();

        time_since_last_check >= VERSION_CHECK_INTERVAL_SECONDS
    })
}

#[cfg(test)]
mod tests;
