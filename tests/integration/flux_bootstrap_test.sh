#!/usr/bin/env bash
# Integration tests for Flux bootstrap script
# Tests the bootstrap.sh script functionality

set -euo pipefail

# Colors
readonly GREEN='\033[0;32m'
readonly RED='\033[0;31m'
readonly YELLOW='\033[1;33m'
readonly NC='\033[0m'

# Test tracking
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Directories
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
readonly FLUX_DIR="$PROJECT_ROOT/infra/flux"

# Test helpers
log_test() {
    echo -e "${YELLOW}[TEST]${NC} $*"
    ((TESTS_RUN++))
}

log_pass() {
    echo -e "${GREEN}[PASS]${NC} $*"
    ((TESTS_PASSED++))
}

log_fail() {
    echo -e "${RED}[FAIL]${NC} $*"
    ((TESTS_FAILED++))
}

# Test: Bootstrap script exists
test_bootstrap_script_exists() {
    log_test "Bootstrap script exists"

    if [[ -f "$FLUX_DIR/bootstrap.sh" ]]; then
        log_pass "bootstrap.sh found"
        return 0
    else
        log_fail "bootstrap.sh not found"
        return 1
    fi
}

# Test: Bootstrap script is executable
test_bootstrap_script_executable() {
    log_test "Bootstrap script is executable"

    if [[ -x "$FLUX_DIR/bootstrap.sh" ]]; then
        log_pass "bootstrap.sh is executable"
        return 0
    else
        log_fail "bootstrap.sh is not executable"
        return 1
    fi
}

# Test: Bootstrap script has valid shebang
test_bootstrap_script_shebang() {
    log_test "Bootstrap script has valid shebang"

    local first_line
    first_line=$(head -n1 "$FLUX_DIR/bootstrap.sh")

    if [[ "$first_line" == "#!/usr/bin/env bash" ]] || [[ "$first_line" == "#!/bin/bash" ]]; then
        log_pass "Valid shebang found"
        return 0
    else
        log_fail "Invalid shebang: $first_line"
        return 1
    fi
}

# Test: Bootstrap script passes shellcheck
test_bootstrap_script_shellcheck() {
    log_test "Bootstrap script passes shellcheck"

    if ! command -v shellcheck &> /dev/null; then
        log_pass "shellcheck not available, skipping"
        return 0
    fi

    if shellcheck "$FLUX_DIR/bootstrap.sh"; then
        log_pass "shellcheck passed"
        return 0
    else
        log_fail "shellcheck found issues"
        return 1
    fi
}

# Test: Validation script exists
test_validation_script_exists() {
    log_test "Validation script exists"

    if [[ -f "$FLUX_DIR/validate.sh" ]]; then
        log_pass "validate.sh found"
        return 0
    else
        log_fail "validate.sh not found"
        return 1
    fi
}

# Test: Validation script is executable
test_validation_script_executable() {
    log_test "Validation script is executable"

    if [[ -x "$FLUX_DIR/validate.sh" ]]; then
        log_pass "validate.sh is executable"
        return 0
    else
        log_fail "validate.sh is not executable"
        return 1
    fi
}

# Test: Validation script passes shellcheck
test_validation_script_shellcheck() {
    log_test "Validation script passes shellcheck"

    if ! command -v shellcheck &> /dev/null; then
        log_pass "shellcheck not available, skipping"
        return 0
    fi

    if shellcheck "$FLUX_DIR/validate.sh"; then
        log_pass "shellcheck passed"
        return 0
    else
        log_fail "shellcheck found issues"
        return 1
    fi
}

# Test: Flux manifests exist
test_flux_manifests_exist() {
    log_test "Flux manifests exist"

    local manifests=(
        "namespace.yaml"
        "gitrepository.yaml"
        "kustomization.yaml"
    )

    local all_exist=true
    for manifest in "${manifests[@]}"; do
        if [[ ! -f "$FLUX_DIR/$manifest" ]]; then
            log_fail "Missing manifest: $manifest"
            all_exist=false
        fi
    done

    if $all_exist; then
        log_pass "All required manifests exist"
        return 0
    else
        return 1
    fi
}

# Test: Flux manifests are valid YAML
test_flux_manifests_valid_yaml() {
    log_test "Flux manifests are valid YAML"

    if ! command -v yamllint &> /dev/null; then
        log_pass "yamllint not available, skipping"
        return 0
    fi

    local manifests=(
        "$FLUX_DIR/namespace.yaml"
        "$FLUX_DIR/gitrepository.yaml"
        "$FLUX_DIR/kustomization.yaml"
    )

    local all_valid=true
    for manifest in "${manifests[@]}"; do
        if [[ -f "$manifest" ]]; then
            if ! yamllint -d relaxed "$manifest" &> /dev/null; then
                log_fail "Invalid YAML: $manifest"
                all_valid=false
            fi
        fi
    done

    if $all_valid; then
        log_pass "All manifests are valid YAML"
        return 0
    else
        return 1
    fi
}

