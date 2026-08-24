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
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::errors::UnavailableCacheDirectory;

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
pub fn get_version_cache_file_path() -> Result<PathBuf> {
    let Some(cache_directory) = dirs::cache_dir() else {
        let unavailable_cache_directory_error = UnavailableCacheDirectory;

        return Err(unavailable_cache_directory_error.into());
    };

    let cache_file_path = cache_directory.join(APP_NAME).join(VERSION_CACHE_FILE_NAME);

    Ok(cache_file_path)
}

pub fn read_version_cache(cache_file_path: &Path) -> Result<VersionCache> {
    let cache_file_contents =
        fs::read_to_string(cache_file_path).into_diagnostic().wrap_err_with(|| {
            format!("failed to read the version cache at {}", cache_file_path.display())
        })?;

    let version_cache = serde_json::from_str(&cache_file_contents)
        .into_diagnostic()
        .wrap_err("failed to parse the version cache")?;

    Ok(version_cache)
}

/// Writes through a temporary file named for this process, so concurrent swelog
/// runs cannot read a half written cache or clobber each other.
pub fn write_version_cache(cache_file_path: &Path, version_cache: &VersionCache) -> Result<()> {
    if let Some(parent) = cache_file_path.parent() {
        fs::create_dir_all(parent).into_diagnostic().wrap_err_with(|| {
            format!("failed to create the version cache directory at {}", parent.display())
        })?;
    }

    let json = serde_json::to_string(version_cache)
        .into_diagnostic()
        .wrap_err("failed to serialize the version cache")?;

    let temporary_file_path = cache_file_path.with_extension(format!("{}.tmp", process::id()));

    fs::write(&temporary_file_path, json).into_diagnostic().wrap_err_with(|| {
        format!("failed to write the version cache at {}", temporary_file_path.display())
    })?;

    fs::rename(&temporary_file_path, cache_file_path).into_diagnostic().wrap_err_with(|| {
        format!("failed to write the version cache at {}", cache_file_path.display())
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
