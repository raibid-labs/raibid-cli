//! Infrastructure Validation Test Framework
//!
//! This module provides a comprehensive test framework for validating infrastructure
//! components with colored output and detailed reporting.

pub mod k3s_validation;
pub mod gitea_validation;
pub mod redis_validation;
pub mod flux_validation;
pub mod keda_validation;
pub mod health_check_validation;
pub mod resource_validation;

pub use k3s_validation::K3sValidator;
pub use gitea_validation::GiteaValidator;
pub use redis_validation::RedisValidator;
pub use flux_validation::FluxValidator;
pub use keda_validation::KedaValidator;
pub use health_check_validation::HealthCheckValidator;
pub use resource_validation::ResourceValidator;

use std::fmt;
use std::time::{Duration, Instant};

/// ANSI color codes for terminal output
pub mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const BOLD: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";
}

/// Test result status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
    Warning,
}

impl fmt::Display for TestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestStatus::Passed => write!(f, "{}PASS{}", colors::GREEN, colors::RESET),
            TestStatus::Failed => write!(f, "{}FAIL{}", colors::RED, colors::RESET),
            TestStatus::Skipped => write!(f, "{}SKIP{}", colors::YELLOW, colors::RESET),
            TestStatus::Warning => write!(f, "{}WARN{}", colors::YELLOW, colors::RESET),
        }
    }
}

/// Individual validation test result
#[derive(Debug, Clone)]
pub struct ValidationTest {
    pub name: String,
    pub status: TestStatus,
    pub message: String,
    pub duration: Duration,
    pub details: Vec<String>,
}

impl ValidationTest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: TestStatus::Passed,
            message: String::new(),
            duration: Duration::from_secs(0),
            details: Vec::new(),
        }
    }

    pub fn passed(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: TestStatus::Passed,
            message: message.into(),
            duration: Duration::from_secs(0),
            details: Vec::new(),
        }
    }

    pub fn failed(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: TestStatus::Failed,
            message: message.into(),
            duration: Duration::from_secs(0),
            details: Vec::new(),
        }
    }

    pub fn skipped(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: TestStatus::Skipped,
            message: message.into(),
            duration: Duration::from_secs(0),
            details: Vec::new(),
        }
    }

    pub fn warning(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: TestStatus::Warning,
            message: message.into(),
            duration: Duration::from_secs(0),
            details: Vec::new(),
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn add_detail(&mut self, detail: impl Into<String>) {
        self.details.push(detail.into());
    }

    pub fn print(&self) {
        println!(
            "  [{}] {} {}({}ms){}",
            self.status,
            self.name,
            colors::DIM,
            self.duration.as_millis(),
            colors::RESET
        );

        if !self.message.is_empty() {
            println!("      {}{}{}", colors::DIM, self.message, colors::RESET);
        }

        for detail in &self.details {
            println!("      {}{}{}", colors::DIM, detail, colors::RESET);
        }
    }
}

/// Component validation suite
pub struct ValidationSuite {
    pub component: String,
    pub tests: Vec<ValidationTest>,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
}

impl ValidationSuite {
    pub fn new(component: impl Into<String>) -> Self {
        Self {
            component: component.into(),
            tests: Vec::new(),
            start_time: Instant::now(),
            end_time: None,
        }
    }

    pub fn add_test(&mut self, test: ValidationTest) {
        self.tests.push(test);
    }

    pub fn run_test<F>(&mut self, name: impl Into<String>, test_fn: F)
    where
        F: FnOnce() -> Result<String, String>,
    {
        let name = name.into();
        let start = Instant::now();

        let test = match test_fn() {
            Ok(message) => ValidationTest::passed(&name, message),
            Err(message) => ValidationTest::failed(&name, message),
        };

        self.tests.push(test.with_duration(start.elapsed()));
    }

    pub fn run_test_with_details<F>(&mut self, name: impl Into<String>, test_fn: F)
    where
        F: FnOnce() -> Result<(String, Vec<String>), String>,
    {
        let name = name.into();
        let start = Instant::now();

        let mut test = match test_fn() {
            Ok((message, details)) => {
                let mut t = ValidationTest::passed(&name, message);
                t.details = details;
                t
            }
            Err(message) => ValidationTest::failed(&name, message),
        };

        test.duration = start.elapsed();
        self.tests.push(test);
    }

    pub fn finish(&mut self) {
        self.end_time = Some(Instant::now());
    }

    pub fn passed_count(&self) -> usize {
        self.tests.iter().filter(|t| t.status == TestStatus::Passed).count()
    }

