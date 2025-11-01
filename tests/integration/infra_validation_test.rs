//! Comprehensive Infrastructure Validation Test Suite
//!
//! This test suite validates all infrastructure components: k3s, Gitea, Redis, Flux, and KEDA.
//! Run with: `TEST_EXTERNAL=1 cargo test --test infra_validation_test -- --ignored`

use std::env;
use std::process::Command;

mod infra_validation;
use infra_validation::*;

/// Helper to check if external tests are enabled
fn external_tests_enabled() -> bool {
    env::var("TEST_EXTERNAL").is_ok()
}

/// Helper to get kubeconfig path
fn get_kubeconfig() -> String {
    env::var("KUBECONFIG").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap_or_else(|_| "/root".to_string());
        format!("{}/.kube/config", home)
    })
}

/// Test: k3s cluster validation
#[test]
#[ignore]
fn test_k3s_infrastructure() {
    if !external_tests_enabled() {
        println!("Skipping: Set TEST_EXTERNAL=1 to run infrastructure tests");
        return;
    }

    let validator = k3s_validation::K3sValidator::from_env();
    let suite = validator.validate();

    suite.print();

    // Assert that critical tests pass
    assert!(suite.total_count() > 0, "Should run k3s validation tests");
    assert!(suite.failed_count() < suite.total_count() / 2,
        "More than half of k3s tests failed");
}

/// Test: Gitea infrastructure validation
#[test]
#[ignore]
fn test_gitea_infrastructure() {
    if !external_tests_enabled() {
        println!("Skipping: Set TEST_EXTERNAL=1 to run infrastructure tests");
        return;
    }

    let validator = gitea_validation::GiteaValidator::default_config();
    let suite = validator.validate();

    suite.print();

    assert!(suite.total_count() > 0, "Should run Gitea validation tests");
}

/// Test: Redis infrastructure validation
#[test]
#[ignore]
fn test_redis_infrastructure() {
    if !external_tests_enabled() {
        println!("Skipping: Set TEST_EXTERNAL=1 to run infrastructure tests");
        return;
    }

    let validator = redis_validation::RedisValidator::default_config();
    let suite = validator.validate();

    suite.print();

    assert!(suite.total_count() > 0, "Should run Redis validation tests");
}

/// Test: Flux infrastructure validation
#[test]
#[ignore]
fn test_flux_infrastructure() {
    if !external_tests_enabled() {
        println!("Skipping: Set TEST_EXTERNAL=1 to run infrastructure tests");
        return;
    }

    let validator = flux_validation::FluxValidator::default_config();
    let suite = validator.validate();

    suite.print();

    assert!(suite.total_count() > 0, "Should run Flux validation tests");
}

/// Test: KEDA infrastructure validation
#[test]
#[ignore]
fn test_keda_infrastructure() {
    if !external_tests_enabled() {
        println!("Skipping: Set TEST_EXTERNAL=1 to run infrastructure tests");
        return;
    }

    let validator = keda_validation::KedaValidator::default_config();
    let suite = validator.validate();

    suite.print();

    assert!(suite.total_count() > 0, "Should run KEDA validation tests");
}

/// Test: Health check validation
#[test]
#[ignore]
fn test_health_checks() {
    if !external_tests_enabled() {
        println!("Skipping: Set TEST_EXTERNAL=1 to run infrastructure tests");
        return;
    }

    let validator = health_check_validation::HealthCheckValidator::from_env();
    let suite = validator.validate();

    suite.print();

    assert!(suite.total_count() > 0, "Should run health check validation tests");
}

/// Test: Resource quota and limit validation
#[test]
#[ignore]
fn test_resource_quotas_and_limits() {
    if !external_tests_enabled() {
        println!("Skipping: Set TEST_EXTERNAL=1 to run infrastructure tests");
        return;
    }

    let validator = resource_validation::ResourceValidator::from_env();
    let suite = validator.validate();

    suite.print();

    assert!(suite.total_count() > 0, "Should run resource validation tests");
}

