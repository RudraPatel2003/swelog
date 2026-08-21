mod cli;
mod commands;

use clap::Parser;
use cli::Cli;
use commands::Commands;
use miette::Result;
use updates::check::{
    print_update_notice,
    refresh_latest_version_cache,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let version_cache_refresh = refresh_latest_version_cache();

    // Store result so update is printed even if command fails
    let command_result = run_command(cli.command).await;

    let cargo_package_version = env!("CARGO_PKG_VERSION");

    print_update_notice(cargo_package_version, version_cache_refresh).await;

    command_result
}

async fn run_command(command: Commands) -> Result<()> {
    match command {
        Commands::Init(init_args) => init_args.run(),

        Commands::Setup(setup_args) => setup_args.run(),

        Commands::Summarize(summarize_args) => summarize_args.run().await,

        Commands::Log(log_args) => log_args.run(),

        Commands::Reset(reset_args) => reset_args.run(),

        Commands::Fetch(fetch_args) => fetch_args.run().await,

        Commands::Config(config_args) => config_args.run(),

        Commands::Auth(auth_args) => auth_args.run(),
    }
}
