//! Infrastructure provisioning modules
//!
//! This module provides infrastructure component installation and management,
//! including k3s, Gitea, Redis, KEDA, and Flux.

pub mod k3s;
pub mod gitea;
pub mod redis;
pub mod keda;
pub mod flux;

pub use k3s::K3sInstaller;
pub use gitea::{GiteaInstaller, GiteaConfig, ServiceType};
pub use redis::{RedisInstaller, RedisConfig, RedisConnectionInfo, RedisStreamsConfig};
pub use keda::{KedaInstaller, KedaConfig, ScaledObjectConfig, TargetKind};
pub use flux::{FluxInstaller, FluxConfig};
