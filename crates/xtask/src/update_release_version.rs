use std::env::Args;

use miette::{
    Result,
    miette,
};

use crate::{
    crate_versions::{
        list_crate_manifest_paths,
        write_crate_version,
    },
    package_json::{
        DOCS_PACKAGE_JSON_PATH,
        NPM_PACKAGE_JSON_PATH,
        write_package_json_version,
    },
    release_tag::get_release_version_from_tag,
};

pub fn run_update_release_version(mut args: Args) -> Result<()> {
    let release_tag = get_release_tag_from_args(&mut args)?;

    let release_version = get_release_version_from_tag(&release_tag)?;

    update_package_json_versions(release_version)?;

    update_crate_versions(release_version)?;

    println!("Updated all versions to {release_version}");

    Ok(())
}

fn get_release_tag_from_args(args: &mut Args) -> Result<String> {
    let Some(release_tag) = args.next() else {
        return Err(miette!("usage: cargo run -p xtask -- update-release-version <release-tag>"));
    };

    if let Some(extra_arg) = args.next() {
        return Err(miette!("unexpected argument: {extra_arg}"));
    }

    Ok(release_tag)
}

fn update_package_json_versions(release_version: &str) -> Result<()> {
    for package_json_path in [NPM_PACKAGE_JSON_PATH, DOCS_PACKAGE_JSON_PATH] {
        write_package_json_version(package_json_path, release_version)?;
    }

    Ok(())
}

fn update_crate_versions(release_version: &str) -> Result<()> {
    let crate_manifest_paths = list_crate_manifest_paths()?;

    for manifest_path in crate_manifest_paths {
        write_crate_version(&manifest_path, release_version)?;
    }

    Ok(())
}
