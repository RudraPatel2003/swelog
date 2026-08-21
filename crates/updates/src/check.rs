use std::{
    io::{
        IsTerminal,
        stderr,
    },
    path::{
        Path,
        PathBuf,
    },
    time::Duration,
};

use chrono::Utc;
use tokio::{
    task::JoinHandle,
    time::timeout,
};

use crate::{
    cache::{
        VersionCache,
        get_version_cache_file_path,
        is_refresh_due,
        read_version_cache,
        write_version_cache,
    },
    errors::UpdateCheckError,
    notice::get_update_notice,
    registry::fetch_latest_version,
};

/// A ceiling rather than a cost, because the wait ends as soon as the request
/// does. A tighter one would pay for most of a request and then discard the
/// answer.
const REFRESH_GRACE_PERIOD: Duration = Duration::from_millis(400);

/// A background refresh of the cached latest version, when one was started.
pub struct VersionCacheRefresh(Option<JoinHandle<()>>);

impl VersionCacheRefresh {
    /// Dropping the tokio runtime cancels spawned tasks instead of waiting for
    /// them, so the refresh needs its own chance to finish before main returns.
    async fn wait_for_completion(self) {
        let Some(refresh) = self.0 else {
            return;
        };

        let _ = timeout(REFRESH_GRACE_PERIOD, refresh).await;
    }
}

/// Starts a background refresh of the cached latest version, once a day.
///
/// The calling task never performs a network request, so this is safe to call
/// ahead of a command that needs to stay fast.
#[must_use]
pub fn refresh_latest_version_cache() -> VersionCacheRefresh {
    if !stderr().is_terminal() {
        return VersionCacheRefresh(None);
    }

    let Ok(cache_file_path) = get_version_cache_file_path() else {
        return VersionCacheRefresh(None);
    };

    let previous_version_cache = read_version_cache(&cache_file_path).ok();

    if !is_refresh_due(previous_version_cache.as_ref(), Utc::now()) {
        return VersionCacheRefresh(None);
    }

    let previous_latest_version =
        previous_version_cache.and_then(|version_cache| version_cache.latest_version);

    if claim_check_window(&cache_file_path, previous_latest_version).is_err() {
        return VersionCacheRefresh(None);
    }

    VersionCacheRefresh(Some(tokio::spawn(store_latest_version(cache_file_path))))
}

/// Prints a notice to stderr when a newer version of swelog has been published.
///
/// Call this after the command has run, so a refresh that finished while the
/// command was working is picked up on this run rather than the next one.
/// Nothing is printed when stderr is not a terminal, which keeps piped output
/// clean.
pub async fn print_update_notice(
    current_version: &str,
    version_cache_refresh: VersionCacheRefresh,
) {
    version_cache_refresh.wait_for_completion().await;

    if !stderr().is_terminal() {
        return;
    }

    let Some(update_notice) = read_update_notice(current_version) else {
        return;
    };

    eprintln!();
    eprint!("{update_notice}");
}

/// Records the attempt before the request starts, so a request that outlives
/// the command still counts as today's check and every invocation does not
/// retry forever. The previously known version is carried forward so an outage
/// cannot hide an update that is already known about.
fn claim_check_window(
    cache_file_path: &Path,
    previous_latest_version: Option<String>,
) -> Result<(), UpdateCheckError> {
    let claimed_version_cache =
        VersionCache { latest_version: previous_latest_version, checked_at: Utc::now() };

    write_version_cache(cache_file_path, &claimed_version_cache)
}

/// A failed fetch leaves the claimed window in place rather than overwriting it.
async fn store_latest_version(cache_file_path: PathBuf) {
    let Ok(latest_version) = fetch_latest_version().await else {
        return;
    };

    let version_cache =
        VersionCache { latest_version: Some(latest_version), checked_at: Utc::now() };

    let _ = write_version_cache(&cache_file_path, &version_cache);
}

fn read_update_notice(current_version: &str) -> Option<String> {
    let cache_file_path = get_version_cache_file_path().ok()?;

    let version_cache = read_version_cache(&cache_file_path).ok()?;

    get_update_notice(current_version, version_cache.latest_version.as_deref())
}
