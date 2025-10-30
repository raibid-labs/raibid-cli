//! Infrastructure provisioning modules
//!
//! This module provides infrastructure component installation and management,
//! including k3s, Gitea, Redis, KEDA, and Flux.

pub mod k3s;

pub use k3s::K3sInstaller;
