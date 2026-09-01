use std::process::Command;

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
    miette,
};

pub fn get_rust_cli_version() -> Result<String> {
    let output = Command::new("cargo")
        .args(["pkgid", "-p", "cli"])
        .output()
        .into_diagnostic()
        .wrap_err("failed to run `cargo pkgid -p cli`")?;

    if !output.status.success() {
        let standard_error = String::from_utf8_lossy(&output.stderr).trim().to_string();

        return Err(miette!("failed to get cli crate version: {standard_error}"));
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
