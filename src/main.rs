mod cli;
mod commands;
mod config;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

use cli::Cli;

fn main() -> Result<()> {
    // Initialize logging
    setup_logging()?;

    // Parse CLI arguments
    let cli = Cli::parse();

    // Load configuration
    let _config = config::Config::load()?;

    // Handle commands
    match cli.command {
        None => {
            // No subcommand provided, show help
            println!("No command specified. Use --help for usage information.");
            std::process::exit(1);
        }
        Some(cmd) => match cmd {
            cli::Commands::Setup { component } => {
                // Parse component string
                let component = component
                    .parse()
                    .map_err(|e| anyhow::anyhow!("Invalid component: {}", e))?;
                commands::setup::execute(component)
            }
            cli::Commands::Teardown { component } => {
                // Parse component string
                let component = component
                    .parse()
                    .map_err(|e| anyhow::anyhow!("Invalid component: {}", e))?;
                commands::teardown::execute(component)
            }
            cli::Commands::Status { component } => {
                // Parse optional component string
                let component = match component {
                    Some(c) => Some(
                        c.parse()
                            .map_err(|e| anyhow::anyhow!("Invalid component: {}", e))?,
                    ),
                    None => None,
                };
                commands::status::execute(component)
            }
            cli::Commands::Job { command } => commands::job::execute(command),
            cli::Commands::Agent { command } => commands::agent::execute(command),
            cli::Commands::Mirror { command } => commands::mirror::execute(command),
        },
    }
}

fn setup_logging() -> Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    Ok(())
}
