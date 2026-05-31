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

pub fn run(mut args: Args) -> Result<()> {
    let release_tag = get_release_tag_from_args(&mut args)?;

    let release_version = get_release_version_from_tag(&release_tag)?;

    let rust_cli_version = get_rust_cli_version()?;

    let npm_cli_version = get_npm_cli_version()?;

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

    println!("Release tag {release_tag} matches CLI and npm package versions");

    Ok(())
}

fn get_release_tag_from_args(args: &mut Args) -> Result<String> {
    let Some(release_tag) = args.next() else {
        return Err(miette!("usage: cargo run -p xtask -- check-release-version <release-tag>"));
    };

    if let Some(extra_arg) = args.next() {
        return Err(miette!("unexpected argument: {extra_arg}"));
    }

    Ok(release_tag)
}

fn get_release_version_from_tag(release_tag: &str) -> Result<&str> {
    let Some(release_version) = release_tag.strip_prefix('v') else {
        return Err(miette!("release tag must look like vX.Y.Z"));
    };

    Ok(release_version)
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
    let package_json = fs::read_to_string("./npm/package.json")
        .into_diagnostic()
        .wrap_err("failed to read npm/package.json")?;

    let package_json: Value = serde_json::from_str(&package_json)
        .into_diagnostic()
        .wrap_err("failed to parse npm/package.json")?;

    let Some(version) = package_json.get("version").and_then(Value::as_str) else {
        return Err(miette!("npm/package.json is missing a version"));
    };

    Ok(version.to_string())
}
