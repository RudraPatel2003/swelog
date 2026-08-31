use clap::Args;
use config::{
    config_file::read_config_file,
    print::print_config,
};
use miette::Result;

#[derive(Debug, Args)]
pub struct ConfigArgs {}

impl ConfigArgs {
    pub fn run(self) -> Result<()> {
        let _ = self;

        let swelog_config = read_config_file()?;

        print_config(&swelog_config)?;

        Ok(())
    }
}
