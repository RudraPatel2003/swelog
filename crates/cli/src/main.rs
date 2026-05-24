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
use logging::log::{
    get_daily_log_file_name,
    log_daily_work,
};
use miette::Result;
use owo_colors::OwoColorize;

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

        Commands::Log { overwrite_existing_daily_log, keep_work_file } => {
            let log_date = log_daily_work(overwrite_existing_daily_log, keep_work_file).await?;

            let daily_log_file_name = get_daily_log_file_name(&log_date);

            println!("Succesfully logged your daily work into {}", daily_log_file_name.cyan());
        }
    }

    Ok(())
}
