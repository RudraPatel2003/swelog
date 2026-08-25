use std::{
    env::Args,
    fs,
    process::Command,
};

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
    miette,
};
use serde_json::Value;

pub const NPM_PACKAGE_JSON_PATH: &str = "./npm/package.json";

pub const DOCS_PACKAGE_JSON_PATH: &str = "./docs/package.json";

pub fn get_release_tag_from_args(args: &mut Args, command: &str) -> Result<String> {
    let Some(release_tag) = args.next() else {
        return Err(miette!("usage: cargo run -p xtask -- {command} <release-tag>"));
    };

    if let Some(extra_arg) = args.next() {
        return Err(miette!("unexpected argument: {extra_arg}"));
    }

    Ok(release_tag)
}

pub fn get_release_version_from_tag(release_tag: &str) -> Result<&str> {
    let Some(release_version) = release_tag.strip_prefix('v') else {
        return Err(miette!("release tag must look like vX.Y.Z"));
    };

    Ok(release_version)
}

fn read_package_json(path: &str) -> Result<Value> {
    let package_json =
        fs::read_to_string(path).into_diagnostic().wrap_err("failed to read package.json")?;

    let package_json: Value = serde_json::from_str(&package_json)
        .into_diagnostic()
        .wrap_err("failed to parse package.json")?;

    Ok(package_json)
}

pub fn read_npm_package_json() -> Result<Value> {
    read_package_json(NPM_PACKAGE_JSON_PATH)
}

pub fn read_docs_package_json() -> Result<Value> {
    read_package_json(DOCS_PACKAGE_JSON_PATH)
}

pub fn get_rust_cli_version() -> Result<String> {
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
