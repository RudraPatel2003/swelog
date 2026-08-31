use std::path::Path;

use miette::Result;

use crate::errors::SwelogFileNotFound;

pub fn ensure_swelog_file_exists(swelog_path: &Path) -> Result<()> {
    if swelog_path.is_file() {
        return Ok(());
    }

    let swelog_file_not_found_error = SwelogFileNotFound { swelog_path: swelog_path.to_path_buf() };

    Err(swelog_file_not_found_error.into())
}

pub fn ensure_swelog_directory_exists(swelog_path: &Path) -> Result<()> {
    if swelog_path.is_dir() {
        return Ok(());
    }

    let swelog_file_not_found_error = SwelogFileNotFound { swelog_path: swelog_path.to_path_buf() };

    Err(swelog_file_not_found_error.into())
}
