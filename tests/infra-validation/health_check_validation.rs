//! Health Check Validation Tests
use std::process::Command;
use super::*;

pub struct HealthCheckValidator {
    kubeconfig: String,
}

impl HealthCheckValidator {
    pub fn new(kubeconfig: impl Into<String>) -> Self {
        Self {
            kubeconfig: kubeconfig.into(),
        }
    }

    pub fn from_env() -> Self {
        let kubeconfig = std::env::var("KUBECONFIG").unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
            format!("{}/.kube/config", home)
        });
        Self::new(kubeconfig)
    }

    pub fn validate(&self) -> ValidationSuite {
        let mut suite = ValidationSuite::new("Health Checks");
        suite.run_test("cluster_health_endpoint", || self.check_cluster_health());
        suite.run_test_with_details("node_conditions", || self.check_node_conditions());
        suite.run_test_with_details("pod_health_summary", || self.check_pod_health_summary());
        suite.finish();
        suite
    }

    fn check_cluster_health(&self) -> Result<String, String> {
        let output = Command::new("kubectl").arg("get").arg("--raw=/healthz")
            .env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to check cluster health: {}", e))?;
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("ok") {
                Ok("Cluster health endpoint returns OK".to_string())
            } else {
                Err(format!("Cluster health endpoint returned: {}", stdout))
            }
        } else {
            Err("Cluster health endpoint check failed".to_string())
        }
    }

    fn check_node_conditions(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl").arg("get").arg("nodes")
            .arg("-o").arg("custom-columns=NAME:.metadata.name,STATUS:.status.conditions[?(@.type=='Ready')].status")
            .arg("--no-headers").env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to check node conditions: {}", e))?;
        if !output.status.success() {
            return Err("Failed to query node conditions".to_string());
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().filter(|l| !l.is_empty()).collect();
        let mut details = Vec::new();
        let mut healthy_count = 0;
        for line in &lines {
            if line.contains("True") { healthy_count += 1; }
            details.push(format!("Node: {}", line.trim()));
        }
        let total = lines.len();
        if healthy_count == total {
            Ok((format!("All {} node(s) are healthy", total), details))
        } else {
            Err(format!("Only {}/{} nodes are healthy", healthy_count, total))
        }
    }

    fn check_pod_health_summary(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl").arg("get").arg("pods").arg("--all-namespaces")
            .arg("-o").arg("custom-columns=STATUS:.status.phase")
            .arg("--no-headers").env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to get pod health summary: {}", e))?;
        if !output.status.success() {
            return Err("Failed to query pod status".to_string());
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        let running = lines.iter().filter(|l| l.contains("Running")).count();
        let total = lines.len();
        let details = vec![format!("Running: {}/{}", running, total)];
        if running > 0 {
            Ok((format!("{}/{} pods running", running, total), details))
        } else {
            Err("No pods are running".to_string())
        }
    }
}