# Test: Flux manifests can be validated with kubectl
test_flux_manifests_kubectl_validate() {
    log_test "Flux manifests kubectl validation"

    if ! command -v kubectl &> /dev/null; then
        log_pass "kubectl not available, skipping"
        return 0
    fi

    local manifests=(
        "$FLUX_DIR/namespace.yaml"
        "$FLUX_DIR/gitrepository.yaml"
        "$FLUX_DIR/kustomization.yaml"
    )

    local all_valid=true
    for manifest in "${manifests[@]}"; do
        if [[ -f "$manifest" ]]; then
            if ! kubectl apply --dry-run=client -f "$manifest" &> /dev/null; then
                log_fail "kubectl validation failed: $manifest"
                all_valid=false
            fi
        fi
    done

    if $all_valid; then
        log_pass "All manifests passed kubectl validation"
        return 0
    else
        return 1
    fi
}

# Test: Flux system directory structure
test_flux_system_directory() {
    log_test "Flux system directory structure"

    if [[ -d "$FLUX_DIR/flux-system" ]]; then
        log_pass "flux-system directory exists"
        return 0
    else
        log_fail "flux-system directory not found"
        return 1
    fi
}

# Test: Flux system kustomization exists
test_flux_system_kustomization() {
    log_test "Flux system kustomization exists"

    if [[ -f "$FLUX_DIR/flux-system/kustomization.yaml" ]]; then
        log_pass "flux-system kustomization.yaml found"
        return 0
    else
        log_fail "flux-system kustomization.yaml not found"
        return 1
    fi
}

# Test: README exists and is not empty
test_readme_exists() {
    log_test "README exists and is not empty"

    if [[ -f "$FLUX_DIR/README.md" ]] && [[ -s "$FLUX_DIR/README.md" ]]; then
        log_pass "README.md exists and has content"
        return 0
    else
        log_fail "README.md missing or empty"
        return 1
    fi
}

# Test: Troubleshooting guide exists
test_troubleshooting_guide_exists() {
    log_test "Troubleshooting guide exists"

    if [[ -f "$FLUX_DIR/TROUBLESHOOTING.md" ]] && [[ -s "$FLUX_DIR/TROUBLESHOOTING.md" ]]; then
        log_pass "TROUBLESHOOTING.md exists and has content"
        return 0
    else
        log_fail "TROUBLESHOOTING.md missing or empty"
        return 1
    fi
}

# Test: Bootstrap script has required functions
test_bootstrap_script_functions() {
    log_test "Bootstrap script has required functions"

    local required_functions=(
        "check_prerequisites"
        "install_flux_cli"
        "setup_gitea_credentials"
        "install_flux_components"
        "verify_flux"
    )

    local all_found=true
    for func in "${required_functions[@]}"; do
        if ! grep -q "^${func}()" "$FLUX_DIR/bootstrap.sh"; then
            log_fail "Missing function: $func"
            all_found=false
        fi
    done

    if $all_found; then
        log_pass "All required functions found"
        return 0
    else
        return 1
    fi
}

# Test: Validation script has required functions
test_validation_script_functions() {
    log_test "Validation script has required functions"

    local required_functions=(
        "test_kubectl"
        "test_flux_cli"
        "test_flux_controllers"
        "test_gitrepository_exists"
        "test_kustomization_exists"
    )

    local all_found=true
    for func in "${required_functions[@]}"; do
        if ! grep -q "^${func}()" "$FLUX_DIR/validate.sh"; then
            log_fail "Missing function: $func"
            all_found=false
        fi
    done

    if $all_found; then
        log_pass "All required validation functions found"
        return 0
    else
        return 1
    fi
}

# Run all tests
main() {
    echo ""
    echo "=========================================="
    echo "  Flux Bootstrap Tests"
    echo "=========================================="
    echo ""

    # Script tests
    test_bootstrap_script_exists || true
    test_bootstrap_script_executable || true
    test_bootstrap_script_shebang || true
    test_bootstrap_script_shellcheck || true
    test_bootstrap_script_functions || true

    test_validation_script_exists || true
    test_validation_script_executable || true
    test_validation_script_shellcheck || true
    test_validation_script_functions || true

    # Manifest tests
    test_flux_manifests_exist || true
    test_flux_manifests_valid_yaml || true
    test_flux_manifests_kubectl_validate || true

    # Directory structure tests
    test_flux_system_directory || true
    test_flux_system_kustomization || true

    # Documentation tests
    test_readme_exists || true
    test_troubleshooting_guide_exists || true

    # Results
    echo ""
    echo "=========================================="
    echo "  Test Results"
    echo "=========================================="
    echo ""
    echo "Tests run:    $TESTS_RUN"
    echo -e "${GREEN}Tests passed: $TESTS_PASSED${NC}"
    echo -e "${RED}Tests failed: $TESTS_FAILED${NC}"
    echo ""

    if [[ $TESTS_FAILED -eq 0 ]]; then
        echo -e "${GREEN}All tests passed!${NC}"
        exit 0
    else
        echo -e "${RED}Some tests failed${NC}"
        exit 1
    fi
}

# Run tests
main "$@"
