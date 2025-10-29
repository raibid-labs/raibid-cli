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
    /// Setup infrastructure component
    Setup {
        /// Component to setup (k3s, gitea, redis, keda, flux, all)
        component: String,
    },

    /// Teardown infrastructure component
    Teardown {
        /// Component to teardown (k3s, gitea, redis, keda, flux, all)
        component: String,
    },

    /// Show status of infrastructure components
    Status {
        /// Component to check status (k3s, gitea, redis, keda, flux, all, or omit for all)
        component: Option<String>,
    },
    // Future subcommands (CLI-003+):
    // - Job
    // - Agent
    // - Mirror
    // - Tui
}
