use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

use crate::errors::UnavailableCacheDirectory;

const APP_NAME: &str = "swelog";
const HIDE_COMMENTS_FLAG_FILE_NAME: &str = "hide-comments";

pub fn set_hide_comments_flag() -> Result<()> {
    let flag_file_path = get_hide_comments_flag_file_path()?;

    set_hide_comments_flag_at(&flag_file_path)
}

#[must_use]
pub fn has_hide_comments_flag() -> bool {
    let Ok(flag_file_path) = get_hide_comments_flag_file_path() else {
        return false;
    };

    has_hide_comments_flag_at(&flag_file_path)
}

fn get_hide_comments_flag_file_path() -> Result<PathBuf> {
    let Some(cache_directory) = dirs::cache_dir() else {
        let unavailable_cache_directory_error = UnavailableCacheDirectory;

        return Err(unavailable_cache_directory_error.into());
    };

    let flag_file_path = cache_directory.join(APP_NAME).join(HIDE_COMMENTS_FLAG_FILE_NAME);

    Ok(flag_file_path)
}

fn set_hide_comments_flag_at(flag_file_path: &Path) -> Result<()> {
    if has_hide_comments_flag_at(flag_file_path) {
        return Ok(());
    }

    if let Some(parent) = flag_file_path.parent() {
        fs::create_dir_all(parent).into_diagnostic().wrap_err_with(|| {
            format!("failed to create the swelog cache directory at {}", parent.display())
        })?;
    }

    fs::write(flag_file_path, "").into_diagnostic().wrap_err_with(|| {
        format!("failed to write the hide comments flag at {}", flag_file_path.display())
    })
}

fn has_hide_comments_flag_at(flag_file_path: &Path) -> bool {
    flag_file_path.is_file()
}

#[cfg(test)]
mod tests;
