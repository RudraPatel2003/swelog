use std::{
    env::Args,
    process::Command,
};

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
    miette,
};
use serde_json::Value;

use crate::utils::{
    get_release_tag_from_args,
    get_release_version_from_tag,
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

fn get_rust_cli_version() -> Result<String> {
    let output = Command::new("cargo")
        .args(["pkgid", "-p", "cli"])
        .output()
        .into_diagnostic()
        .wrap_err("failed to run `cargo pkgid -p cli`")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        return Err(miette!("failed to get cli crate version: {stderr}"));
    }

    let package_id = String::from_utf8(output.stdout)
        .into_diagnostic()
        .wrap_err("cargo package id was not valid UTF-8")?;

    let package_id = package_id.trim();

    let version = package_id
        .split('#')
        .next_back()
        .filter(|version| !version.is_empty())
        .ok_or_else(|| miette!("could not parse version from package id: {package_id}"))?;

    Ok(version.to_string())
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
