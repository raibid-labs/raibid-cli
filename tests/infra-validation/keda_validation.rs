//! KEDA Infrastructure Validation Tests
use std::process::Command;
use super::*;

pub struct KedaValidator {
    kubeconfig: String,
    namespace: String,
}

impl KedaValidator {
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
        Self::new(kubeconfig, "keda")
    }

    pub fn validate(&self) -> ValidationSuite {
        let mut suite = ValidationSuite::new("KEDA");
        suite.run_test("namespace_exists", || self.check_namespace_exists());
        suite.run_test_with_details("operator_running", || self.check_operator_running());
        suite.run_test_with_details("crds_installed", || self.check_crds());
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

    fn check_operator_running(&self) -> Result<(String, Vec<String>), String> {
        let output = Command::new("kubectl").arg("get").arg("pods").arg("-n").arg(&self.namespace)
            .arg("-l").arg("app=keda-operator")
            .arg("-o").arg("custom-columns=NAME:.metadata.name,STATUS:.status.phase")
            .arg("--no-headers").env("KUBECONFIG", &self.kubeconfig).output()
            .map_err(|e| format!("Failed to get KEDA operator pods: {}", e))?;
        if !output.status.success() {
            return Err("Failed to query KEDA operator status".to_string());
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        if lines.is_empty() {
            return Err("No KEDA operator pods found".to_string());
        }
        let mut details = Vec::new();
        for line in &lines {
            details.push(format!("Operator pod: {}", line.trim()));
        }
        if lines[0].contains("Running") {
            Ok(("KEDA operator is Running".to_string(), details))
        } else {
            Err("KEDA operator not ready".to_string())
        }
    }

    fn check_crds(&self) -> Result<(String, Vec<String>), String> {
        let expected_crds = vec!["scaledobjects.keda.sh", "scaledjobs.keda.sh", "triggerauthentications.keda.sh"];
        let mut details = Vec::new();
        let mut found_count = 0;
        for crd in &expected_crds {
            let output = Command::new("kubectl").arg("get").arg("crd").arg(crd)
                .env("KUBECONFIG", &self.kubeconfig).output()
                .map_err(|e| format!("Failed to check CRD {}: {}", crd, e))?;
            if output.status.success() {
                found_count += 1;
                details.push(format!("CRD found: {}", crd));
            } else {
                details.push(format!("CRD missing: {}", crd));
            }
        }
        let total = expected_crds.len();
        if found_count == total {
            Ok((format!("All {} KEDA CRD(s) are installed", total), details))
        } else {
            Err(format!("Only {}/{} KEDA CRDs are installed", found_count, total))
        }
    }
}
