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
pub use gitea::{GiteaInstaller, GiteaConfig, ServiceType};
pub use redis::{RedisInstaller, RedisConfig, RedisConnectionInfo, RedisStreamsConfig};
pub use keda::{KedaInstaller, KedaConfig, ScaledObjectConfig, TargetKind};
pub use flux::{FluxInstaller, FluxConfig};
pub use status::{
    ComponentStatusChecker, K3sStatusChecker, GiteaStatusChecker,
    RedisStatusChecker, KedaStatusChecker, FluxStatusChecker,
    ComponentHealth, ComponentStatus, PodStatus, ResourceUsage,
    VersionInfo, EndpointInfo,
};

// Error handling exports
pub use error::{InfraError, InfraResult, InstallPhase, HelmOperation, ValidationError, ErrorContext};
pub use retry::{RetryConfig, retry_with_backoff, retry_with_backoff_async, poll_until, poll_until_async};
pub use preflight::{
    SystemRequirements, PreFlightValidator, PreFlightResult,
    k3s_requirements, gitea_requirements, redis_requirements,
    keda_requirements, flux_requirements,
};
pub use rollback::{RollbackManager, RollbackContext, RollbackAction};
pub use healthcheck::{
    HealthStatus, HealthCheckResult, CheckResult,
    K3sHealthChecker, HelmHealthChecker,
};
