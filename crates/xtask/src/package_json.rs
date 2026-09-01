use std::fs;

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
    miette,
};
use serde_json::Value;

pub const NPM_PACKAGE_JSON_PATH: &str = "./npm/package.json";

pub const DOCS_PACKAGE_JSON_PATH: &str = "./docs/package.json";

pub fn read_package_json_version(path: &str) -> Result<String> {
    let package_json = read_package_json(path)?;

    let Some(version) = package_json.get("version").and_then(Value::as_str) else {
        return Err(miette!("{path} is missing a version"));
    };

    Ok(version.to_string())
}

pub fn write_package_json_version(path: &str, release_version: &str) -> Result<()> {
    let mut package_json = read_package_json(path)?;

    let package_json_object =
        package_json.as_object_mut().ok_or_else(|| miette!("{path} is not a JSON object"))?;

    package_json_object.insert(String::from("version"), Value::String(release_version.to_string()));

    let mut serialized = serde_json::to_string_pretty(&package_json)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to serialize {path}"))?;

    serialized.push('\n');

    fs::write(path, serialized)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to write {path}"))
}

fn read_package_json(path: &str) -> Result<Value> {
    let package_json = fs::read_to_string(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read {path}"))?;

    serde_json::from_str(&package_json)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to parse {path}"))
}
