use miette::Result;

use crate::crate_versions::read_cli_version;

pub fn run_list_release_tag() -> Result<()> {
    let cli_version = read_cli_version()?;

    println!("v{cli_version}");

    Ok(())
}
