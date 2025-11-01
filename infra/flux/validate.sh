#!/usr/bin/env bash
# Flux CD Validation Script
# Validates Flux installation and configuration

set -euo pipefail

# Colors for output
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly NC='\033[0m' # No Color

# Configuration
readonly FLUX_NAMESPACE="${FLUX_NAMESPACE:-flux-system}"
readonly GITEA_NAMESPACE="${GITEA_NAMESPACE:-raibid-gitea}"

# Test results
TESTS_PASSED=0
TESTS_FAILED=0

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $*"
}

log_warning() {
    echo -e "${YELLOW}[WARN]${NC} $*"
}

log_failure() {
    echo -e "${RED}[FAIL]${NC} $*"
}

# Test helper function
run_test() {
    local test_name="$1"
    local test_command="$2"

    echo ""
    log_info "Testing: $test_name"

    if eval "$test_command"; then
        log_success "$test_name"
        ((TESTS_PASSED++))
        return 0
    else
        log_failure "$test_name"
        ((TESTS_FAILED++))
        return 1
    fi
}

# Test: kubectl connectivity
test_kubectl() {
    kubectl cluster-info &> /dev/null
}

# Test: Flux CLI installed
test_flux_cli() {
    command -v flux &> /dev/null
}

# Test: Flux namespace exists
test_flux_namespace() {
    kubectl get namespace "$FLUX_NAMESPACE" &> /dev/null
}

# Test: Flux controllers running
test_flux_controllers() {
    local controllers=(
        "source-controller"
        "kustomize-controller"
        "helm-controller"
        "notification-controller"
    )

    for controller in "${controllers[@]}"; do
        if ! kubectl get deployment -n "$FLUX_NAMESPACE" | grep -q "$controller"; then
            log_failure "Controller $controller not found"
            return 1
        fi

        if ! kubectl get pods -n "$FLUX_NAMESPACE" -l "app=$controller" \
            -o jsonpath='{.items[0].status.phase}' 2>/dev/null | grep -q "Running"; then
            log_failure "Controller $controller not running"
            return 1
        fi
    done

    return 0
}

# Test: Flux check command
test_flux_check() {
    flux check &> /dev/null
}

# Test: GitRepository resource exists
test_gitrepository_exists() {
    kubectl get gitrepository raibid-infrastructure -n "$FLUX_NAMESPACE" &> /dev/null
}

# Test: GitRepository is ready
test_gitrepository_ready() {
    local status
    status=$(kubectl get gitrepository raibid-infrastructure -n "$FLUX_NAMESPACE" \
        -o jsonpath='{.status.conditions[?(@.type=="Ready")].status}' 2>/dev/null || echo "Unknown")

    [[ "$status" == "True" ]]
}

# Test: GitRepository artifact available
test_gitrepository_artifact() {
    local artifact
    artifact=$(kubectl get gitrepository raibid-infrastructure -n "$FLUX_NAMESPACE" \
        -o jsonpath='{.status.artifact.revision}' 2>/dev/null || echo "")

    [[ -n "$artifact" ]]
}

# Test: Kustomization resource exists
test_kustomization_exists() {
    kubectl get kustomization raibid-ci-infrastructure -n "$FLUX_NAMESPACE" &> /dev/null
}

# Test: Kustomization is ready
test_kustomization_ready() {
    local status
    status=$(kubectl get kustomization raibid-ci-infrastructure -n "$FLUX_NAMESPACE" \
        -o jsonpath='{.status.conditions[?(@.type=="Ready")].status}' 2>/dev/null || echo "Unknown")

    # Kustomization may not be ready if manifests directory doesn't exist yet
    if [[ "$status" == "True" ]]; then
        return 0
    elif [[ "$status" == "False" ]]; then
        local reason
        reason=$(kubectl get kustomization raibid-ci-infrastructure -n "$FLUX_NAMESPACE" \
            -o jsonpath='{.status.conditions[?(@.type=="Ready")].reason}' 2>/dev/null || echo "")

        # It's OK if the path doesn't exist yet
        if [[ "$reason" == *"path not found"* ]] || [[ "$reason" == *"PathNotFound"* ]]; then
            log_warning "Kustomization path not found (this is expected if manifests directory doesn't exist yet)"
            return 0
        fi

        return 1
    else
        return 1
    fi
}

