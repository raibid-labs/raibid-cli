//! Flux Infrastructure Validation Tests
use std::process::Command;
use super::*;

pub struct FluxValidator {
    kubeconfig: String,
    namespace: String,
}

impl FluxValidator {
    pub fn new(kubeconfig: impl Into<String>, namespace: impl Into<String>) -> Self {
        Self {
            kubeconfig: kubeconfig.into(),
            namespace: namespace.into(),
        }
    }

    pub fn default_config() -> Self {
        let kubeconfig = std::env::var("KUBECONFIG").unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
            format!("{}/.kube/config", home)
        });
        Self::new(kubeconfig, "flux-system")
    }

    pub fn validate(&self) -> ValidationSuite {
        let mut suite = ValidationSuite::new("Flux");
        suite.run_test("flux_cli_available", || self.check_flux_cli());
        suite.run_test("namespace_exists", || self.check_namespace_exists());
        suite.run_test_with_details("controllers_running", || self.check_controllers());
        suite.finish();
        suite
    }

    fn check_flux_cli(&self) -> Result<String, String> {
        Command::new("flux").arg("--version").output()
            .map_err(|e| format!("Flux CLI not found: {}", e))
            .and_then(|output| if output.status.success() {
                Ok("Flux CLI available".to_string())
            } else {
                Err("Flux CLI command failed".to_string())
            })
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

    fn check_controllers(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl").arg("get").arg("deployments").arg("-n").arg(&self.namespace)
            .arg("-o").arg("custom-columns=NAME:.metadata.name,READY:.status.readyReplicas")
            .arg("--no-headers").env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to get deployments: {}", e))?;
        if !output.status.success() {
            return Err("Failed to query Flux controller status".to_string());
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        if lines.is_empty() {
            return Err("No Flux controllers found".to_string());
        }
        let mut details = Vec::new();
        for line in &lines {
            details.push(format!("Controller: {}", line.trim()));
        }
        Ok((format!("{} Flux controller(s) found", lines.len()), details))
    }
}
