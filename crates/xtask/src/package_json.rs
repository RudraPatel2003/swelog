use std::fs;

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use serde_json::Value;

pub const NPM_PACKAGE_JSON_PATH: &str = "./npm/package.json";

pub const DOCS_PACKAGE_JSON_PATH: &str = "./docs/package.json";

pub fn read_npm_package_json() -> Result<Value> {
    read_package_json(NPM_PACKAGE_JSON_PATH)
}

pub fn read_docs_package_json() -> Result<Value> {
    read_package_json(DOCS_PACKAGE_JSON_PATH)
}

fn read_package_json(path: &str) -> Result<Value> {
    let package_json =
        fs::read_to_string(path).into_diagnostic().wrap_err("failed to read package.json")?;

    let package_json: Value = serde_json::from_str(&package_json)
        .into_diagnostic()
        .wrap_err("failed to parse package.json")?;

    Ok(package_json)
}
