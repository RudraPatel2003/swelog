use std::{
    env::Args,
    fs,
};

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
    miette,
};
use serde_json::Value;

pub const NPM_PACKAGE_JSON_PATH: &str = "./npm/package.json";

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

pub fn read_npm_package_json() -> Result<Value> {
    let package_json = fs::read_to_string(NPM_PACKAGE_JSON_PATH)
        .into_diagnostic()
        .wrap_err("failed to read npm/package.json")?;

    let package_json: Value = serde_json::from_str(&package_json)
        .into_diagnostic()
        .wrap_err("failed to parse npm/package.json")?;

    Ok(package_json)
}
