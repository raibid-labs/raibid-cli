//! Resource Quota and Limit Validation Tests
//!
//! Tests for validating resource quotas, limits, and usage.

use std::process::Command;
use super::*;

/// Resource validator
pub struct ResourceValidator {
    kubeconfig: String,
}

impl ResourceValidator {
    pub fn new(kubeconfig: impl Into<String>) -> Self {
        Self {
            kubeconfig: kubeconfig.into(),
        }
    }

    pub fn from_env() -> Self {
        let kubeconfig = std::env::var("KUBECONFIG")
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
                format!("{}/.kube/config", home)
            });
        Self::new(kubeconfig)
    }

    /// Run all resource validation tests
    pub fn validate(&self) -> ValidationSuite {
        let mut suite = ValidationSuite::new("Resources");

        // Check resource quotas
        suite.run_test_with_details("resource_quotas", || self.check_resource_quotas());

        // Check limit ranges
        suite.run_test_with_details("limit_ranges", || self.check_limit_ranges());

        // Check node capacity and allocatable resources
        suite.run_test_with_details("node_resources", || self.check_node_resources());

        // Check pod resource requests and limits
        suite.run_test_with_details("pod_resource_definitions", || self.check_pod_resources());

        // Check for pods without resource limits
        suite.run_test_with_details("pods_without_limits", || self.check_pods_without_limits());

        // Check resource usage (if metrics-server available)
        suite.run_test_with_details("resource_usage", || self.check_resource_usage());

        // Check namespace resource allocation
        suite.run_test_with_details("namespace_resources", || self.check_namespace_resources());

        // Check PVC storage allocations
        suite.run_test_with_details("storage_resources", || self.check_storage_resources());

        suite.finish();
        suite
    }

    fn check_resource_quotas(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl")
            .arg("get")
            .arg("resourcequotas")
            .arg("--all-namespaces")
            .arg("-o")
            .arg("custom-columns=NAMESPACE:.metadata.namespace,NAME:.metadata.name,HARD:.status.hard,USED:.status.used")
            .arg("--no-headers")
            .env("KUBECONFIG", &self.kubeconfig)
            .output()
            .map_err(|e| format!("Failed to get resource quotas: {}", e))?;

        if !output.status.success() {
            return Err("Failed to query resource quotas".to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();

        if lines.is_empty() {
            return Ok(("No resource quotas configured".to_string(), vec![]));
        }

        let details: Vec<String> = lines.iter().map(|l| format!("Quota: {}", l.trim())).collect();
        Ok((format!("{} resource quota(s) configured", lines.len()), details))
    }

    fn check_limit_ranges(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl")
            .arg("get")
            .arg("limitranges")
            .arg("--all-namespaces")
            .arg("-o")
            .arg("custom-columns=NAMESPACE:.metadata.namespace,NAME:.metadata.name")
            .arg("--no-headers")
            .env("KUBECONFIG", &self.kubeconfig)
            .output()
            .map_err(|e| format!("Failed to get limit ranges: {}", e))?;

        if !output.status.success() {
            return Err("Failed to query limit ranges".to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();

        if lines.is_empty() {
            return Ok(("No limit ranges configured".to_string(), vec![]));
        }

        let details: Vec<String> = lines.iter().map(|l| format!("LimitRange: {}", l.trim())).collect();
        Ok((format!("{} limit range(s) configured", lines.len()), details))
    }

    fn check_node_resources(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl")
            .arg("get")
            .arg("nodes")
            .arg("-o")
            .arg("custom-columns=NAME:.metadata.name,CPU-CAPACITY:.status.capacity.cpu,MEMORY-CAPACITY:.status.capacity.memory,CPU-ALLOCATABLE:.status.allocatable.cpu,MEMORY-ALLOCATABLE:.status.allocatable.memory")
            .arg("--no-headers")
            .env("KUBECONFIG", &self.kubeconfig)
            .output()
            .map_err(|e| format!("Failed to get node resources: {}", e))?;

        if !output.status.success() {
            return Err("Failed to query node resources".to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();

        if lines.is_empty() {
            return Err("No nodes found".to_string());
        }

        let details: Vec<String> = lines.iter().map(|l| format!("Node: {}", l.trim())).collect();
        Ok((format!("{} node(s) with resource information", lines.len()), details))
    }

    fn check_pod_resources(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl")
            .arg("get")
            .arg("pods")
            .arg("--all-namespaces")
            .arg("-o")
            .arg("jsonpath={range .items[*]}{.metadata.namespace}/{.metadata.name}: CPU-REQ={.spec.containers[*].resources.requests.cpu}, MEM-REQ={.spec.containers[*].resources.requests.memory}, CPU-LIM={.spec.containers[*].resources.limits.cpu}, MEM-LIM={.spec.containers[*].resources.limits.memory}{\"\n\"}{end}")
            .env("KUBECONFIG", &self.kubeconfig)
            .output()
            .map_err(|e| format!("Failed to get pod resources: {}", e))?;

        if !output.status.success() {
            return Err("Failed to query pod resources".to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().filter(|l| !l.is_empty()).collect();

        let pods_with_requests = lines.iter()
            .filter(|l| l.contains("CPU-REQ=") && !l.contains("CPU-REQ=,"))
            .count();

        let pods_with_limits = lines.iter()
            .filter(|l| l.contains("CPU-LIM=") && !l.contains("CPU-LIM=,"))
            .count();

        let total = lines.len();
        let details = vec![
            format!("Total pods: {}", total),
            format!("Pods with resource requests: {}", pods_with_requests),
            format!("Pods with resource limits: {}", pods_with_limits),
        ];

        Ok((format!("Resource definitions: {}/{} with requests, {}/{} with limits",
            pods_with_requests, total, pods_with_limits, total), details))
    }

    fn check_pods_without_limits(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl")
            .arg("get")
            .arg("pods")
            .arg("--all-namespaces")
            .arg("-o")
            .arg("jsonpath={range .items[*]}{.metadata.namespace}/{.metadata.name}: CPU-LIM={.spec.containers[*].resources.limits.cpu}, MEM-LIM={.spec.containers[*].resources.limits.memory}{\"\n\"}{end}")
            .env("KUBECONFIG", &self.kubeconfig)
            .output()
            .map_err(|e| format!("Failed to check pods without limits: {}", e))?;

        if !output.status.success() {
            return Err("Failed to query pod resource limits".to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().filter(|l| !l.is_empty()).collect();

        let mut pods_without_limits = Vec::new();

        for line in &lines {
            // Check if both CPU and memory limits are empty
            if (line.contains("CPU-LIM=,") || line.contains("CPU-LIM= ")) &&
               (line.contains("MEM-LIM=,") || line.contains("MEM-LIM= ")) {
                if let Some(pod_name) = line.split(':').next() {
                    // Filter out system pods which might not need limits
                    if !pod_name.contains("kube-system") {
                        pods_without_limits.push(format!("No limits: {}", pod_name.trim()));
                    }
                }
            }
        }

        let details = if pods_without_limits.is_empty() {
            vec![format!("All user pods have resource limits configured")]
        } else {
            pods_without_limits.clone()
        };

        if pods_without_limits.len() < 10 {
            Ok((format!("{} pod(s) without resource limits", pods_without_limits.len()), details))
        } else {
            Err(format!("{} pods are missing resource limits", pods_without_limits.len()))
        }
    }

    fn check_resource_usage(&self) -> Result<(String, Vec<String>), String> {
        // Check if metrics-server is available
        let check_output = Command::new("kubectl")
            .arg("top")
            .arg("nodes")
            .env("KUBECONFIG", &self.kubeconfig)
            .output()
            .map_err(|e| format!("Failed to check resource usage: {}", e))?;

        if !check_output.status.success() {
            return Ok(("Metrics server not available (resource usage not available)".to_string(), vec![]));
        }

        let stdout = String::from_utf8_lossy(&check_output.stdout);
        let lines: Vec<&str> = stdout.lines().skip(1).collect(); // Skip header

        if lines.is_empty() {
            return Ok(("No resource usage data available".to_string(), vec![]));
        }

        let details: Vec<String> = lines.iter().map(|l| format!("Node usage: {}", l.trim())).collect();
        Ok((format!("Resource usage data available for {} node(s)", lines.len()), details))
    }

    fn check_namespace_resources(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl")
            .arg("get")
            .arg("namespaces")
            .arg("-o")
            .arg("custom-columns=NAME:.metadata.name")
            .arg("--no-headers")
            .env("KUBECONFIG", &self.kubeconfig)
            .output()
            .map_err(|e| format!("Failed to get namespaces: {}", e))?;

        if !output.status.success() {
            return Err("Failed to query namespaces".to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let namespaces: Vec<&str> = stdout.lines().collect();

        let mut details = Vec::new();

        for ns in &namespaces {
            let ns = ns.trim();
            if ns.is_empty() {
                continue;
            }

            // Count pods in namespace
            let pod_output = Command::new("kubectl")
                .arg("get")
                .arg("pods")
                .arg("-n")
                .arg(ns)
                .arg("--no-headers")
                .env("KUBECONFIG", &self.kubeconfig)
                .output();

            if let Ok(pod_output) = pod_output {
                let pod_count = String::from_utf8_lossy(&pod_output.stdout)
                    .lines()
                    .count();
                details.push(format!("Namespace {}: {} pod(s)", ns, pod_count));
            }
        }

        Ok((format!("{} namespace(s) analyzed", namespaces.len()), details))
    }

    fn check_storage_resources(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl")
            .arg("get")
            .arg("pvc")
            .arg("--all-namespaces")
            .arg("-o")
            .arg("custom-columns=NAMESPACE:.metadata.namespace,NAME:.metadata.name,STATUS:.status.phase,CAPACITY:.status.capacity.storage,STORAGECLASS:.spec.storageClassName")
            .arg("--no-headers")
            .env("KUBECONFIG", &self.kubeconfig)
            .output()
            .map_err(|e| format!("Failed to get PVCs: {}", e))?;

        if !output.status.success() {
            return Err("Failed to query PVC resources".to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();

        if lines.is_empty() {
            return Ok(("No PVCs configured".to_string(), vec![]));
        }

        let bound_count = lines.iter().filter(|l| l.contains("Bound")).count();
        let total = lines.len();

        let details: Vec<String> = lines.iter()
            .take(10) // Limit to first 10 for readability
            .map(|l| format!("PVC: {}", l.trim()))
            .collect();

        if bound_count == total {
            Ok((format!("All {} PVC(s) are bound", total), details))
        } else {
            Err(format!("Only {}/{} PVCs are bound", bound_count, total))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_validator_creation() {
        let validator = ResourceValidator::new("/tmp/kubeconfig");
        assert_eq!(validator.kubeconfig, "/tmp/kubeconfig");
    }

    #[test]
    fn test_resource_validator_from_env() {
        let _validator = ResourceValidator::from_env();
    }

    #[test]
    #[ignore] // Only run with actual cluster
    fn test_resource_validation_suite() {
        let validator = ResourceValidator::from_env();
        let suite = validator.validate();

        suite.print();
        assert!(suite.total_count() > 0);
    }
}
