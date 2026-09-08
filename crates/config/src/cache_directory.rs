use std::path::PathBuf;

use miette::Result;

use crate::errors::UnavailableCacheDirectory;

const APP_NAME: &str = "swelog";

pub fn get_default_cache_directory() -> Result<PathBuf> {
    let Some(cache_directory) = dirs::cache_dir() else {
        let unavailable_cache_directory_error = UnavailableCacheDirectory;

        return Err(unavailable_cache_directory_error.into());
    };

    Ok(cache_directory.join(APP_NAME))
}
