mod cli;
mod commands;
mod shared;

use clap::Parser;
use cli::Cli;
use commands::Commands;
use miette::Result;
use updates::check::{
    print_update_notice,
    start_version_check,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let cargo_package_version = env!("CARGO_PKG_VERSION");

    let pending_update_notice = start_version_check(cargo_package_version);

    // Store result so update is printed even if command fails
    let command_result = run_command(cli.command).await;

    print_update_notice(pending_update_notice).await;

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
