use miette::Result;

use crate::{
    swelog_config::SwelogConfig,
    utils::read_config_file,
};

pub fn setup_swelog_files(overwrite_existing_files: bool) -> Result<SwelogConfig> {
    let swelog_config = read_config_file()?;

    Ok(swelog_config)
}
