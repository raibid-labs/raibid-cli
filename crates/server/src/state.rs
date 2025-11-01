//! Shared application state

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Application state shared across all handlers
#[derive(Debug)]
pub struct AppState {
    /// Server start time
    start_time: chrono::DateTime<chrono::Utc>,

    /// Request counter
    request_count: AtomicU64,

    /// Active connections
    active_connections: AtomicU64,

    /// Health check status
    health_status: Arc<RwLock<HealthStatus>>,
}

/// Health check status
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HealthStatus {
    /// Overall health status
    pub healthy: bool,

    /// Status message
    pub message: String,

    /// Last check time
    pub last_check: chrono::DateTime<chrono::Utc>,
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self {
            healthy: true,
            message: "OK".to_string(),
            last_check: chrono::Utc::now(),
        }
    }
}

impl AppState {
    /// Create new application state
    pub fn new() -> Self {
        Self {
            start_time: chrono::Utc::now(),
            request_count: AtomicU64::new(0),
            active_connections: AtomicU64::new(0),
            health_status: Arc::new(RwLock::new(HealthStatus::default())),
        }
    }

    /// Get server start time
    pub fn start_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.start_time
    }

    /// Increment request counter
    pub fn increment_requests(&self) {
        self.request_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Get total request count
    pub fn request_count(&self) -> u64 {
        self.request_count.load(Ordering::Relaxed)
    }

    /// Increment active connections
    pub fn increment_connections(&self) {
        self.active_connections.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrement active connections
    pub fn decrement_connections(&self) {
        self.active_connections.fetch_sub(1, Ordering::Relaxed);
    }

    /// Get active connections count
    pub fn active_connections(&self) -> u64 {
        self.active_connections.load(Ordering::Relaxed)
    }

    /// Get health status
    pub async fn health_status(&self) -> HealthStatus {
        self.health_status.read().await.clone()
    }

    /// Update health status
    pub async fn update_health_status(&self, healthy: bool, message: String) {
        let mut status = self.health_status.write().await;
        status.healthy = healthy;
        status.message = message;
        status.last_check = chrono::Utc::now();
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_creation() {
        let state = AppState::new();
        assert_eq!(state.request_count(), 0);
        assert_eq!(state.active_connections(), 0);
    }

    #[test]
    fn test_request_counter() {
        let state = AppState::new();
        assert_eq!(state.request_count(), 0);

        state.increment_requests();
        assert_eq!(state.request_count(), 1);

        state.increment_requests();
        assert_eq!(state.request_count(), 2);
    }

    #[test]
    fn test_connection_counter() {
        let state = AppState::new();
        assert_eq!(state.active_connections(), 0);

        state.increment_connections();
        assert_eq!(state.active_connections(), 1);

        state.increment_connections();
        assert_eq!(state.active_connections(), 2);

        state.decrement_connections();
        assert_eq!(state.active_connections(), 1);
    }

    #[tokio::test]
    async fn test_health_status() {
        let state = AppState::new();
        let status = state.health_status().await;
        assert!(status.healthy);
        assert_eq!(status.message, "OK");

        state
            .update_health_status(false, "Service degraded".to_string())
            .await;
        let status = state.health_status().await;
        assert!(!status.healthy);
        assert_eq!(status.message, "Service degraded");
    }
}
