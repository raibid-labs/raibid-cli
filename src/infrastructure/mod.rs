//! Infrastructure provisioning modules
//!
//! This module provides infrastructure component installation and management,
//! including k3s, Gitea, Redis, KEDA, and Flux.
//!
//! It also provides comprehensive error handling, retry logic, pre-flight validation,
//! rollback mechanisms, and health checking capabilities.

pub mod k3s;
pub mod gitea;
pub mod redis;
pub mod keda;
pub mod flux;
pub mod status;

// Error handling and utilities
pub mod error;
pub mod retry;
pub mod preflight;
pub mod rollback;
pub mod healthcheck;

pub use k3s::K3sInstaller;
pub use gitea::GiteaInstaller;
pub use redis::RedisInstaller;
pub use keda::KedaInstaller;
pub use flux::{FluxInstaller, FluxConfig};
pub use status::{
    ComponentStatusChecker, K3sStatusChecker, GiteaStatusChecker,
    RedisStatusChecker, KedaStatusChecker, FluxStatusChecker,
    ComponentHealth, ComponentStatus, ResourceUsage,
};

// Error handling exports
