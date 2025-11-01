#!/usr/bin/env bash
# Flux CD Bootstrap Script
# Bootstraps Flux CD with Gitea as the source repository

set -euo pipefail

# Colors for output
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly NC='\033[0m' # No Color

# Configuration
readonly FLUX_VERSION="${FLUX_VERSION:-2.2.0}"
readonly FLUX_NAMESPACE="${FLUX_NAMESPACE:-flux-system}"
readonly GITEA_NAMESPACE="${GITEA_NAMESPACE:-raibid-gitea}"
readonly GITEA_USER="${GITEA_USER:-raibid-admin}"
readonly GITEA_REPO="${GITEA_REPO:-infrastructure}"
readonly GITEA_ORG="${GITEA_ORG:-raibid}"
readonly GITEA_BRANCH="${GITEA_BRANCH:-main}"

# Script directory
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly INFRA_DIR="$(dirname "$SCRIPT_DIR")"

# Logging functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $*"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*" >&2
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."

    # Check if kubectl is installed
    if ! command -v kubectl &> /dev/null; then
        log_error "kubectl not found. Please install kubectl first."
        exit 1
    fi

    # Check if k3s is running
    if ! kubectl cluster-info &> /dev/null; then
        log_error "Cannot connect to Kubernetes cluster. Is k3s running?"
        exit 1
    fi

    # Check if Gitea is deployed
    if ! kubectl get namespace "$GITEA_NAMESPACE" &> /dev/null; then
        log_error "Gitea namespace not found. Please deploy Gitea first."
        exit 1
    fi

    # Check if Gitea is running
    if ! kubectl get pods -n "$GITEA_NAMESPACE" | grep -q "Running"; then
        log_error "Gitea pods not running. Please ensure Gitea is deployed and healthy."
        exit 1
    fi

    log_info "Prerequisites check passed"
}

# Install Flux CLI
install_flux_cli() {
    log_info "Installing Flux CLI v${FLUX_VERSION}..."

    if command -v flux &> /dev/null; then
        local current_version
        current_version=$(flux version --client | grep -oP 'flux version \K[\d.]+' || echo "unknown")
        if [[ "$current_version" == "$FLUX_VERSION" ]]; then
            log_info "Flux CLI v${FLUX_VERSION} already installed"
            return 0
        else
            log_warn "Flux CLI v${current_version} found, installing v${FLUX_VERSION}..."
        fi
    fi

    # Download and install Flux CLI
    local temp_dir
    temp_dir=$(mktemp -d)
    cd "$temp_dir"

    curl -sL "https://github.com/fluxcd/flux2/releases/download/v${FLUX_VERSION}/flux_${FLUX_VERSION}_linux_amd64.tar.gz" \
        -o flux.tar.gz

    tar -xzf flux.tar.gz

    # Install to user's local bin if no sudo, otherwise to /usr/local/bin
    if [[ -w /usr/local/bin ]]; then
        mv flux /usr/local/bin/
    else
        mkdir -p "$HOME/.local/bin"
        mv flux "$HOME/.local/bin/"
        log_warn "Flux installed to ~/.local/bin/. Ensure it's in your PATH."
    fi

    cd - > /dev/null
    rm -rf "$temp_dir"

    log_info "Flux CLI installed successfully"
}

# Get or create Gitea credentials
setup_gitea_credentials() {
    log_info "Setting up Gitea credentials..."

    # Check if GITEA_PASSWORD is provided
    if [[ -z "${GITEA_PASSWORD:-}" ]]; then
        log_error "GITEA_PASSWORD environment variable not set"
        log_error "Please set it with: export GITEA_PASSWORD='your-password'"
        exit 1
    fi

    # Create flux-system namespace
    kubectl create namespace "$FLUX_NAMESPACE" --dry-run=client -o yaml | kubectl apply -f -

    # Create or update Gitea credentials secret
    kubectl create secret generic gitea-credentials \
        --namespace="$FLUX_NAMESPACE" \
        --from-literal=username="$GITEA_USER" \
        --from-literal=password="$GITEA_PASSWORD" \
        --dry-run=client -o yaml | kubectl apply -f -

    log_info "Gitea credentials configured"
}

