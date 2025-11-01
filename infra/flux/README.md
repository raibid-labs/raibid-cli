# Flux Infrastructure

GitOps continuous delivery from Gitea.

## Documentation

- **[Bootstrap Guide](bootstrap.sh)** - Automated Flux installation script
- **[Validation Guide](validate.sh)** - Comprehensive validation script
- **[GitOps Workflow](GITOPS_WORKFLOW.md)** - Complete GitOps workflow guide
- **[Troubleshooting](TROUBLESHOOTING.md)** - Troubleshooting guide and solutions

## Quick Start

### Bootstrap Flux

```bash
# Set Gitea password
export GITEA_PASSWORD="your-gitea-password"

# Run bootstrap script
./bootstrap.sh

# Validate installation
./validate.sh
```

### Via raibid-cli

```bash
raibid-cli setup flux
```

## Overview

Flux provides GitOps-based continuous delivery for raibid-ci. It monitors the Gitea repository and automatically applies Kubernetes manifests when changes are detected.

### Key Features

- **Automated Sync**: Git commits automatically applied to cluster
- **Image Automation**: Automatic image updates (optional)
- **Multi-Source**: Support for Git, Helm, and OCI repositories
- **Notifications**: Alerts for reconciliation events
- **Progressive Delivery**: Integration with Flagger for canary deployments

## Manifests

- `namespace.yaml` - Flux namespace
- `gitrepository.yaml` - GitRepository source pointing to Gitea
- `kustomization.yaml` - Kustomization for infrastructure deployment
- `flux-system/` - Flux system components directory
  - `gotk-components.yaml` - Flux controllers and CRDs
  - `gotk-sync.yaml` - Flux self-management sync
  - `kustomization.yaml` - Kustomize overlay
- `bootstrap.sh` - Automated bootstrap script
- `validate.sh` - Validation and health check script

## Deployment

### Method 1: Bootstrap Script (Recommended)

The bootstrap script automates the entire Flux installation:

```bash
# Prerequisites
# - k3s cluster running
# - Gitea deployed and accessible
# - kubectl configured

# Set required environment variable
export GITEA_PASSWORD="your-gitea-password"

# Run bootstrap
cd infra/flux
./bootstrap.sh

# Output:
# - Installs Flux CLI
# - Creates Gitea repository
# - Installs Flux controllers
# - Applies GitRepository and Kustomization
# - Validates installation
```

### Method 2: raibid-cli

```bash
raibid-cli setup flux
```

### Method 3: Manual Installation

```bash
# Install Flux CLI
curl -s https://fluxcd.io/install.sh | sudo bash

# Create namespace
kubectl apply -f namespace.yaml

# Install Flux components
flux install \
  --namespace=flux-system \
  --components=source-controller,kustomize-controller,helm-controller,notification-controller \
  --components-extra=image-reflector-controller,image-automation-controller

# Create Gitea credentials
kubectl create secret generic gitea-credentials \
  --namespace=flux-system \
  --from-literal=username=raibid-admin \
  --from-literal=password=$GITEA_PASSWORD

# Apply GitRepository and Kustomization
kubectl apply -f gitrepository.yaml
kubectl apply -f kustomization.yaml
```

## Configuration

### Default Settings

- **Namespace**: `flux-system`
- **Source**: Gitea repository at `http://gitea.raibid-gitea.svc.cluster.local:3000/raibid/infrastructure`
- **Branch**: `main`
- **Sync Interval**: 1 minute
- **Prune**: Enabled (delete removed resources)
- **Retry**: Exponential backoff

### GitRepository

Defines the source Git repository:

```yaml
apiVersion: source.toolkit.fluxcd.io/v1
kind: GitRepository
metadata:
  name: raibid-infrastructure
  namespace: flux-system
spec:
  interval: 1m
  url: http://gitea.raibid-gitea.svc.cluster.local:3000/raibid/infrastructure
  ref:
    branch: main
  secretRef:
    name: gitea-credentials
```

