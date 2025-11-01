//! k3s Infrastructure Validation Tests

use std::process::Command;
use super::*;

pub struct K3sValidator {
    kubeconfig: String,
}

impl K3sValidator {
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

    pub fn validate(&self) -> ValidationSuite {
        let mut suite = ValidationSuite::new("k3s");
        suite.run_test("kubectl_available", || self.check_kubectl_available());
        suite.run_test("cluster_accessible", || self.check_cluster_accessible());
        suite.run_test_with_details("nodes_ready", || self.check_nodes_ready());
        suite.run_test("api_server_responsive", || self.check_api_server());
        suite.run_test("kube_system_namespace", || self.check_kube_system_namespace());
        suite.run_test_with_details("system_pods_running", || self.check_system_pods());
        suite.run_test_with_details("coredns_running", || self.check_coredns());
        suite.run_test("metrics_server_available", || self.check_metrics_server());
        suite.run_test_with_details("cluster_version", || self.check_cluster_version());
        suite.run_test("traefik_running", || self.check_traefik());
        suite.finish();
        suite
    }

    fn check_kubectl_available(&self) -> Result<String, String> {
        Command::new("kubectl").arg("version").arg("--client").arg("--short").output()
            .map_err(|e| format!("kubectl not found: {}", e))
            .and_then(|output| if output.status.success() {
                Ok("kubectl is available".to_string())
            } else {
                Err("kubectl command failed".to_string())
            })
    }

    fn check_cluster_accessible(&self) -> Result<String, String> {
        let output = Command::new("kubectl").arg("cluster-info")
            .env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to access cluster: {}", e))?;
        if output.status.success() {
            Ok("Cluster is accessible".to_string())
        } else {
            Err(format!("Cluster not accessible: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }

    fn check_nodes_ready(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl").arg("get").arg("nodes")
            .arg("-o").arg("custom-columns=NAME:.metadata.name,STATUS:.status.conditions[?(@.type=='Ready')].status,VERSION:.status.nodeInfo.kubeletVersion")
            .arg("--no-headers").env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to get nodes: {}", e))?;
        if !output.status.success() {
            return Err("Failed to query node status".to_string());
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        if lines.is_empty() {
            return Err("No nodes found in cluster".to_string());
        }
        let mut details = Vec::new();
        let mut ready_count = 0;
        for line in &lines {
            if line.contains("True") { ready_count += 1; }
            details.push(format!("Node: {}", line.trim()));
        }
        let total = lines.len();
        if ready_count == total {
            Ok((format!("All {} node(s) are Ready", total), details))
        } else {
            Err(format!("Only {}/{} nodes are Ready", ready_count, total))
        }
    }

    fn check_api_server(&self) -> Result<String, String> {
        let output = Command::new("kubectl").arg("get").arg("--raw=/healthz")
            .env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to check API server health: {}", e))?;
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("ok") {
                Ok("API server is healthy".to_string())
            } else {
                Err(format!("API server health check returned: {}", stdout))
            }
        } else {
            Err("API server health check failed".to_string())
        }
    }

    fn check_kube_system_namespace(&self) -> Result<String, String> {
        let output = Command::new("kubectl").arg("get").arg("namespace").arg("kube-system")
            .env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to check kube-system namespace: {}", e))?;
        if output.status.success() {
            Ok("kube-system namespace exists".to_string())
        } else {
            Err("kube-system namespace not found".to_string())
        }
    }

    fn check_system_pods(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl").arg("get").arg("pods").arg("-n").arg("kube-system")
            .arg("-o").arg("custom-columns=NAME:.metadata.name,STATUS:.status.phase,READY:.status.conditions[?(@.type=='Ready')].status")
            .arg("--no-headers").env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to get system pods: {}", e))?;
        if !output.status.success() {
            return Err("Failed to query system pods".to_string());
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        if lines.is_empty() {
            return Err("No system pods found".to_string());
        }
        let mut details = Vec::new();
        let mut running_count = 0;
        for line in &lines {
            if line.contains("Running") && line.contains("True") { running_count += 1; }
            details.push(format!("Pod: {}", line.trim()));
        }
        let total = lines.len();
        if running_count == total {
            Ok((format!("All {} system pod(s) are Running and Ready", total), details))
        } else {
            Err(format!("Only {}/{} system pods are Running and Ready", running_count, total))
        }
    }

    fn check_coredns(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl").arg("get").arg("pods").arg("-n").arg("kube-system")
            .arg("-l").arg("k8s-app=kube-dns")
            .arg("-o").arg("custom-columns=NAME:.metadata.name,STATUS:.status.phase,READY:.status.conditions[?(@.type=='Ready')].status")
            .arg("--no-headers").env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to get CoreDNS pods: {}", e))?;
        if !output.status.success() {
            return Err("Failed to query CoreDNS status".to_string());
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        if lines.is_empty() {
            return Err("CoreDNS pods not found".to_string());
        }
        let mut details = Vec::new();
        let mut ready_count = 0;
        for line in &lines {
            if line.contains("Running") && line.contains("True") { ready_count += 1; }
            details.push(format!("Pod: {}", line.trim()));
        }
        let total = lines.len();
        if ready_count > 0 {
            Ok((format!("{}/{} CoreDNS pod(s) are ready", ready_count, total), details))
        } else {
            Err("No CoreDNS pods are ready".to_string())
        }
    }

    fn check_metrics_server(&self) -> Result<String, String> {
        let output = Command::new("kubectl").arg("get").arg("deployment").arg("metrics-server")
            .arg("-n").arg("kube-system").env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|_e| "metrics-server not found (optional)".to_string())?;
        if output.status.success() {
            Ok("metrics-server is deployed".to_string())
        } else {
            Ok("metrics-server not found (optional)".to_string())
        }
    }

    fn check_cluster_version(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl").arg("version").arg("--short")
            .env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to get cluster version: {}", e))?;
        if !output.status.success() {
            return Err("Failed to query cluster version".to_string());
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<String> = stdout.lines().map(|s| s.to_string()).collect();
        if lines.is_empty() {
            return Err("No version information returned".to_string());
        }
        Ok(("Cluster version retrieved".to_string(), lines))
    }

    fn check_traefik(&self) -> Result<String, String> {
        let output = Command::new("kubectl").arg("get").arg("pods").arg("-n").arg("kube-system")
            .arg("-l").arg("app.kubernetes.io/name=traefik").arg("--no-headers")
            .env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to check Traefik: {}", e))?;
        if !output.status.success() {
            return Err("Failed to query Traefik status".to_string());
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.is_empty() {
            Ok("Traefik ingress not found (may be disabled)".to_string())
        } else {
            Ok("Traefik ingress is running".to_string())
        }
    }
}
