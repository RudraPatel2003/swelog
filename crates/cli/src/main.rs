mod cli;
mod commands;

use clap::Parser;
use cli::Cli;
use commands::Commands;
use miette::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(init_args) => {
            init_args.run()?;
        }

        Commands::Setup(setup_args) => {
            setup_args.run()?;
        }

        Commands::Summarize(summarize_args) => {
            summarize_args.run().await?;
        }

        Commands::Log(log_args) => {
            log_args.run()?;
        }

        Commands::Reset(reset_args) => {
            reset_args.run()?;
        }

        Commands::Fetch(fetch_args) => {
            fetch_args.run().await?;
        }
    }

    Ok(())
}
