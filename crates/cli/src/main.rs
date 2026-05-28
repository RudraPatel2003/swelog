mod cli;

use clap::Parser;
use cli::{
    Cli,
    Commands,
};
use config::{
    init::initialize_config_file,
    setup::setup_swelog_files,
};
use logging::log_work_item;
use miette::Result;
use owo_colors::OwoColorize;
use summary::day::{
    get_daily_log_file_name,
    summarize_daily_work,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { overwrite_existing_config } => {
            let config_file_path = initialize_config_file(overwrite_existing_config)?;

            println!("Created swelog config at {}", config_file_path.display().cyan());
        }

        Commands::Setup { overwrite_existing_files } => {
            let swelog_config = setup_swelog_files(overwrite_existing_files)?;

            println!(
                "Created swelog files in your Obsidian vault at {}",
                swelog_config.obsidian_vault_path.display().cyan()
            );
        }

        Commands::Summarize { daily_summary_options, command } => {
            let daily_summary_options = match command {
                Some(cli::SummarizeCommands::Day(daily_summary_options)) => daily_summary_options,
                None => daily_summary_options,
            };

            let log_date = summarize_daily_work(
                daily_summary_options.overwrite_existing_daily_log,
                daily_summary_options.keep_work_file,
            )
            .await?;

            let daily_log_file_name = get_daily_log_file_name(&log_date);

            println!("Successfully summarized your daily work into {}", daily_log_file_name.cyan());
        }

        Commands::Log { work_item } => {
            log_work_item(&work_item)?;

            println!("Logged work item to your work file");
        }
    }

    Ok(())
}