# Create Gitea repository structure
setup_gitea_repository() {
    log_info "Setting up Gitea repository structure..."

    # Get Gitea service URL
    local gitea_service_url="http://gitea.${GITEA_NAMESPACE}.svc.cluster.local:3000"

    # Port forward to Gitea for API access
    kubectl port-forward -n "$GITEA_NAMESPACE" svc/gitea-http 3000:3000 &
    local pf_pid=$!
    sleep 3

    # Create repository via Gitea API
    local repo_exists
    repo_exists=$(curl -s -o /dev/null -w "%{http_code}" \
        -u "${GITEA_USER}:${GITEA_PASSWORD}" \
        "http://localhost:3000/api/v1/repos/${GITEA_ORG}/${GITEA_REPO}")

    if [[ "$repo_exists" == "200" ]]; then
        log_info "Repository ${GITEA_ORG}/${GITEA_REPO} already exists"
    else
        log_info "Creating repository ${GITEA_ORG}/${GITEA_REPO}..."

        # Create organization if it doesn't exist
        curl -s -X POST \
            -u "${GITEA_USER}:${GITEA_PASSWORD}" \
            -H "Content-Type: application/json" \
            -d "{\"username\":\"${GITEA_ORG}\"}" \
            "http://localhost:3000/api/v1/orgs" || true

        # Create repository
        curl -s -X POST \
            -u "${GITEA_USER}:${GITEA_PASSWORD}" \
            -H "Content-Type: application/json" \
            -d "{
                \"name\":\"${GITEA_REPO}\",
                \"description\":\"raibid-ci infrastructure repository\",
                \"private\":false,
                \"auto_init\":true,
                \"default_branch\":\"${GITEA_BRANCH}\"
            }" \
            "http://localhost:3000/api/v1/orgs/${GITEA_ORG}/repos"

        log_info "Repository created successfully"
    fi

    # Kill port-forward
    kill $pf_pid 2>/dev/null || true

    log_info "Gitea repository configured"
}

# Install Flux components
install_flux_components() {
    log_info "Installing Flux components..."

    # Install Flux components
    flux install \
        --namespace="$FLUX_NAMESPACE" \
        --network-policy=false \
        --components=source-controller,kustomize-controller,helm-controller,notification-controller \
        --components-extra=image-reflector-controller,image-automation-controller

    log_info "Waiting for Flux controllers to be ready..."
    kubectl wait --for=condition=ready pod \
        -l app.kubernetes.io/part-of=flux \
        -n "$FLUX_NAMESPACE" \
        --timeout=300s

    log_info "Flux components installed successfully"
}

# Apply Flux manifests
apply_flux_manifests() {
    log_info "Applying Flux manifests..."

    # Apply namespace
    kubectl apply -f "$SCRIPT_DIR/namespace.yaml"

    # Apply GitRepository
    kubectl apply -f "$SCRIPT_DIR/gitrepository.yaml"

    # Apply Kustomization
    kubectl apply -f "$SCRIPT_DIR/kustomization.yaml"

    log_info "Flux manifests applied"
}

# Verify Flux installation
verify_flux() {
    log_info "Verifying Flux installation..."

    # Check Flux components
    if ! flux check; then
        log_error "Flux check failed"
        return 1
    fi

    # Check GitRepository
    log_info "Checking GitRepository status..."
    kubectl wait --for=condition=ready gitrepository/raibid-infrastructure \
        -n "$FLUX_NAMESPACE" \
        --timeout=120s || {
        log_error "GitRepository not ready"
        kubectl describe gitrepository raibid-infrastructure -n "$FLUX_NAMESPACE"
        return 1
    }

    # Check Kustomization
    log_info "Checking Kustomization status..."
    kubectl wait --for=condition=ready kustomization/raibid-ci-infrastructure \
        -n "$FLUX_NAMESPACE" \
        --timeout=120s || {
        log_warn "Kustomization not ready yet (this may be expected if manifests directory doesn't exist)"
    }

    log_info "Flux verification complete"
}

# Display summary
display_summary() {
    cat << EOF

${GREEN}Flux CD Bootstrap Complete!${NC}

Flux Namespace:     $FLUX_NAMESPACE
Git Repository:     $GITEA_ORG/$GITEA_REPO
Branch:             $GITEA_BRANCH
Sync Interval:      1 minute

Next steps:
1. Check Flux status:
   flux get all

2. View GitRepository status:
   kubectl describe gitrepository raibid-infrastructure -n $FLUX_NAMESPACE

3. View Kustomization status:
   kubectl describe kustomization raibid-ci-infrastructure -n $FLUX_NAMESPACE

4. Watch reconciliation:
   flux logs --follow

5. Force reconciliation:
   flux reconcile kustomization raibid-ci-infrastructure --with-source

Documentation: $SCRIPT_DIR/README.md

EOF
}

# Main execution
main() {
    log_info "Starting Flux CD bootstrap..."

    check_prerequisites
    install_flux_cli
    setup_gitea_credentials
    setup_gitea_repository
    install_flux_components
    apply_flux_manifests
    verify_flux
    display_summary

    log_info "Bootstrap complete!"
}

# Run main function
main "$@"