### Kustomization

Defines how to apply manifests:

```yaml
apiVersion: kustomize.toolkit.fluxcd.io/v1
kind: Kustomization
metadata:
  name: raibid-ci-infrastructure
  namespace: flux-system
spec:
  interval: 5m
  sourceRef:
    kind: GitRepository
    name: raibid-infrastructure
  path: ./infra/manifests
  prune: true
  wait: true
  timeout: 5m
```

## Validation

### Automated Validation

```bash
# Run comprehensive validation
./validate.sh

# Checks performed:
# - Kubectl connectivity
# - Flux CLI installed
# - Flux namespace exists
# - Flux controllers running
# - GitRepository syncing
# - Kustomization applying
# - Credentials configured
# - No failed reconciliations
```

### Manual Validation

```bash
# Check Flux pods
kubectl get pods -n flux-system

# Expected pods:
# - source-controller
# - kustomize-controller
# - helm-controller
# - notification-controller
# - image-reflector-controller (optional)
# - image-automation-controller (optional)

# Check GitRepository
flux get sources git

# Check Kustomization
flux get kustomizations

# View sync status
flux check
```

## GitOps Workflow

See [GITOPS_WORKFLOW.md](GITOPS_WORKFLOW.md) for complete workflow guide.

### Quick Workflow

```bash
# 1. Make changes to infrastructure
vim infra/gitea/values.yaml

# 2. Commit and push
git add infra/gitea/values.yaml
git commit -m "Update Gitea configuration"
git push origin main

# 3. Watch Flux reconcile
flux logs --follow

# 4. Verify changes applied
kubectl get pods -n raibid-gitea
```

### Flux Detects Changes

Flux polls the GitRepository every minute:

```bash
# Watch Flux reconcile
kubectl logs -n flux-system -l app=source-controller -f
```

### Apply Changes

Flux applies changes automatically:

```bash
# View reconciliation
flux reconcile kustomization raibid-ci-infrastructure --with-source

# Check events
kubectl get events -n flux-system --sort-by='.lastTimestamp'
```

## Monitoring

### Reconciliation Status

```bash
# Check source status
flux get sources git

# Check kustomization status
flux get kustomizations

# View detailed status
kubectl describe gitrepository raibid-infrastructure -n flux-system
kubectl describe kustomization raibid-ci-infrastructure -n flux-system
```

### Notifications

Configure notifications for sync events:

```yaml
apiVersion: notification.toolkit.fluxcd.io/v1beta1
kind: Alert
metadata:
  name: raibid-ci-alerts
  namespace: flux-system
spec:
  providerRef:
    name: slack
  eventSeverity: info
  eventSources:
  - kind: GitRepository
    name: raibid-infrastructure
  - kind: Kustomization
    name: raibid-ci-infrastructure
```

## Troubleshooting

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for comprehensive troubleshooting guide.

### Quick Diagnostics

```bash
# Check system health
flux check

# View all resources
flux get all

# Check controller logs
kubectl logs -n flux-system deploy/source-controller --tail=50
kubectl logs -n flux-system deploy/kustomize-controller --tail=50

# Force reconciliation
flux reconcile source git raibid-infrastructure
flux reconcile kustomization raibid-ci-infrastructure
```

### Common Issues

#### GitRepository Not Syncing

```bash
# Check GitRepository status
kubectl describe gitrepository raibid-infrastructure -n flux-system

# Verify credentials
kubectl get secret gitea-credentials -n flux-system

# Force reconciliation
flux reconcile source git raibid-infrastructure

# Check source controller logs
kubectl logs -n flux-system -l app=source-controller --tail=50
```

#### Kustomization Failing

```bash
# Check Kustomization status
kubectl describe kustomization raibid-ci-infrastructure -n flux-system

# View error messages
flux get kustomizations

# Check kustomize controller logs
kubectl logs -n flux-system -l app=kustomize-controller --tail=50

# Validate manifests locally
kustomize build ./infra/manifests/
```

