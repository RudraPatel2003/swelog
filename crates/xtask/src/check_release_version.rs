use std::env::Args;

use miette::{
    Result,
    miette,
};
use serde_json::Value;

use crate::utils::{
    get_release_tag_from_args,
    get_release_version_from_tag,
    get_rust_cli_version,
    read_docs_package_json,
    read_npm_package_json,
};

pub fn run_check_release_version(mut args: Args) -> Result<()> {
    let release_tag = get_release_tag_from_args(&mut args, "check-release-version")?;

    let release_version = get_release_version_from_tag(&release_tag)?;

    let rust_cli_version = get_rust_cli_version()?;

    let npm_cli_version = get_npm_cli_version()?;

    let docs_version = get_docs_version()?;

    if rust_cli_version != release_version {
        return Err(miette!(
            "cli crate version {rust_cli_version} does not match release tag {release_tag}"
        ));
    }

    if npm_cli_version != release_version {
        return Err(miette!(
            "npm package version {npm_cli_version} does not match release tag {release_tag}"
        ));
    }

    if docs_version != release_version {
        return Err(miette!(
            "docs package version {docs_version} does not match release tag {release_tag}"
        ));
    }

    println!("Release tag {release_tag} matches CLI and npm package versions");

    Ok(())
}

fn get_npm_cli_version() -> Result<String> {
    let package_json = read_npm_package_json()?;

    let Some(version) = package_json.get("version").and_then(Value::as_str) else {
        return Err(miette!("npm/package.json is missing a version"));
    };

    Ok(version.to_string())
}

fn get_docs_version() -> Result<String> {
    let package_json = read_docs_package_json()?;

    let Some(version) = package_json.get("version").and_then(Value::as_str) else {
        return Err(miette!("docs/package.json is missing a version"));
    };

    Ok(version.to_string())
}
