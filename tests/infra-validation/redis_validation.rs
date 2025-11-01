//! Redis Infrastructure Validation Tests
use std::process::Command;
use super::*;

pub struct RedisValidator {
    kubeconfig: String,
    namespace: String,
    release_name: String,
}

impl RedisValidator {
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
        Self::new(kubeconfig, "raibid-redis", "raibid-redis")
    }

    pub fn validate(&self) -> ValidationSuite {
        let mut suite = ValidationSuite::new("Redis");
        suite.run_test("namespace_exists", || self.check_namespace_exists());
        suite.run_test_with_details("master_running", || self.check_master_running());
        suite.run_test_with_details("service_exists", || self.check_service());
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

    fn check_master_running(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl").arg("get").arg("pods").arg("-n").arg(&self.namespace)
            .arg("-l").arg("app.kubernetes.io/component=master")
            .arg("-o").arg("custom-columns=NAME:.metadata.name,STATUS:.status.phase")
            .arg("--no-headers").env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to get master pods: {}", e))?;
        if !output.status.success() {
            return Err("Failed to query Redis master status".to_string());
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        if lines.is_empty() {
            return Err("No Redis master pod found".to_string());
        }
        let mut details = Vec::new();
        for line in &lines {
            details.push(format!("Master pod: {}", line.trim()));
        }
        if lines[0].contains("Running") {
            Ok(("Redis master is Running".to_string(), details))
        } else {
            Err("Redis master is not ready".to_string())
        }
    }

    fn check_service(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl").arg("get").arg("service").arg("-n").arg(&self.namespace)
            .arg("-l").arg("app.kubernetes.io/name=redis")
            .arg("-o").arg("custom-columns=NAME:.metadata.name,PORT:.spec.ports[0].port")
            .arg("--no-headers").env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to get services: {}", e))?;
        if !output.status.success() {
            return Err("Failed to query services".to_string());
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        if lines.is_empty() {
            return Err("No Redis service found".to_string());
        }
        let details: Vec<String> = lines.iter().map(|l| format!("Service: {}", l.trim())).collect();
        Ok((format!("{} Redis service(s) found", lines.len()), details))
    }
}