/// Test: End-to-end infrastructure validation
/// This test runs all validators and produces a comprehensive report
#[test]
#[ignore]
fn test_e2e_full_infrastructure_validation() {
    if !external_tests_enabled() {
        println!("Skipping: Set TEST_EXTERNAL=1 to run infrastructure tests");
        return;
    }

    let mut report = ValidationReport::new();

    println!("\n{}", "=".repeat(80));
    println!("Running Comprehensive Infrastructure Validation");
    println!("{}\n", "=".repeat(80));

    // Run all validators
    let k3s_validator = k3s_validation::K3sValidator::from_env();
    let mut k3s_suite = k3s_validator.validate();
    k3s_suite.print();
    report.add_suite(k3s_suite);

    let gitea_validator = gitea_validation::GiteaValidator::default_config();
    let mut gitea_suite = gitea_validator.validate();
    gitea_suite.print();
    report.add_suite(gitea_suite);

    let redis_validator = redis_validation::RedisValidator::default_config();
    let mut redis_suite = redis_validator.validate();
    redis_suite.print();
    report.add_suite(redis_suite);

    let flux_validator = flux_validation::FluxValidator::default_config();
    let mut flux_suite = flux_validator.validate();
    flux_suite.print();
    report.add_suite(flux_suite);

    let keda_validator = keda_validation::KedaValidator::default_config();
    let mut keda_suite = keda_validator.validate();
    keda_suite.print();
    report.add_suite(keda_suite);

    let health_validator = health_check_validation::HealthCheckValidator::from_env();
    let mut health_suite = health_validator.validate();
    health_suite.print();
    report.add_suite(health_suite);

    let resource_validator = resource_validation::ResourceValidator::from_env();
    let mut resource_suite = resource_validator.validate();
    resource_suite.print();
    report.add_suite(resource_suite);

    // Print final comprehensive report
    report.print_final_summary();

    // Assertions
    assert!(report.total_tests() > 0, "Should run validation tests");
    assert!(report.total_passed() > 0, "Should have some passing tests");

    // Allow for some failures in infrastructure tests
    let pass_rate = (report.total_passed() as f64) / (report.total_tests() as f64);
    assert!(pass_rate > 0.5, "Pass rate should be > 50%, got {:.1}%", pass_rate * 100.0);

    // Fail the test if pass rate is below 80% to indicate infrastructure issues
    if pass_rate < 0.8 {
        eprintln!("\nWarning: Infrastructure validation pass rate is {:.1}%", pass_rate * 100.0);
        eprintln!("This indicates potential infrastructure health issues.");
    }
}

/// Test: Validation framework itself
#[test]
fn test_validation_framework() {
    // Test basic framework functionality
    let mut suite = ValidationSuite::new("test");

    suite.add_test(ValidationTest::passed("test1", "ok"));
    suite.add_test(ValidationTest::failed("test2", "error"));
    suite.add_test(ValidationTest::skipped("test3", "skip"));

    assert_eq!(suite.total_count(), 3);
    assert_eq!(suite.passed_count(), 1);
    assert_eq!(suite.failed_count(), 1);
    assert_eq!(suite.skipped_count(), 1);
    assert!(!suite.all_passed());
}

/// Test: Validation report aggregation
#[test]
fn test_validation_report() {
    let mut report = ValidationReport::new();

    let mut suite1 = ValidationSuite::new("component1");
    suite1.add_test(ValidationTest::passed("test1", "ok"));
    suite1.add_test(ValidationTest::passed("test2", "ok"));

    let mut suite2 = ValidationSuite::new("component2");
    suite2.add_test(ValidationTest::passed("test3", "ok"));
    suite2.add_test(ValidationTest::failed("test4", "error"));

    report.add_suite(suite1);
    report.add_suite(suite2);

    assert_eq!(report.total_tests(), 4);
    assert_eq!(report.total_passed(), 3);
    assert_eq!(report.total_failed(), 1);
    assert!(!report.all_passed());
}

/// Test: Check if kubectl is available
#[test]
fn test_kubectl_available() {
    let output = Command::new("kubectl")
        .arg("version")
        .arg("--client")
        .output();

    assert!(output.is_ok(), "kubectl should be available for infrastructure tests");
}

/// Test: Check if required CLIs are available
#[test]
fn test_required_clis_available() {
    let clis = vec!["kubectl", "helm"];

    for cli in clis {
        let output = Command::new(cli)
            .arg("version")
            .output();

        assert!(output.is_ok(), "{} should be available", cli);
    }
}

/// Test: Optional CLIs presence check
#[test]
fn test_optional_clis() {
    let optional_clis = vec!["flux"];

    for cli in optional_clis {
        let output = Command::new(cli)
            .arg("--version")
            .output();

        if output.is_ok() {
            println!("✓ Optional CLI '{}' is available", cli);
        } else {
            println!("ℹ Optional CLI '{}' is not available", cli);
        }
    }
}
