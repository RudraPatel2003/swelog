use std::{
    env::Args,
    fs,
    path::Path,
};

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
    miette,
};
use serde_json::Value;

use crate::utils::{
    NPM_PACKAGE_JSON_PATH,
    get_release_tag_from_args,
    get_release_version_from_tag,
    read_npm_package_json,
};

const CRATES_DIRECTORY: &str = "./crates";

pub fn run(mut args: Args) -> Result<()> {
    let release_tag = get_release_tag_from_args(&mut args, "update-release-version")?;

    let release_version = get_release_version_from_tag(&release_tag)?;

    update_npm_version(release_version)?;

    update_crate_versions(release_version)?;

    println!("Updated all versions to {release_version}");

    Ok(())
}

fn update_npm_version(release_version: &str) -> Result<()> {
    let mut package_json = read_npm_package_json()?;

    let package_json_object = package_json
        .as_object_mut()
        .ok_or_else(|| miette!("npm/package.json is not a JSON object"))?;

    package_json_object.insert(String::from("version"), Value::String(release_version.to_string()));

    let mut serialized = serde_json::to_string_pretty(&package_json)
        .into_diagnostic()
        .wrap_err("failed to serialize npm/package.json")?;

    serialized.push('\n');

    fs::write(NPM_PACKAGE_JSON_PATH, serialized)
        .into_diagnostic()
        .wrap_err("failed to write npm/package.json")?;

    Ok(())
}

fn update_crate_versions(release_version: &str) -> Result<()> {
    let entries = fs::read_dir(CRATES_DIRECTORY)
        .into_diagnostic()
        .wrap_err("failed to read crates directory")?;

    for entry in entries {
        let entry = entry.into_diagnostic().wrap_err("failed to read crates directory entry")?;

        let cargo_toml_path = entry.path().join("Cargo.toml");

        if !cargo_toml_path.is_file() {
            continue;
        }

        update_cargo_toml_version(&cargo_toml_path, release_version)?;
    }

    Ok(())
}

fn update_cargo_toml_version(cargo_toml_path: &Path, release_version: &str) -> Result<()> {
    let cargo_toml = fs::read_to_string(cargo_toml_path)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read {}", cargo_toml_path.display()))?;

    let mut lines: Vec<String> = cargo_toml.lines().map(str::to_string).collect();

    let version_line = lines
        .iter_mut()
        .find(|line| line.trim_start().starts_with("version ="))
        .ok_or_else(|| miette!("no version line found in {}", cargo_toml_path.display()))?;

    *version_line = format!("version = \"{release_version}\"");

    let mut updated = lines.join("\n");

    updated.push('\n');

    fs::write(cargo_toml_path, updated)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to write {}", cargo_toml_path.display()))?;

    Ok(())
}
