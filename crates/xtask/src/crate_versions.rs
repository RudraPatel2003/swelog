use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
    miette,
};

const CRATES_DIRECTORY: &str = "./crates";

const CLI_MANIFEST_PATH: &str = "./crates/cli/Cargo.toml";

pub fn read_cli_version() -> Result<String> {
    let cli_crate_manifest_path = Path::new(CLI_MANIFEST_PATH);

    read_crate_version(cli_crate_manifest_path)
}

pub fn list_crate_manifest_paths() -> Result<Vec<PathBuf>> {
    let entries = fs::read_dir(CRATES_DIRECTORY)
        .into_diagnostic()
        .wrap_err("failed to read crates directory")?;

    let mut manifest_paths = Vec::new();

    for entry in entries {
        let entry = entry.into_diagnostic().wrap_err("failed to read crates directory entry")?;

        let manifest_path = entry.path().join("Cargo.toml");

        if manifest_path.is_file() {
            manifest_paths.push(manifest_path);
        }
    }

    manifest_paths.sort();

    Ok(manifest_paths)
}

pub fn read_crate_version(manifest_path: &Path) -> Result<String> {
    let manifest = read_manifest(manifest_path)?;

    let version_line = manifest
        .lines()
        .find(|line| is_version_line(line))
        .ok_or_else(|| miette!("no version line found in {}", manifest_path.display()))?;

    let version = version_line.split('"').nth(1).ok_or_else(|| {
        miette!("could not parse version from {}: {version_line}", manifest_path.display())
    })?;

    Ok(version.to_string())
}

pub fn write_crate_version(manifest_path: &Path, release_version: &str) -> Result<()> {
    let manifest = read_manifest(manifest_path)?;

    let mut lines: Vec<String> = manifest.lines().map(str::to_string).collect();

    let version_line = lines
        .iter_mut()
        .find(|line| is_version_line(line))
        .ok_or_else(|| miette!("no version line found in {}", manifest_path.display()))?;

    *version_line = format!("version = \"{release_version}\"");

    let mut updated = lines.join("\n");

    updated.push('\n');

    fs::write(manifest_path, updated)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to write {}", manifest_path.display()))
}

fn read_manifest(manifest_path: &Path) -> Result<String> {
    fs::read_to_string(manifest_path)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read {}", manifest_path.display()))
}

fn is_version_line(line: &str) -> bool {
    line.trim_start().starts_with("version =")
}
