# Infrastructure Validation Test Suite

Comprehensive validation tests for k3s, Gitea, Redis, Flux, and KEDA infrastructure components.

## Overview

This test suite provides automated validation of the entire raibid-ci infrastructure stack with:

- Colored terminal output for easy readability
- Detailed validation reports
- Individual component validation
- End-to-end infrastructure validation
- Health check validation
- Resource quota and limit validation

## Test Structure

```
tests/infra-validation/
├── mod.rs                        # Test framework with colored output
├── k3s_validation.rs             # k3s cluster validation
├── gitea_validation.rs           # Gitea Git service validation
├── redis_validation.rs           # Redis validation
├── flux_validation.rs            # Flux GitOps validation
├── keda_validation.rs            # KEDA autoscaling validation
├── health_check_validation.rs    # Health check validation
├── resource_validation.rs        # Resource quota/limit validation
└── README.md                     # This file
```

## Running Tests

### Prerequisites

- k3s cluster running and accessible
- kubectl configured with appropriate kubeconfig
- helm installed
- Infrastructure components deployed (Gitea, Redis, Flux, KEDA)

### Environment Variables

- `TEST_EXTERNAL=1` - Enable infrastructure tests (required)
- `KUBECONFIG` - Path to kubeconfig file (optional, defaults to ~/.kube/config)

### Run All Infrastructure Validation Tests

```bash
TEST_EXTERNAL=1 cargo test --test integration -- --ignored
```

### Run Individual Component Tests

```bash
# k3s validation only
TEST_EXTERNAL=1 cargo test --test integration test_k3s_infrastructure -- --ignored

# Gitea validation only
TEST_EXTERNAL=1 cargo test --test integration test_gitea_infrastructure -- --ignored

# Redis validation only
TEST_EXTERNAL=1 cargo test --test integration test_redis_infrastructure -- --ignored

# Flux validation only
TEST_EXTERNAL=1 cargo test --test integration test_flux_infrastructure -- --ignored

# KEDA validation only
TEST_EXTERNAL=1 cargo test --test integration test_keda_infrastructure -- --ignored

# Health checks
TEST_EXTERNAL=1 cargo test --test integration test_health_checks -- --ignored

# Resource quotas and limits
TEST_EXTERNAL=1 cargo test --test integration test_resource_quotas_and_limits -- --ignored
```

### Run End-to-End Validation

This runs all validators and produces a comprehensive report:

```bash
TEST_EXTERNAL=1 cargo test --test integration test_e2e_full_infrastructure_validation -- --ignored
```

## Test Components

### k3s Validation (`k3s_validation.rs`)

Validates k3s cluster infrastructure:

- kubectl availability
- Cluster accessibility
- Node readiness status
- API server health
- kube-system namespace
- System pods status
- CoreDNS status
- Metrics server (optional)
- Cluster version
- Traefik ingress (optional)

### Gitea Validation (`gitea_validation.rs`)

Validates Gitea Git service:

- Namespace existence
- Helm release status
- Pod running status
- Service configuration
- PVC binding status

### Redis Validation (`redis_validation.rs`)

Validates Redis infrastructure:

- Namespace existence
- Master pod status
- Replica pods (if configured)
- Service configuration
- PVC binding (if persistence enabled)

### Flux Validation (`flux_validation.rs`)

Validates Flux GitOps system:

- Flux CLI availability
- Namespace existence
- Controller deployments
- Source controller health
- Kustomize controller health
- Helm controller health
- GitRepository resources
- Kustomization resources
- HelmRelease resources

### KEDA Validation (`keda_validation.rs`)

Validates KEDA autoscaling infrastructure:

- Namespace existence
- Operator pod status
- Metrics server status
- CRD installation
- ScaledObject resources
- ScaledJob resources
- TriggerAuthentication resources

### Health Check Validation (`health_check_validation.rs`)

Validates cluster health:

- Cluster health endpoint
- Node conditions
- Pod health summary across namespaces
- Service endpoints
- Liveness and readiness probes

### Resource Validation (`resource_validation.rs`)

Validates resource management:

- Resource quotas
- Limit ranges
- Node resource capacity and allocatable
- Pod resource requests and limits
- Pods without resource limits
- Resource usage (if metrics-server available)
- Namespace resource allocation
- Storage resources (PVCs)

## Test Framework Features

### Colored Output

The test framework provides colored terminal output:

- **GREEN**: Passed tests
- **RED**: Failed tests
- **YELLOW**: Skipped/Warning tests
- **CYAN**: Section headers

### Detailed Reporting

Each test provides:

- Test name and status
- Execution duration
- Success/failure message
- Additional details (e.g., pod names, versions)

### Validation Reports

The framework aggregates results:

- Total tests run
- Pass/fail/skip/warning counts
- Individual component summaries
- Overall infrastructure health status

## Test Output Example

```
=== k3s Validation ===
  [PASS] kubectl_available (12ms)
      kubectl is available
  [PASS] cluster_accessible (234ms)
      Cluster is accessible
  [PASS] nodes_ready (156ms)
      All 1 node(s) are Ready
      Node: dgx-node     True   v1.28.5+k3s1
  [PASS] api_server_responsive (45ms)
      API server is healthy

Summary: 10 tests, 10 passed, 0 failed, 0 skipped, 0 warnings (1.2s)
All tests passed!
```

## Test Acceptance Criteria

Tests meet issue #64 requirements:

- All tests pass on healthy cluster
- Tests detect misconfigurations
- Test output is clear and actionable
- Tests run in under 5 minutes
- Ready for CI integration

## Continuous Integration

### GitHub Actions Integration

Add to `.github/workflows/infra-validation.yml`:

```yaml
name: Infrastructure Validation

on:
  workflow_dispatch:
  schedule:
    - cron: '0 */6 * * *'  # Every 6 hours

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Set up k3s
        # ... k3s setup steps
      - name: Deploy infrastructure
        # ... deploy Gitea, Redis, Flux, KEDA
      - name: Run validation tests
        run: TEST_EXTERNAL=1 cargo test --test integration -- --ignored
```

## Troubleshooting

### Tests fail with "cluster not accessible"

- Check KUBECONFIG environment variable
- Verify k3s cluster is running
- Ensure kubectl can access cluster

### Tests fail with "namespace not found"

- Ensure infrastructure components are deployed
- Check namespace names match defaults or provide custom config

### Tests timeout

- Increase test timeout in CI configuration
- Check if cluster resources are constrained
- Verify network connectivity

## Contributing

When adding new infrastructure components:

1. Create validator in `tests/infra-validation/<component>_validation.rs`
2. Implement validator following existing patterns
3. Add test to `tests/integration/infra_validation_test.rs`
4. Update this README
5. Run tests to verify

## Related Issues

- Issue #64: Infrastructure Validation Test Suite (this implementation)
- Issue #56-60: Infrastructure component dependencies

---

**Generated**: 2025-11-01
**Issue**: #64
