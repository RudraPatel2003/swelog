use miette::Result;

use crate::utils::get_rust_cli_version;

pub fn run_list_version() -> Result<()> {
    let rust_cli_version = get_rust_cli_version()?;

    println!("CLI crate version: {rust_cli_version}");

    Ok(())
}
