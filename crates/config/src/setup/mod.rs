use miette::Result;

use crate::utils::read_config_file;

pub fn setup_swelog_files(overwrite_existing_files: bool) -> Result<()> {
    let swelog_config = read_config_file()?;

    println!("{:?}", swelog_config);

    Ok(())
}
