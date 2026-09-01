use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
    process,
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

use crate::errors::{
    NoUndoSnapshot,
    UnavailableCacheDirectory,
};

const APP_NAME: &str = "swelog";

const UNDO_SNAPSHOT_FILE_NAME: &str = "undo.json";

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoSnapshot {
    pub created_file: Option<PathBuf>,

    pub work_file_content: String,
}

pub fn get_undo_snapshot_file_path() -> Result<PathBuf> {
    let Some(cache_directory) = dirs::cache_dir() else {
        let unavailable_cache_directory_error = UnavailableCacheDirectory;

        return Err(unavailable_cache_directory_error.into());
    };

    let undo_snapshot_file = cache_directory.join(APP_NAME).join(UNDO_SNAPSHOT_FILE_NAME);

    Ok(undo_snapshot_file)
}

pub fn read_undo_snapshot(undo_snapshot_file: &Path) -> Result<UndoSnapshot> {
    if !undo_snapshot_file.is_file() {
        let no_undo_snapshot_error = NoUndoSnapshot;

        return Err(no_undo_snapshot_error.into());
    }

    let undo_snapshot_contents =
        fs::read_to_string(undo_snapshot_file).into_diagnostic().wrap_err_with(|| {
            format!("failed to read the undo snapshot at {}", undo_snapshot_file.display())
        })?;

    let undo_snapshot = serde_json::from_str(&undo_snapshot_contents)
        .into_diagnostic()
        .wrap_err("failed to parse the undo snapshot")?;

    Ok(undo_snapshot)
}

pub fn write_undo_snapshot(undo_snapshot_file: &Path, undo_snapshot: &UndoSnapshot) -> Result<()> {
    if let Some(parent) = undo_snapshot_file.parent() {
        fs::create_dir_all(parent).into_diagnostic().wrap_err_with(|| {
            format!("failed to create the swelog cache directory at {}", parent.display())
        })?;
    }

    let json = serde_json::to_string(undo_snapshot)
        .into_diagnostic()
        .wrap_err("failed to serialize the undo snapshot")?;

    let temporary_file_path = undo_snapshot_file.with_extension(format!("{}.tmp", process::id()));

    fs::write(&temporary_file_path, json).into_diagnostic().wrap_err_with(|| {
        format!("failed to write the undo snapshot at {}", temporary_file_path.display())
    })?;

    fs::rename(&temporary_file_path, undo_snapshot_file).into_diagnostic().wrap_err_with(|| {
        format!("failed to write the undo snapshot at {}", undo_snapshot_file.display())
    })?;

    Ok(())
}

pub fn remove_undo_snapshot(undo_snapshot_file: &Path) -> Result<()> {
    if !undo_snapshot_file.exists() {
        return Ok(());
    }

    fs::remove_file(undo_snapshot_file).into_diagnostic().wrap_err_with(|| {
        format!("failed to remove the undo snapshot at {}", undo_snapshot_file.display())
    })
}

#[cfg(test)]
mod tests;
