use miette::{
    Result,
    miette,
};

use crate::crate_versions::read_cli_version;

pub fn run_list_release_tag() -> Result<()> {
    let cli_version = read_cli_version()?;

    println!("v{cli_version}");

    Ok(())
}

pub fn get_release_version_from_tag(release_tag: &str) -> Result<&str> {
    let Some(release_version) = release_tag.strip_prefix('v') else {
        return Err(miette!("release tag must look like vX.Y.Z"));
    };

    Ok(release_version)
}