# Test: Gitea credentials secret exists
test_gitea_secret() {
    kubectl get secret gitea-credentials -n "$FLUX_NAMESPACE" &> /dev/null
}

# Test: Source controller connectivity to Gitea
test_gitea_connectivity() {
    kubectl logs -n "$FLUX_NAMESPACE" -l app=source-controller --tail=100 \
        | grep -q "raibid-infrastructure" 2>/dev/null
}

# Test: No failed reconciliations
test_no_failures() {
    local failed_sources
    failed_sources=$(flux get sources git --all-namespaces 2>/dev/null \
        | grep -c "False" || echo "0")

    [[ "$failed_sources" -eq 0 ]]
}

# Test: Image automation controllers (optional)
test_image_automation() {
    local image_controllers=(
        "image-reflector-controller"
        "image-automation-controller"
    )

    for controller in "${image_controllers[@]}"; do
        if kubectl get deployment -n "$FLUX_NAMESPACE" "$controller" &> /dev/null; then
            if ! kubectl get pods -n "$FLUX_NAMESPACE" -l "app=$controller" \
                -o jsonpath='{.items[0].status.phase}' 2>/dev/null | grep -q "Running"; then
                log_warning "Optional controller $controller not running"
                return 1
            fi
        fi
    done

    return 0
}

# Display detailed status
show_detailed_status() {
    echo ""
    log_info "Detailed Flux Status:"
    echo ""

    echo "=== Flux Check ==="
    flux check || true
    echo ""

    echo "=== GitRepository Status ==="
    flux get sources git --all-namespaces || true
    echo ""

    echo "=== Kustomization Status ==="
    flux get kustomizations --all-namespaces || true
    echo ""

    echo "=== Flux Controllers ==="
    kubectl get pods -n "$FLUX_NAMESPACE" || true
    echo ""

    echo "=== Recent GitRepository Events ==="
    kubectl get events -n "$FLUX_NAMESPACE" \
        --field-selector involvedObject.kind=GitRepository \
        --sort-by='.lastTimestamp' | tail -10 || true
    echo ""

    echo "=== Recent Kustomization Events ==="
    kubectl get events -n "$FLUX_NAMESPACE" \
        --field-selector involvedObject.kind=Kustomization \
        --sort-by='.lastTimestamp' | tail -10 || true
    echo ""
}

# Main validation
main() {
    echo ""
    echo "=========================================="
    echo "  Flux CD Validation"
    echo "=========================================="

    # Core validation tests
    run_test "Kubernetes cluster connectivity" "test_kubectl"
    run_test "Flux CLI installed" "test_flux_cli"
    run_test "Flux namespace exists" "test_flux_namespace"
    run_test "Flux controllers running" "test_flux_controllers"
    run_test "Flux system health check" "test_flux_check"

    # GitRepository validation
    run_test "GitRepository resource exists" "test_gitrepository_exists"
    run_test "GitRepository is ready" "test_gitrepository_ready"
    run_test "GitRepository artifact available" "test_gitrepository_artifact"

    # Kustomization validation
    run_test "Kustomization resource exists" "test_kustomization_exists"
    run_test "Kustomization is ready" "test_kustomization_ready"

    # Credentials and connectivity
    run_test "Gitea credentials secret exists" "test_gitea_secret"
    run_test "Source controller can access Gitea" "test_gitea_connectivity"

    # Health checks
    run_test "No failed source reconciliations" "test_no_failures"

    # Optional features
    run_test "Image automation controllers (optional)" "test_image_automation" || true

    # Display results
    echo ""
    echo "=========================================="
    echo "  Validation Results"
    echo "=========================================="
    echo ""
    echo -e "${GREEN}Tests Passed:${NC} $TESTS_PASSED"
    echo -e "${RED}Tests Failed:${NC} $TESTS_FAILED"
    echo ""

    if [[ $TESTS_FAILED -eq 0 ]]; then
        log_success "All validation tests passed!"
        echo ""
        show_detailed_status
        exit 0
    else
        log_failure "Some validation tests failed"
        echo ""
        show_detailed_status
        exit 1
    fi
}

# Run main function
main "$@"