    pub fn failed_count(&self) -> usize {
        self.tests.iter().filter(|t| t.status == TestStatus::Failed).count()
    }

    pub fn skipped_count(&self) -> usize {
        self.tests.iter().filter(|t| t.status == TestStatus::Skipped).count()
    }

    pub fn warning_count(&self) -> usize {
        self.tests.iter().filter(|t| t.status == TestStatus::Warning).count()
    }

    pub fn total_count(&self) -> usize {
        self.tests.len()
    }

    pub fn all_passed(&self) -> bool {
        self.tests.iter().all(|t| t.status == TestStatus::Passed || t.status == TestStatus::Skipped)
    }

    pub fn duration(&self) -> Duration {
        match self.end_time {
            Some(end) => end.duration_since(self.start_time),
            None => Instant::now().duration_since(self.start_time),
        }
    }

    pub fn print_header(&self) {
        println!(
            "\n{}{}=== {} Validation ==={}",
            colors::BOLD,
            colors::CYAN,
            self.component,
            colors::RESET
        );
    }

    pub fn print_summary(&self) {
        println!();
        let duration = self.duration();

        println!(
            "{}Summary:{} {} tests, {} passed, {} failed, {} skipped, {} warnings {}({}ms){}",
            colors::BOLD,
            colors::RESET,
            self.total_count(),
            self.passed_count(),
            self.failed_count(),
            self.skipped_count(),
            self.warning_count(),
            colors::DIM,
            duration.as_millis(),
            colors::RESET
        );

        if self.all_passed() {
            println!("{}{}All tests passed!{}", colors::BOLD, colors::GREEN, colors::RESET);
        } else {
            println!("{}{}Some tests failed!{}", colors::BOLD, colors::RED, colors::RESET);
        }
    }

    pub fn print(&self) {
        self.print_header();
        for test in &self.tests {
            test.print();
        }
        self.print_summary();
    }
}

/// Validation report aggregating multiple suites
pub struct ValidationReport {
    pub suites: Vec<ValidationSuite>,
    pub start_time: Instant,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            suites: Vec::new(),
            start_time: Instant::now(),
        }
    }

    pub fn add_suite(&mut self, suite: ValidationSuite) {
        self.suites.push(suite);
    }

    pub fn total_tests(&self) -> usize {
        self.suites.iter().map(|s| s.total_count()).sum()
    }

    pub fn total_passed(&self) -> usize {
        self.suites.iter().map(|s| s.passed_count()).sum()
    }

    pub fn total_failed(&self) -> usize {
        self.suites.iter().map(|s| s.failed_count()).sum()
    }

    pub fn total_skipped(&self) -> usize {
        self.suites.iter().map(|s| s.skipped_count()).sum()
    }

    pub fn total_warnings(&self) -> usize {
        self.suites.iter().map(|s| s.warning_count()).sum()
    }

    pub fn all_passed(&self) -> bool {
        self.suites.iter().all(|s| s.all_passed())
    }

    pub fn duration(&self) -> Duration {
        Instant::now().duration_since(self.start_time)
    }

    pub fn print_final_summary(&self) {
        println!("\n{}{}==========================================={}",
            colors::BOLD, colors::CYAN, colors::RESET);
        println!("{}{}    Infrastructure Validation Report    {}",
            colors::BOLD, colors::CYAN, colors::RESET);
        println!("{}{}==========================================={}",
            colors::BOLD, colors::CYAN, colors::RESET);

        for suite in &self.suites {
            let status = if suite.all_passed() {
                format!("{}PASS{}", colors::GREEN, colors::RESET)
            } else {
                format!("{}FAIL{}", colors::RED, colors::RESET)
            };
            println!(
                "  [{}] {} ({} tests, {}ms)",
                status,
                suite.component,
                suite.total_count(),
                suite.duration().as_millis()
            );
        }

        println!("\n{}Total:{} {} tests, {} passed, {} failed, {} skipped, {} warnings",
            colors::BOLD,
            colors::RESET,
            self.total_tests(),
            self.total_passed(),
            self.total_failed(),
            self.total_skipped(),
            self.total_warnings()
        );

        let duration = self.duration();
        println!("{}Duration:{} {:.2}s", colors::BOLD, colors::RESET, duration.as_secs_f64());

        if self.all_passed() {
            println!("\n{}{}All infrastructure validation tests passed!{}",
                colors::BOLD, colors::GREEN, colors::RESET);
        } else {
            println!("\n{}{}Some infrastructure validation tests failed!{}",
                colors::BOLD, colors::RED, colors::RESET);
        }
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}
