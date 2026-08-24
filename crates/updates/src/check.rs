use std::{
    path::{
        Path,
        PathBuf,
    },
    time::Duration,
};

use chrono::Utc;
use miette::Result;
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
    notice::get_update_notice,
    registry::fetch_latest_version,
};

const REFRESH_GRACE_PERIOD: Duration = Duration::from_millis(400);

pub struct VersionCacheRefresh(Option<JoinHandle<()>>);

impl VersionCacheRefresh {
    async fn wait_for_completion(self) {
        let Some(refresh) = self.0 else {
            return;
        };

        let _ = timeout(REFRESH_GRACE_PERIOD, refresh).await;
    }
}

#[must_use]
pub fn refresh_latest_version_cache() -> VersionCacheRefresh {
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

pub async fn print_update_notice(
    current_version: &str,
    version_cache_refresh: VersionCacheRefresh,
) {
    version_cache_refresh.wait_for_completion().await;

    let Some(update_notice) = read_update_notice(current_version) else {
        return;
    };

    eprintln!();
    eprint!("{update_notice}");
}

fn claim_check_window(
    cache_file_path: &Path,
    previous_latest_version: Option<String>,
) -> Result<()> {
    let claimed_version_cache =
        VersionCache { latest_version: previous_latest_version, checked_at: Utc::now() };

    write_version_cache(cache_file_path, &claimed_version_cache)
}

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
