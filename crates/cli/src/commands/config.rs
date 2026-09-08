use clap::Args;
use config::{
    config_file::read_config_file,
    print::print_config,
};
use miette::Result;

use crate::environment::Environment;

#[derive(Debug, Args)]
pub struct ConfigArgs {}

impl ConfigArgs {
    pub fn run(self, environment: &Environment) -> Result<()> {
        let _ = self;

        let swelog_config = read_config_file(&environment.config_file_path)?;

        print_config(&environment.config_file_path, &swelog_config);

        Ok(())
    }
}
