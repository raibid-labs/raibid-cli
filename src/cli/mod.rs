//! CLI argument parsing and command definitions
//!
//! This module handles all command-line argument parsing using clap.
//! It defines the CLI structure and routes commands to their implementations.

use clap::{Parser, Subcommand};

/// DGX Spark Personal CI Agent Pool
///
/// A TUI-first developer tool for managing self-hosted CI agents
#[derive(Parser, Debug)]
#[command(name = "raibid-cli")]
#[command(version, about, long_about = None)]
#[command(author = "Raibid Labs")]
pub struct Cli {
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available CLI subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    // Placeholder for future subcommands
    // These will be added in CLI-002 and beyond:
    // - Setup
    // - Teardown
    // - Status
    // - Job
    // - Agent
    // - Mirror
    // - Tui
}
