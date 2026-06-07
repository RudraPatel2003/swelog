use clap::Args;
use config::{
    print::print_config,
    utils::read_config_file,
};
use miette::Result;

#[derive(Debug, Args)]
pub struct ConfigArgs {}

impl ConfigArgs {
    pub fn run(self) -> Result<()> {
        let swelog_config = read_config_file()?;

        print_config(&swelog_config)?;

        Ok(())
    }
}
