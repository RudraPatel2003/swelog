use std::{
    path::PathBuf,
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
    notice::get_update_notice,
    registry::fetch_latest_version,
};

const REFRESH_GRACE_PERIOD: Duration = Duration::from_secs(1);

pub struct PendingUpdateNotice {
    update_notice: Option<String>,
    refresh: Option<JoinHandle<()>>,
}

#[must_use]
pub fn start_version_check(current_version: &str) -> PendingUpdateNotice {
    let Ok(cache_file_path) = get_version_cache_file_path() else {
        return PendingUpdateNotice { update_notice: None, refresh: None };
    };

    let version_cache = read_version_cache(&cache_file_path).ok();

    let refresh = is_refresh_due(version_cache.as_ref(), Utc::now())
        .then(|| tokio::spawn(store_latest_version(cache_file_path)));

    let latest_version = version_cache.map(|version_cache| version_cache.latest_version);

    let update_notice = get_update_notice(current_version, latest_version.as_deref());

    PendingUpdateNotice { update_notice, refresh }
}

pub async fn print_update_notice(pending_update_notice: PendingUpdateNotice) {
    let PendingUpdateNotice { update_notice, refresh } = pending_update_notice;

    print_notice(update_notice);

    wait_for_refresh(refresh).await;
}

fn print_notice(update_notice: Option<String>) {
    let Some(update_notice) = update_notice else {
        return;
    };

    eprintln!();

    eprint!("{update_notice}");
}

async fn wait_for_refresh(refresh: Option<JoinHandle<()>>) {
    let Some(refresh) = refresh else {
        return;
    };

    let _ = timeout(REFRESH_GRACE_PERIOD, refresh).await;
}

async fn store_latest_version(cache_file_path: PathBuf) {
    let Ok(latest_version) = fetch_latest_version().await else {
        return;
    };

    let version_cache = VersionCache { latest_version, checked_at: Utc::now() };

    let _ = write_version_cache(&cache_file_path, &version_cache);
}
