# Nushell Development Guide

This guide covers using Nushell for raibid-ci development automation, scripting, and infrastructure management.

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Project Configuration](#project-configuration)
- [Utility Modules](#utility-modules)
- [Examples](#examples)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## Installation

### Ubuntu/Debian

```bash
# Install from Homebrew (recommended)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install nushell

# Or install from cargo
cargo install nu
```

### Verify Installation

```bash
nu --version
# Should show version 0.96 or later
```

## Quick Start

### Load Project Environment

```bash
# Start Nushell
nu

# Load project configuration
source scripts/nu/env.nu
source scripts/nu/config.nu

# Check project status
project-status
```

### Run Example Scripts

```bash
# Check cluster health
nu scripts/nu/examples/check-cluster.nu

# Check Redis health
nu scripts/nu/examples/check-redis.nu

# Check Gitea health
nu scripts/nu/examples/check-gitea.nu

# Development workflow
nu scripts/nu/examples/dev-workflow.nu --status
```

## Project Configuration

### Environment Variables

The `env.nu` file sets up project-specific environment variables:

- `RAIBID_ROOT` - Project root directory
- `RAIBID_SCRIPTS` - Scripts directory
- `RAIBID_MODULES` - Nushell modules directory
- `RAIBID_DOCS` - Documentation directory
- `RAIBID_SRC` - Source code directory
- `RAIBID_TESTS` - Tests directory
- `RAIBID_K3S_CONFIG` - k3s kubeconfig path
- `RAIBID_GITEA_URL` - Gitea instance URL
- `RAIBID_REDIS_URL` - Redis instance URL

### Configuration Functions

The `config.nu` file provides helper functions:

#### Logging

```nu
log-success "Operation completed"
log-error "Something went wrong"
log-warning "This is a warning"
log-info "Informational message"
```

#### Utilities

```nu
# Check if command exists
command-exists "kubectl"  # returns true/false

# Check development prerequisites
check-dev-prerequisites

# Show project information
project-info

# Show project status
project-status
```

## Utility Modules

### kubectl Module

Interact with Kubernetes/k3s clusters.

```nu
use modules/kubectl.nu *

# Check cluster connectivity
kubectl-check-cluster

# Get cluster nodes
kubectl-get-nodes

# Get pods in namespace
kubectl-get-pods "default"
kubectl-get-pods "raibid-ci"

# Get services
kubectl-get-services "default"

# Check if namespace exists
kubectl-namespace-exists "raibid-ci"

# Create namespace
kubectl-ensure-namespace "raibid-ci"

# View pod logs
kubectl-logs "my-pod" "default" --tail 100
kubectl-logs "my-pod" "default" --follow

# Port forward
kubectl-port-forward "service/gitea" 3000 3000 "raibid-ci"

# Apply manifest
kubectl-apply "manifests/deployment.yaml" "raibid-ci"

# Wait for deployment
kubectl-wait-deployment "gitea" "raibid-ci"

# Execute command in pod
kubectl-exec "my-pod" ["ls", "-la"] "default"

# Get resource usage
kubectl-top-pods "raibid-ci"
```

### redis Module

Interact with Redis and Redis Streams.

```nu
use modules/redis.nu *

# Check connection
redis-check-connection "redis://localhost:6379"

# Ping Redis
redis-ping

# Get Redis info
redis-info
redis-info "memory"
redis-info "server"

# Key operations
redis-keys "*"
redis-get "mykey"
redis-set "mykey" "myvalue"
redis-set "session" "data" --expire 3600
redis-del "mykey"

# Stream operations
redis-stream-add "raibid:jobs" {job_id: "123", type: "build"}
redis-stream-read "raibid:jobs" --count 10
redis-stream-len "raibid:jobs"
redis-stream-info "raibid:jobs"

# Consumer groups
redis-stream-create-group "raibid:jobs" "workers" --from-start
redis-stream-read-group "raibid:jobs" "workers" "worker-1" --count 5
redis-stream-ack "raibid:jobs" "workers" "1234567890-0"
redis-stream-pending "raibid:jobs" "workers"

# Utilities
redis-memory
redis-stats
redis-monitor  # Real-time command monitoring
```

### gitea Module

Interact with Gitea API.

```nu
use modules/gitea.nu *

# Set GITEA_TOKEN environment variable for authenticated requests
$env.GITEA_TOKEN = "your-token-here"

# Check connection
gitea-check-connection
gitea-check-connection "http://localhost:3000"

# Get version
gitea-version

# User operations
gitea-user

# Repository operations
gitea-list-repos --limit 20
gitea-get-repo "owner" "repo"
gitea-create-repo "my-new-repo" --description "My repo" --private
gitea-delete-repo "owner" "repo" --confirm

# Branch and tag operations
gitea-list-branches "owner" "repo"
gitea-list-tags "owner" "repo"

# Webhook operations
gitea-create-webhook "owner" "repo" "http://webhook.url" --events ["push", "pull_request"]
gitea-list-webhooks "owner" "repo"

# Organization operations
gitea-create-org "my-org" --description "My organization"
gitea-list-orgs

# Release operations
gitea-create-release "owner" "repo" "v1.0.0" --name "Version 1.0" --body "Release notes"
gitea-list-releases "owner" "repo"

# Search
gitea-search-repos "raibid" --limit 10

# Mirror repository
gitea-mirror-repo "https://github.com/user/repo" "mirrored-repo" --private
```

## Examples

### Health Check Script

Create a comprehensive health check:

```nu
#!/usr/bin/env nu
source scripts/nu/env.nu
source scripts/nu/config.nu

use modules/kubectl.nu *
use modules/redis.nu *
use modules/gitea.nu *

def main [] {
    log-info "Running health checks..."

    # Check k3s
    if (kubectl-check-cluster) {
        log-success "k3s is healthy"
        kubectl-get-nodes | table
    } else {
        log-error "k3s is not available"
    }

    # Check Redis
    if (redis-check-connection) {
        log-success "Redis is healthy"
        print $"Memory: (redis-memory)"
    } else {
        log-error "Redis is not available"
    }

    # Check Gitea
    if (gitea-check-connection) {
        log-success "Gitea is healthy"
        let version = (gitea-version)
        print $"Version: ($version.version)"
    } else {
        log-error "Gitea is not available"
    }
}

main
```

### CI Job Publisher

Publish jobs to Redis Streams:

```nu
#!/usr/bin/env nu
source scripts/nu/env.nu

use modules/redis.nu *

def publish-job [job_type: string, repo: string, branch: string] {
    let job_data = {
        job_id: (date now | format date "%Y%m%d%H%M%S")
        type: $job_type
        repo: $repo
        branch: $branch
        timestamp: (date now | format date "%Y-%m-%d %H:%M:%S")
    }

    redis-stream-add $env.RAIBID_REDIS_STREAM $job_data
    log-success $"Job published: ($job_type) for ($repo):($branch)"
}

publish-job "build" "raibid-labs/raibid-ci" "main"
```

### Repository Mirror Automation

Mirror GitHub repositories to Gitea:

```nu
#!/usr/bin/env nu
source scripts/nu/env.nu

use modules/gitea.nu *

def mirror-repos [repos: list<string>] {
    for repo in $repos {
        let parts = ($repo | split row "/")
        let owner = ($parts | first)
        let name = ($parts | last)

        log-info $"Mirroring ($repo)..."

        gitea-mirror-repo $"https://github.com/($repo)" $name --private

        log-success $"Mirrored: ($repo)"
    }
}

mirror-repos [
    "raibid-labs/raibid-ci"
    "rust-lang/cargo"
]
```

### Deployment Script

Deploy to k3s cluster:

```nu
#!/usr/bin/env nu
source scripts/nu/env.nu

use modules/kubectl.nu *

def deploy [namespace: string, manifest: string] {
    log-info $"Deploying to ($namespace)..."

    # Ensure namespace exists
    kubectl-ensure-namespace $namespace

    # Apply manifest
    kubectl-apply $manifest $namespace

    # Wait for deployment
    let deployment_name = "raibid-api"  # Extract from manifest
    kubectl-wait-deployment $deployment_name $namespace

    log-success "Deployment complete!"

    # Show pods
    kubectl-get-pods $namespace | table
}

deploy "raibid-ci" "manifests/api-deployment.yaml"
```

## Best Practices

### Module Usage

Always use modules for reusable functionality:

```nu
# Good
use modules/kubectl.nu *
kubectl-get-pods "default"

# Avoid
kubectl get pods -n default  # Raw commands
```

### Error Handling

Use try-catch for operations that might fail:

```nu
try {
    kubectl-get-pods "raibid-ci"
} catch {
    log-warning "Namespace doesn't exist yet"
    kubectl-ensure-namespace "raibid-ci"
}
```

### Configuration Management

Always source environment and config files:

```nu
#!/usr/bin/env nu
source scripts/nu/env.nu
source scripts/nu/config.nu

# Now you can use project functions
project-status
```

### Logging

Use structured logging functions:

```nu
# Good
log-info "Starting deployment"
log-success "Deployment complete"
log-error "Deployment failed"

# Avoid
print "Starting deployment"  # No visual distinction
```

### Pipelines

Use Nushell pipelines for data transformation:

```nu
# Get pod names in specific phase
kubectl-get-pods "default" |
    where phase == "Running" |
    get name

# Count pods by phase
kubectl-get-pods "default" |
    group-by phase |
    transpose key count
```

## Troubleshooting

### Nushell Version

Ensure you're using Nushell 0.96 or later:

```bash
nu --version
```

### Module Path Issues

If modules aren't loading, check `NU_LIB_DIRS`:

```nu
$env.NU_LIB_DIRS
# Should include /path/to/raibid-ci/scripts/nu/modules
```

Fix by sourcing config:

```nu
source scripts/nu/config.nu
```

### Command Not Found

Ensure tools are installed:

```nu
command-exists "kubectl"  # Check kubectl
command-exists "redis-cli"  # Check redis-cli
command-exists "curl"  # Check curl
```

### Environment Variables

Check environment setup:

```nu
$env | where name =~ "RAIBID"
```

If variables are missing:

```nu
source scripts/nu/env.nu
```

### API Token Issues

For Gitea operations requiring authentication:

```nu
# Set token
$env.GITEA_TOKEN = "your-token-here"

# Verify
"GITEA_TOKEN" in $env
```

### Redis Connection

If Redis connection fails:

```bash
# Check if Redis is running
redis-cli ping

# Check connection string
echo $RAIBID_REDIS_URL
```

### k3s Connection

If kubectl commands fail:

```bash
# Check kubeconfig
echo $RAIBID_KUBECONFIG

# Verify k3s is running
sudo systemctl status k3s
```

## Additional Resources

- [Nushell Official Documentation](https://www.nushell.sh/)
- [Nushell Book](https://www.nushell.sh/book/)
- [kubectl Reference](https://kubernetes.io/docs/reference/kubectl/)
- [Redis Commands](https://redis.io/commands)
- [Gitea API](https://docs.gitea.io/en-us/api-usage/)

## Contributing

When adding new modules or scripts:

1. Follow existing patterns in `modules/`
2. Add examples to `examples/`
3. Update this guide
4. Test with `nu --commands "source your-script.nu"`
5. Ensure CI validation passes
