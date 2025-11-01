//! raibid-agent
//!
//! CI agent runner that polls the job queue and executes builds.
//! This crate handles:
//! - Job polling from Redis Streams
//! - Build execution in isolated environments
//! - Cache management for dependencies
//! - Result reporting back to the server
//!
//! ## Architecture
//!
//! The agent uses Redis Streams as a job queue with consumer groups for reliable
//! job processing. Each agent:
//! 1. Connects to Redis and joins a consumer group
//! 2. Polls for new jobs using XREADGROUP
//! 3. Updates job status to "running"
//! 4. Clones the repository
//! 5. Executes the build pipeline
//! 6. Reports results and acknowledges the message
//!
//! ## Example
//!
//! ```no_run
//! use raibid_agent::{Agent, AgentConfig};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = AgentConfig::default();
//!     let agent = Agent::new(config).await?;
//!     agent.run().await?;
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod consumer;
pub mod error;
pub mod executor;
pub mod git;

use std::sync::Arc;
use tokio::signal;
use tracing::info;

pub use config::AgentConfig;
pub use error::{AgentError, AgentResult};

/// Main agent struct
pub struct Agent {
    config: Arc<AgentConfig>,
    consumer: consumer::JobConsumer,
}

impl Agent {
    /// Create a new agent instance
    pub async fn new(config: AgentConfig) -> AgentResult<Self> {
        let config = Arc::new(config);
        let consumer = consumer::JobConsumer::new(config.clone()).await?;

        Ok(Self { config, consumer })
    }

    /// Run the agent
    pub async fn run(self) -> AgentResult<()> {
        info!("Starting raibid-agent {}", self.config.agent_id);

        // Run consumer with graceful shutdown
        tokio::select! {
            result = self.consumer.run() => {
                result?;
            }
            _ = shutdown_signal() => {
                info!("Received shutdown signal");
            }
        }

        info!("Agent shutdown complete");
        Ok(())
    }
}

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C signal");
        },
        _ = terminate => {
            info!("Received terminate signal");
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_config_default() {
        let config = AgentConfig::default();
        assert!(!config.agent_id.is_empty());
    }
}
