use clap::Args;
use config::config_file::read_config_file;
use miette::Result;
use owo_colors::{
    OwoColorize,
    Stream,
};

use crate::{
    commands::fetch::sources::{
        FetchSource,
        FetchSourceAvailability,
        collect_fetch_source_availabilities,
    },
    environment::Environment,
};

const LABEL_WIDTH: usize = 20;

#[derive(Debug, Args)]
pub struct StatusArgs {}

impl StatusArgs {
    pub fn run(self, environment: &Environment) -> Result<()> {
        let _ = self;

        let swelog_config = read_config_file(&environment.config_file_path)?;

        println!("Fetch commands included in `swelog fetch all`:");

        println!();

        for (fetch_source, availability) in collect_fetch_source_availabilities(&swelog_config)? {
            println!(
                "{:LABEL_WIDTH$}{}",
                fetch_source.label(),
                describe_fetch_source_availability(fetch_source, availability)
            );
        }

        println!();

        println!("Run `swelog auth status` to see every credential swelog has stored.");

        Ok(())
    }
}

fn describe_fetch_source_availability(
    fetch_source: FetchSource,
    availability: FetchSourceAvailability,
) -> String {
    match availability {
        FetchSourceAvailability::Included => {
            format!("{}", "included".if_supports_color(Stream::Stdout, |status| status.green()))
        }

        FetchSourceAvailability::MissingAuthorization => {
            let description =
                format!("not included, {} is not stored", fetch_source.credential().label());

            format!("{}", description.if_supports_color(Stream::Stdout, |text| text.dimmed()))
        }

        FetchSourceAvailability::MissingConfiguration { reason } => {
            let description = format!("not included, {reason}");

            format!("{}", description.if_supports_color(Stream::Stdout, |text| text.dimmed()))
        }
    }
}
