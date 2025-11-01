# Flux CD Troubleshooting Guide

Comprehensive guide for diagnosing and resolving Flux CD issues.

## Table of Contents

- [Quick Diagnostics](#quick-diagnostics)
- [Common Issues](#common-issues)
- [GitRepository Issues](#gitrepository-issues)
- [Kustomization Issues](#kustomization-issues)
- [HelmRelease Issues](#helmrelease-issues)
- [Authentication Issues](#authentication-issues)
- [Performance Issues](#performance-issues)
- [Recovery Procedures](#recovery-procedures)

## Quick Diagnostics

### Basic Health Check

```bash
# Check Flux system status
flux check

# View all Flux resources
flux get all

# Check controller pods
kubectl get pods -n flux-system

# View recent events
kubectl get events -n flux-system --sort-by='.lastTimestamp' | tail -20
```

### Component Status

```bash
# Check GitRepository sources
flux get sources git

# Check Kustomizations
flux get kustomizations

# Check HelmRepositories
flux get sources helm

# Check HelmReleases
flux get helmreleases
```

### Logs

```bash
# Source controller logs (Git/Helm repositories)
kubectl logs -n flux-system deploy/source-controller --tail=100 -f

# Kustomize controller logs (Kustomizations)
kubectl logs -n flux-system deploy/kustomize-controller --tail=100 -f

# Helm controller logs (HelmReleases)
kubectl logs -n flux-system deploy/helm-controller --tail=100 -f

# Notification controller logs (alerts/webhooks)
kubectl logs -n flux-system deploy/notification-controller --tail=100 -f
```

## Common Issues

### Issue: Flux Controllers Not Running

**Symptoms:**
- `flux check` fails
- No pods in flux-system namespace
- Controllers in CrashLoopBackOff

**Diagnosis:**

```bash
kubectl get pods -n flux-system
kubectl describe pod -n flux-system <pod-name>
kubectl logs -n flux-system <pod-name>
```

**Solutions:**

1. **Resource constraints:**
   ```bash
   # Check node resources
   kubectl top nodes
   kubectl describe node

   # Increase resource limits if needed
   flux install --export | kubectl apply -f -
   ```

2. **Network policies:**
   ```bash
   # Disable network policies
   flux install --network-policy=false --export | kubectl apply -f -
   ```

3. **Reinstall Flux:**
   ```bash
   flux uninstall --silent
   flux install --namespace=flux-system
   ```

### Issue: GitRepository Not Syncing

**Symptoms:**
- GitRepository shows "False" ready status
- No artifact available
- Reconciliation fails

**Diagnosis:**

```bash
# Check GitRepository status
kubectl describe gitrepository <name> -n flux-system

# View source controller logs
kubectl logs -n flux-system deploy/source-controller --tail=100

# Test Git connectivity manually
kubectl run test-git --rm -it --image=alpine/git -- \
  git ls-remote http://gitea.raibid-gitea.svc.cluster.local:3000/raibid/infrastructure
```

**Solutions:**

See [GitRepository Issues](#gitrepository-issues) section below.

### Issue: Kustomization Failing

**Symptoms:**
- Kustomization shows "False" ready status
- Resources not applied
- Reconciliation errors

**Diagnosis:**

```bash
# Check Kustomization status
kubectl describe kustomization <name> -n flux-system

# View kustomize controller logs
kubectl logs -n flux-system deploy/kustomize-controller --tail=100

# Validate manifests locally
kustomize build <path>
```

**Solutions:**

See [Kustomization Issues](#kustomization-issues) section below.

## GitRepository Issues

### Authentication Failure

**Error:**
```
authentication required
```

**Solution:**

```bash
# Verify secret exists
kubectl get secret gitea-credentials -n flux-system

# Verify secret contains correct credentials
kubectl get secret gitea-credentials -n flux-system -o yaml

# Update credentials
kubectl create secret generic gitea-credentials \
  --namespace=flux-system \
  --from-literal=username=raibid-admin \
  --from-literal=password=$GITEA_PASSWORD \
  --dry-run=client -o yaml | kubectl apply -f -

# Force reconciliation
flux reconcile source git raibid-infrastructure
```

### Repository Not Found

**Error:**
```
repository not found
```

**Solution:**

```bash
# Verify repository exists in Gitea
curl -u raibid-admin:$GITEA_PASSWORD \
  http://localhost:30080/api/v1/repos/raibid/infrastructure

# Create repository if missing
curl -X POST -u raibid-admin:$GITEA_PASSWORD \
  -H "Content-Type: application/json" \
  -d '{"name":"infrastructure","auto_init":true}' \
  http://localhost:30080/api/v1/orgs/raibid/repos

# Update GitRepository URL
kubectl edit gitrepository raibid-infrastructure -n flux-system
```

### Branch Not Found

**Error:**
```
couldn't find remote ref "refs/heads/main"
```

**Solution:**

```bash
# List available branches
curl -u raibid-admin:$GITEA_PASSWORD \
  http://localhost:30080/api/v1/repos/raibid/infrastructure/branches

# Update branch in GitRepository
kubectl patch gitrepository raibid-infrastructure -n flux-system \
  --type=merge -p '{"spec":{"ref":{"branch":"master"}}}'
```

### Network Connectivity Issues

**Error:**
```
dial tcp: lookup gitea.raibid-gitea.svc.cluster.local: no such host
```

**Solution:**

```bash
# Verify Gitea service exists
kubectl get svc -n raibid-gitea gitea-http

# Test DNS resolution
kubectl run test-dns --rm -it --image=busybox -- \
  nslookup gitea.raibid-gitea.svc.cluster.local

# Test HTTP connectivity
kubectl run test-http --rm -it --image=curlimages/curl -- \
  curl -v http://gitea.raibid-gitea.svc.cluster.local:3000

# If using NodePort, switch to ClusterIP URL
kubectl edit gitrepository raibid-infrastructure -n flux-system
```

### Timeout Issues

**Error:**
```
context deadline exceeded
```

**Solution:**

```bash
# Increase timeout in GitRepository
kubectl patch gitrepository raibid-infrastructure -n flux-system \
  --type=merge -p '{"spec":{"timeout":"120s"}}'

# Check Gitea pod health
kubectl get pods -n raibid-gitea
kubectl logs -n raibid-gitea <gitea-pod>

# Restart Gitea if needed
kubectl rollout restart deployment/gitea -n raibid-gitea
```

## Kustomization Issues

### Path Not Found

**Error:**
```
kustomization path not found: ./infra/manifests
```

**Solution:**

```bash
# Verify path exists in repository
git ls-tree HEAD --name-only

# Create missing directory
mkdir -p infra/manifests
echo "apiVersion: v1\nkind: Namespace\nmetadata:\n  name: placeholder" > infra/manifests/namespace.yaml
git add infra/manifests/
git commit -m "Add manifests directory"
git push

# Wait for GitRepository to sync
flux reconcile source git raibid-infrastructure

# Reconcile Kustomization
flux reconcile kustomization raibid-ci-infrastructure
```

### Invalid Manifests

**Error:**
```
invalid YAML: error unmarshaling JSON
```

**Solution:**

```bash
# Validate manifests locally
find infra/manifests -name "*.yaml" -exec kubectl apply --dry-run=client -f {} \;

# Use kustomize to build and validate
kustomize build infra/manifests | kubectl apply --dry-run=client -f -

# Fix YAML syntax errors
yamllint infra/manifests/

# Commit fixes
git add infra/manifests/
git commit -m "Fix YAML syntax"
git push
```

### Resource Conflicts

**Error:**
```
field is immutable
```

**Solution:**

```bash
# Delete conflicting resource
kubectl delete <resource-type> <resource-name> -n <namespace>

# Or use force apply
kubectl patch kustomization raibid-ci-infrastructure -n flux-system \
  --type=merge -p '{"spec":{"force":true}}'

# Wait for reconciliation
flux reconcile kustomization raibid-ci-infrastructure
```

### Health Check Failures

**Error:**
```
health check failed: deployment not ready
```

**Solution:**

```bash
# Check deployment status
kubectl get deployments -n <namespace>
kubectl describe deployment <name> -n <namespace>

# Increase timeout
kubectl patch kustomization raibid-ci-infrastructure -n flux-system \
  --type=merge -p '{"spec":{"timeout":"10m"}}'

# Remove health checks temporarily
kubectl patch kustomization raibid-ci-infrastructure -n flux-system \
  --type=merge -p '{"spec":{"healthChecks":null}}'
```

### Prune Issues

**Error:**
```
prune failed: resource not found
```

**Solution:**

```bash
# Disable pruning temporarily
kubectl patch kustomization raibid-ci-infrastructure -n flux-system \
  --type=merge -p '{"spec":{"prune":false}}'

# Manually delete orphaned resources
kubectl delete <resource-type> <resource-name> -n <namespace>

# Re-enable pruning
kubectl patch kustomization raibid-ci-infrastructure -n flux-system \
  --type=merge -p '{"spec":{"prune":true}}'
```

## HelmRelease Issues

### Chart Not Found

**Error:**
```
chart not found
```

**Solution:**

```bash
# Update HelmRepository
flux reconcile source helm <repository-name>

# Verify chart exists
helm search repo <repository-name>/<chart-name>

# Check HelmRepository status
kubectl describe helmrepository <name> -n flux-system
```

### Values Override Issues

**Error:**
```
values don't meet the specifications of the schema
```

**Solution:**

```bash
# Test values locally
helm template <chart-name> <repository>/<chart> -f values.yaml

# Validate against schema
helm show values <repository>/<chart>

# Fix values and update
git add values.yaml
git commit -m "Fix Helm values"
git push
```

## Authentication Issues

### Invalid Credentials

**Symptoms:**
- 401 Unauthorized errors
- Authentication failed messages

**Solution:**

```bash
# Test credentials manually
curl -u raibid-admin:$GITEA_PASSWORD \
  http://localhost:30080/api/v1/user

# Reset Gitea password
kubectl exec -it -n raibid-gitea gitea-0 -- \
  gitea admin user change-password --username raibid-admin --password $NEW_PASSWORD

# Update secret
kubectl create secret generic gitea-credentials \
  --namespace=flux-system \
  --from-literal=username=raibid-admin \
  --from-literal=password=$NEW_PASSWORD \
  --dry-run=client -o yaml | kubectl apply -f -

# Force reconciliation
flux reconcile source git raibid-infrastructure
```

### SSH Key Issues

**Symptoms:**
- SSH authentication failures
- Host key verification failed

**Solution:**

```bash
# Generate SSH key pair
ssh-keygen -t ed25519 -C "flux@raibid-ci" -f flux-ssh-key

# Add public key to Gitea
curl -X POST -u raibid-admin:$GITEA_PASSWORD \
  -H "Content-Type: application/json" \
  -d "{\"title\":\"Flux CD\",\"key\":\"$(cat flux-ssh-key.pub)\"}" \
  http://localhost:30080/api/v1/user/keys

# Create SSH secret
kubectl create secret generic gitea-ssh \
  --namespace=flux-system \
  --from-file=identity=flux-ssh-key \
  --from-literal=known_hosts="$(ssh-keyscan -H gitea.raibid-gitea.svc.cluster.local)"

# Update GitRepository to use SSH
kubectl patch gitrepository raibid-infrastructure -n flux-system \
  --type=merge -p '{
    "spec":{
      "url":"ssh://git@gitea.raibid-gitea.svc.cluster.local:22/raibid/infrastructure.git",
      "secretRef":{"name":"gitea-ssh"}
    }
  }'
```

## Performance Issues

### Slow Reconciliation

**Symptoms:**
- Long sync times
- High CPU/memory usage
- Timeouts

**Solutions:**

```bash
# Increase sync interval
kubectl patch gitrepository raibid-infrastructure -n flux-system \
  --type=merge -p '{"spec":{"interval":"5m"}}'

kubectl patch kustomization raibid-ci-infrastructure -n flux-system \
  --type=merge -p '{"spec":{"interval":"10m"}}'

# Increase resource limits
kubectl set resources deployment source-controller -n flux-system \
  --limits=cpu=1000m,memory=1Gi

# Split large Kustomizations
# Create separate Kustomizations for different components
```

### High API Server Load

**Symptoms:**
- API server errors
- Rate limiting

**Solutions:**

```bash
# Reduce reconciliation frequency
# Set longer intervals on resources

# Use server-side apply
kubectl patch kustomization raibid-ci-infrastructure -n flux-system \
  --type=merge -p '{"spec":{"force":false}}'

# Limit concurrent reconciliations
kubectl set env deployment kustomize-controller -n flux-system \
  --containers=manager CONCURRENT_RECONCILES=2
```

## Recovery Procedures

### Complete Flux Reset

```bash
# Suspend all reconciliations
flux suspend kustomization --all

# Uninstall Flux
flux uninstall --silent

# Clean up namespace
kubectl delete namespace flux-system --wait=false
kubectl patch namespace flux-system -p '{"metadata":{"finalizers":null}}'

# Reinstall Flux
flux install --namespace=flux-system

# Resume reconciliations
flux resume kustomization --all
```

### Restore from Backup

```bash
# Export current state
flux export source git --all > backup-git.yaml
flux export kustomization --all > backup-kustomizations.yaml

# Restore
kubectl apply -f backup-git.yaml
kubectl apply -f backup-kustomizations.yaml
```

### Emergency Manual Intervention

```bash
# Suspend Flux to prevent conflicts
flux suspend kustomization raibid-ci-infrastructure

# Make manual changes
kubectl apply -f emergency-fix.yaml

# Resume when ready
flux resume kustomization raibid-ci-infrastructure
```

## Getting Help

### Collect Diagnostic Information

```bash
#!/bin/bash
# flux-diagnostics.sh - Collect diagnostic information

OUTPUT_DIR="flux-diagnostics-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$OUTPUT_DIR"

# Flux status
flux check > "$OUTPUT_DIR/flux-check.txt"
flux get all > "$OUTPUT_DIR/flux-get-all.txt"

# Kubernetes resources
kubectl get all -n flux-system > "$OUTPUT_DIR/flux-resources.txt"
kubectl describe pods -n flux-system > "$OUTPUT_DIR/flux-pods.txt"

# Logs
for controller in source-controller kustomize-controller helm-controller notification-controller; do
  kubectl logs -n flux-system deploy/$controller --tail=500 > "$OUTPUT_DIR/$controller.log"
done

# Events
kubectl get events -n flux-system --sort-by='.lastTimestamp' > "$OUTPUT_DIR/events.txt"

# GitRepository details
kubectl get gitrepository -A -o yaml > "$OUTPUT_DIR/gitrepositories.yaml"

# Kustomization details
kubectl get kustomization -A -o yaml > "$OUTPUT_DIR/kustomizations.yaml"

echo "Diagnostics collected in $OUTPUT_DIR/"
tar -czf "$OUTPUT_DIR.tar.gz" "$OUTPUT_DIR"
echo "Archive: $OUTPUT_DIR.tar.gz"
```

### Useful Resources

- [Flux Documentation](https://fluxcd.io/docs/)
- [Flux GitHub Discussions](https://github.com/fluxcd/flux2/discussions)
- [Flux Slack](https://cloud-native.slack.com/messages/flux)
- [Flux FAQ](https://fluxcd.io/docs/faq/)
