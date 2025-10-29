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

    /// Job management commands
    Job {
        #[command(subcommand)]
        command: JobCommands,
    },

    /// Agent management commands
    Agent {
        #[command(subcommand)]
        command: AgentCommands,
    },

    /// Repository mirroring commands
    Mirror {
        #[command(subcommand)]
        command: MirrorCommands,
    },
}

/// Job subcommands
#[derive(Subcommand, Debug)]
pub enum JobCommands {
    /// List jobs
    List {
        /// Filter by status (running, success, failed, pending)
        #[arg(long)]
        status: Option<String>,

        /// Filter by repository
        #[arg(long)]
        repo: Option<String>,

        /// Show last N jobs (default: 20)
        #[arg(long, default_value = "20")]
        limit: usize,

        /// Output format (table or json)
        #[arg(long, default_value = "table")]
        output: String,
    },

    /// Show job details
    Show {
        /// Job ID
        job_id: String,

        /// Output format (table or json)
        #[arg(long, default_value = "table")]
        output: String,
    },

    /// Cancel a running job
    Cancel {
        /// Job ID
        job_id: String,

        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Retry a failed job
    Retry {
        /// Job ID
        job_id: String,
    },
}

/// Agent subcommands
#[derive(Subcommand, Debug)]
pub enum AgentCommands {
    /// List agents
    List {
        /// Filter by status
        #[arg(long)]
        status: Option<String>,

        /// Output format (table or json)
        #[arg(long, default_value = "table")]
        output: String,
    },

    /// Show agent details
    Show {
        /// Agent ID
        agent_id: String,

        /// Output format (table or json)
        #[arg(long, default_value = "table")]
        output: String,
    },

    /// Restart an agent
    Restart {
        /// Agent ID
        agent_id: String,

        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Scale agents
    Scale {
        /// Number of agents
        #[arg(long)]
        count: usize,

        /// Minimum agents (default: 0)
        #[arg(long)]
        min: Option<usize>,

        /// Maximum agents (default: 10)
        #[arg(long)]
        max: Option<usize>,
    },
}

/// Mirror subcommands
#[derive(Subcommand, Debug)]
pub enum MirrorCommands {
    /// Add a repository mirror
    Add {
        /// GitHub repository URL
        github_url: String,

        /// Custom mirror name
        #[arg(long)]
        name: Option<String>,

        /// Sync interval in minutes (default: 60)
        #[arg(long, default_value = "60")]
        sync_interval: u32,
    },

    /// List repository mirrors
    List {
        /// Output format (table or json)
        #[arg(long, default_value = "table")]
        output: String,
    },

    /// Sync a repository mirror
    Sync {
        /// Repository URL
        repo: String,

        /// Force sync even if up-to-date
        #[arg(long)]
        force: bool,
    },

    /// Remove a repository mirror
    Remove {
        /// Repository URL
        repo: String,

        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },
}