#### Authentication Issues

```bash
# Test Gitea connection
kubectl run test-gitea --rm -it --image=curlimages/curl -- \
  curl -u raibid-admin:$PASSWORD \
  http://gitea.raibid-gitea.svc.cluster.local:3000/api/v1/version

# Recreate credentials secret
kubectl create secret generic gitea-credentials \
  --namespace=flux-system \
  --from-literal=username=raibid-admin \
  --from-literal=password=$GITEA_PASSWORD \
  --dry-run=client -o yaml | kubectl apply -f -
```

## Advanced Configuration

### Multi-Environment Setup

Create separate Kustomizations for different environments:

```yaml
---
apiVersion: kustomize.toolkit.fluxcd.io/v1
kind: Kustomization
metadata:
  name: raibid-ci-dev
  namespace: flux-system
spec:
  path: ./infra/overlays/dev
  sourceRef:
    kind: GitRepository
    name: raibid-infrastructure

---
apiVersion: kustomize.toolkit.fluxcd.io/v1
kind: Kustomization
metadata:
  name: raibid-ci-prod
  namespace: flux-system
spec:
  path: ./infra/overlays/prod
  sourceRef:
    kind: GitRepository
    name: raibid-infrastructure
```

### Helm Repository

Use HelmRepository and HelmRelease for Helm charts:

```yaml
apiVersion: source.toolkit.fluxcd.io/v1beta2
kind: HelmRepository
metadata:
  name: bitnami
  namespace: flux-system
spec:
  interval: 24h
  url: https://charts.bitnami.com/bitnami

---
apiVersion: helm.toolkit.fluxcd.io/v2beta1
kind: HelmRelease
metadata:
  name: redis
  namespace: raibid-redis
spec:
  interval: 5m
  chart:
    spec:
      chart: redis
      sourceRef:
        kind: HelmRepository
        name: bitnami
      version: "18.x"
  values:
    # Values from values.yaml
```

### Image Automation

Automatically update container images:

```yaml
apiVersion: image.toolkit.fluxcd.io/v1beta1
kind: ImageRepository
metadata:
  name: raibid-ci-agent
  namespace: flux-system
spec:
  image: gitea.raibid-gitea.svc.cluster.local:3000/raibid/ci-agent
  interval: 1m

---
apiVersion: image.toolkit.fluxcd.io/v1beta1
kind: ImagePolicy
metadata:
  name: raibid-ci-agent
  namespace: flux-system
spec:
  imageRepositoryRef:
    name: raibid-ci-agent
  policy:
    semver:
      range: '>=1.0.0'

---
apiVersion: image.toolkit.fluxcd.io/v1beta1
kind: ImageUpdateAutomation
metadata:
  name: raibid-ci
  namespace: flux-system
spec:
  interval: 1m
  sourceRef:
    kind: GitRepository
    name: raibid-infrastructure
  git:
    checkout:
      ref:
        branch: main
    commit:
      author:
        email: fluxbot@raibid.local
        name: Flux Bot
      messageTemplate: 'Update image to {{range .Updated.Images}}{{println .}}{{end}}'
  update:
    path: ./infra/manifests
    strategy: Setters
```

## Uninstallation

```bash
# Via raibid-cli
raibid-cli teardown flux

# Via Flux CLI
flux uninstall --namespace=flux-system

# Manual cleanup
kubectl delete namespace flux-system
```

## References

- [Flux Documentation](https://fluxcd.io/docs/)
- [GitOps Toolkit](https://fluxcd.io/flux/components/)
- [Flux GitHub](https://github.com/fluxcd/flux2)
- [Get Started with Flux](https://fluxcd.io/flux/get-started/)
- [GitOps Workflow Guide](GITOPS_WORKFLOW.md)
- [Troubleshooting Guide](TROUBLESHOOTING.md)
