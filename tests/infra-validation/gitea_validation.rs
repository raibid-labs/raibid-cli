//! Gitea Infrastructure Validation Tests
use std::process::Command;
use super::*;

pub struct GiteaValidator {
    kubeconfig: String,
    namespace: String,
    release_name: String,
}

impl GiteaValidator {
    pub fn new(kubeconfig: impl Into<String>, namespace: impl Into<String>, release_name: impl Into<String>) -> Self {
        Self {
            kubeconfig: kubeconfig.into(),
            namespace: namespace.into(),
            release_name: release_name.into(),
        }
    }

    pub fn default_config() -> Self {
        let kubeconfig = std::env::var("KUBECONFIG").unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
            format!("{}/.kube/config", home)
        });
        Self::new(kubeconfig, "gitea", "gitea")
    }

    pub fn validate(&self) -> ValidationSuite {
        let mut suite = ValidationSuite::new("Gitea");
        suite.run_test("namespace_exists", || self.check_namespace_exists());
        suite.run_test("helm_release_exists", || self.check_helm_release());
        suite.run_test_with_details("pods_running", || self.check_pods_running());
        suite.run_test_with_details("service_exists", || self.check_service());
        suite.run_test_with_details("pvcs_bound", || self.check_pvcs());
        suite.finish();
        suite
    }

    fn check_namespace_exists(&self) -> Result<String, String> {
        let output = Command::new("kubectl").arg("get").arg("namespace").arg(&self.namespace)
            .env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to check namespace: {}", e))?;
        if output.status.success() {
            Ok(format!("Namespace '{}' exists", self.namespace))
        } else {
            Err(format!("Namespace '{}' not found", self.namespace))
        }
    }

    fn check_helm_release(&self) -> Result<String, String> {
        let output = Command::new("helm").arg("list").arg("--namespace").arg(&self.namespace)
            .arg("--filter").arg(&self.release_name).arg("-o").arg("json")
            .env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to list Helm releases: {}", e))?;
        if !output.status.success() {
            return Err("helm list command failed".to_string());
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains(&self.release_name) {
            Ok(format!("Helm release '{}' exists", self.release_name))
        } else {
            Err(format!("Helm release '{}' not found", self.release_name))
        }
    }

    fn check_pods_running(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl").arg("get").arg("pods").arg("-n").arg(&self.namespace)
            .arg("-l").arg("app.kubernetes.io/name=gitea")
            .arg("-o").arg("custom-columns=NAME:.metadata.name,STATUS:.status.phase")
            .arg("--no-headers").env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to get pods: {}", e))?;
        if !output.status.success() {
            return Err("Failed to query pod status".to_string());
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        if lines.is_empty() {
            return Err("No Gitea pods found".to_string());
        }
        let mut details = Vec::new();
        let mut running_count = 0;
        for line in &lines {
            if line.contains("Running") { running_count += 1; }
            details.push(format!("Pod: {}", line.trim()));
        }
        let total = lines.len();
        if running_count == total {
            Ok((format!("All {} Gitea pod(s) are Running", total), details))
        } else {
            Err(format!("Only {}/{} Gitea pods are Running", running_count, total))
        }
    }

    fn check_service(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl").arg("get").arg("service").arg("-n").arg(&self.namespace)
            .arg("-l").arg("app.kubernetes.io/name=gitea")
            .arg("-o").arg("custom-columns=NAME:.metadata.name,TYPE:.spec.type,PORT:.spec.ports[0].port")
            .arg("--no-headers").env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to get services: {}", e))?;
        if !output.status.success() {
            return Err("Failed to query services".to_string());
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        if lines.is_empty() {
            return Err("No Gitea service found".to_string());
        }
        let details: Vec<String> = lines.iter().map(|l| format!("Service: {}", l.trim())).collect();
        Ok((format!("{} Gitea service(s) found", lines.len()), details))
    }

    fn check_pvcs(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl").arg("get").arg("pvc").arg("-n").arg(&self.namespace)
            .arg("-o").arg("custom-columns=NAME:.metadata.name,STATUS:.status.phase")
            .arg("--no-headers").env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to get PVCs: {}", e))?;
        if !output.status.success() {
            return Err("Failed to query PVC status".to_string());
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        if lines.is_empty() {
            return Err("No PVCs found in namespace".to_string());
        }
        let mut details = Vec::new();
        let mut bound_count = 0;
        for line in &lines {
            if line.contains("Bound") { bound_count += 1; }
            details.push(format!("PVC: {}", line.trim()));
        }
        let total = lines.len();
        if bound_count == total {
            Ok((format!("All {} PVC(s) are Bound", total), details))
        } else {
            Err(format!("Only {}/{} PVCs are Bound", bound_count, total))
        }
    }
}
