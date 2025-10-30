# raibid-cli User Guide

> **Comprehensive guide for using raibid-cli to manage your DGX Spark CI infrastructure**

This guide provides detailed examples and workflows for using raibid-cli effectively.

## Table of Contents

- [Getting Started](#getting-started)
- [Configuration](#configuration)
- [Infrastructure Management](#infrastructure-management)
- [TUI Dashboard](#tui-dashboard)
- [Job Management](#job-management)
- [Agent Management](#agent-management)
- [Repository Mirroring](#repository-mirroring)
- [Common Workflows](#common-workflows)
- [Troubleshooting](#troubleshooting)
- [Tips and Best Practices](#tips-and-best-practices)

## Getting Started

### Installation

#### Quick Install (from source)

```bash
# Clone and build
git clone https://github.com/raibid-labs/raibid-cli.git
cd raibid-cli
cargo build --release

# Make available in PATH
sudo cp target/release/raibid-cli /usr/local/bin/
```

#### ARM64 Build for DGX Spark

```bash
# Add ARM64 target
rustup target add aarch64-unknown-linux-gnu

# Build for ARM64
cargo build --release --target aarch64-unknown-linux-gnu

# Copy binary to DGX Spark
scp target/aarch64-unknown-linux-gnu/release/raibid-cli dgx-spark:/usr/local/bin/
```

### First-Time Setup

1. **Initialize Configuration**

```bash
# Create default configuration file
raibid-cli config init

# This creates ~/.config/raibid/config.yaml with sensible defaults
```

2. **Verify Installation**

```bash
# Check version
raibid-cli --version

# View help
raibid-cli --help

# Show current configuration
raibid-cli config show
```

3. **Set Up Infrastructure**

```bash
# Bootstrap all infrastructure components
raibid-cli setup all

# Or set up components individually
raibid-cli setup k3s      # First
raibid-cli setup gitea    # Requires k3s
raibid-cli setup redis    # Requires k3s
raibid-cli setup keda     # Requires k3s
raibid-cli setup flux     # Requires k3s and gitea
```

## Configuration

### Configuration File Locations

raibid-cli loads configuration from multiple locations (in priority order):

1. Environment variables (`RAIBID_*`)
2. Local file: `./raibid.yaml`
3. User file: `~/.config/raibid/config.yaml`
4. System file: `/etc/raibid/config.yaml`

### Creating Configuration Files

#### Full Example Configuration

```bash
# Generate full configuration with all options
raibid-cli config init

# Save to custom location
raibid-cli config init --output ~/my-config.yaml
```

This creates:

```yaml
# Cluster configuration
cluster:
  name: "dgx-spark-ci"
  namespace: "raibid-ci"
  kubeconfig: "~/.kube/config"

# API server configuration
api:
  host: "localhost"
  port: 8080
  timeout_seconds: 30

# Agent configuration
agents:
  min_count: 0
  max_count: 8
  idle_timeout_minutes: 5
  image: "raibid/rust-builder:latest"

# Gitea configuration
gitea:
  url: "http://gitea.raibid-ci.svc.cluster.local:3000"
  admin_user: "admin"
  admin_password: ""  # Set via RAIBID_GITEA_ADMIN_PASSWORD

# Redis configuration
redis:
  url: "redis://redis.raibid-ci.svc.cluster.local:6379"
  stream_name: "ci-jobs"
  consumer_group: "ci-workers"

# TUI configuration
tui:
  refresh_interval_ms: 1000
  panel_proportions: [70, 15, 15]
```

#### Minimal Configuration

```bash
# Generate minimal configuration
raibid-cli config init --minimal
```

This creates a smaller config with only essential settings.

### Environment Variable Overrides

Override any configuration value using environment variables:

```bash
# API settings
export RAIBID_API_HOST="api.example.com"
export RAIBID_API_PORT="9090"

# Agent settings
export RAIBID_AGENTS_MAX_COUNT="16"
export RAIBID_AGENTS_IDLE_TIMEOUT_MINUTES="10"

# Gitea settings
export RAIBID_GITEA_ADMIN_PASSWORD="secure-password"

# Run with overrides
raibid-cli config show
```

### Validating Configuration

```bash
# Validate merged configuration
raibid-cli config validate

# Validate specific file
raibid-cli config validate ~/my-config.yaml

# View merged configuration
raibid-cli config show

# View in different formats
raibid-cli config show --format json
raibid-cli config show --format toml
```

### Finding Configuration Files

```bash
# Show which config file is being used
raibid-cli config path
```

## Infrastructure Management

### Setup Commands

#### Complete Setup

```bash
# Set up all components in the correct order
raibid-cli setup all
```

This performs:
1. k3s cluster bootstrap
2. Gitea deployment
3. Redis Streams setup
4. KEDA installation
5. Flux GitOps bootstrap

#### Individual Component Setup

```bash
# k3s cluster (must be first)
raibid-cli setup k3s

# Gitea with OCI registry
raibid-cli setup gitea

# Redis Streams for job queue
raibid-cli setup redis

# KEDA autoscaler
raibid-cli setup keda

# Flux GitOps
raibid-cli setup flux
```

### Teardown Commands

```bash
# Remove all components
raibid-cli teardown all

# Remove individual components
raibid-cli teardown flux     # Remove Flux (do first)
raibid-cli teardown keda     # Remove KEDA
raibid-cli teardown redis    # Remove Redis
raibid-cli teardown gitea    # Remove Gitea
raibid-cli teardown k3s      # Remove k3s (do last)
```

### Status Commands

```bash
# Check all components
raibid-cli status

# Check specific component
raibid-cli status k3s
raibid-cli status gitea
raibid-cli status redis
raibid-cli status keda
raibid-cli status flux
```

## TUI Dashboard

### Launching the TUI

```bash
# Launch interactive dashboard
raibid-cli tui

# Launch with debug logging
RUST_LOG=debug raibid-cli tui
```

### TUI Navigation

#### Tab Navigation

- **Tab** - Next tab
- **Shift+Tab** - Previous tab
- **1** - Jump to Jobs tab
- **2** - Jump to Agents tab
- **3** - Jump to Config tab
- **4** - Jump to Logs tab

#### List Navigation

- **↑/↓** or **k/j** - Move up/down in lists
- **Page Up/Down** - Scroll page
- **Home/End** - Jump to start/end
- **Enter** - View details of selected item

#### Actions

- **r** - Refresh data
- **f** - Open filter menu
- **/** - Enter search mode
- **?** - Show help screen
- **q** or **Ctrl+C** - Quit

### Jobs Tab

The Jobs tab shows all CI jobs with their status.

**Columns:**
- ID - Job identifier
- Repository - Source repository
- Branch - Git branch
- Status - Running, Success, Failed, Pending
- Started - Relative time
- Duration - Elapsed time

**Filtering:**
1. Press **f** to open filter menu
2. Use **↑/↓** to select filter
3. Press **Enter** to apply
4. Press **Esc** to cancel

**Searching:**
1. Press **/** to enter search mode
2. Type search query
3. Press **Enter** to search
4. Press **Esc** to clear search

### Agents Tab

The Agents tab displays build agent status and resource usage.

**Columns:**
- ID - Agent identifier
- Status - Running, Idle, Offline
- Current Job - Active job ID (if any)
- CPU % - CPU utilization
- Memory % - Memory usage
- Uptime - Hours online

**Actions:**
- **Enter** - View detailed agent information

### Config Tab

Displays the current merged configuration in YAML format.

### Logs Tab

Shows real-time log output from the system.

## Job Management

### Listing Jobs

```bash
# List all jobs
raibid-cli job list

# Filter by status
raibid-cli job list --status running
raibid-cli job list --status success
raibid-cli job list --status failed
raibid-cli job list --status pending

# Filter by repository
raibid-cli job list --repo raibid/core
raibid-cli job list --repo raibid/cli

# Limit results
raibid-cli job list --limit 10

# JSON output
raibid-cli job list --output json

# Combine filters
raibid-cli job list --status failed --repo raibid/core --limit 5
```

### Viewing Job Details

```bash
# Show job details
raibid-cli job show a1b2c3

# JSON output
raibid-cli job show a1b2c3 --output json
```

### Canceling Jobs

```bash
# Cancel with confirmation
raibid-cli job cancel a1b2c3

# Cancel without confirmation
raibid-cli job cancel a1b2c3 --force
```

### Retrying Failed Jobs

```bash
# Retry a failed job
raibid-cli job retry g7h8i9
```

## Agent Management

### Listing Agents

```bash
# List all agents
raibid-cli agent list

# Filter by status
raibid-cli agent list --status running
raibid-cli agent list --status idle
raibid-cli agent list --status offline

# JSON output
raibid-cli agent list --output json
```

### Viewing Agent Details

```bash
# Show agent details
raibid-cli agent show rust-builder-1

# JSON output
raibid-cli agent show rust-builder-1 --output json
```

### Restarting Agents

```bash
# Restart with confirmation
raibid-cli agent restart rust-builder-1

# Restart without confirmation
raibid-cli agent restart rust-builder-1 --force
```

### Scaling Agents

```bash
# Scale to specific count
raibid-cli agent scale --count 5

# Scale with min/max constraints
raibid-cli agent scale --count 3 --min 2 --max 8

# Scale to zero (all idle)
raibid-cli agent scale --count 0
```

## Repository Mirroring

### Adding Mirrors

```bash
# Add repository mirror
raibid-cli mirror add github.com/raibid/core

# Add with custom name
raibid-cli mirror add github.com/raibid/cli --name raibid-cli

# Add with sync interval (minutes)
raibid-cli mirror add github.com/raibid/api --sync-interval 30
```

### Listing Mirrors

```bash
# List all mirrors
raibid-cli mirror list

# JSON output
raibid-cli mirror list --output json
```

### Syncing Mirrors

```bash
# Sync repository
raibid-cli mirror sync github.com/raibid/core

# Force sync (ignore sync interval)
raibid-cli mirror sync github.com/raibid/core --force
```

### Removing Mirrors

```bash
# Remove with confirmation
raibid-cli mirror remove github.com/raibid/docs

# Remove without confirmation
raibid-cli mirror remove github.com/raibid/docs --force
```

## Common Workflows

### Initial System Setup

```bash
# 1. Install raibid-cli
cargo build --release
sudo cp target/release/raibid-cli /usr/local/bin/

# 2. Initialize configuration
raibid-cli config init

# 3. Customize configuration (optional)
$EDITOR ~/.config/raibid/config.yaml

# 4. Validate configuration
raibid-cli config validate

# 5. Set up infrastructure
raibid-cli setup all

# 6. Verify status
raibid-cli status

# 7. Launch TUI
raibid-cli tui
```

### Daily Monitoring Workflow

```bash
# Launch TUI dashboard
raibid-cli tui

# Alternative: Use CLI commands
raibid-cli status                    # Check system status
raibid-cli job list --status running # Check active jobs
raibid-cli agent list                # Check agent status
```

### Investigating Failed Jobs

```bash
# 1. List failed jobs
raibid-cli job list --status failed

# 2. View job details
raibid-cli job show <job-id>

# 3. Check agent status (if agent-related)
raibid-cli agent show <agent-id>

# 4. Retry the job
raibid-cli job retry <job-id>
```

### Scaling for High Load

```bash
# 1. Check current agent count
raibid-cli agent list

# 2. Check pending jobs
raibid-cli job list --status pending

# 3. Scale up agents
raibid-cli agent scale --count 8

# 4. Monitor in TUI
raibid-cli tui
```

### Scaling Down During Idle

```bash
# 1. Check for running jobs
raibid-cli job list --status running

# 2. Check agent activity
raibid-cli agent list

# 3. Scale down (if no active jobs)
raibid-cli agent scale --count 2

# Or scale to zero
raibid-cli agent scale --count 0
```

### Maintaining Mirrors

```bash
# 1. List all mirrors
raibid-cli mirror list

# 2. Check sync status
# Look for mirrors with old "last sync" times

# 3. Force sync outdated mirrors
raibid-cli mirror sync github.com/user/repo --force

# 4. Add new mirrors as needed
raibid-cli mirror add github.com/new/repo

# 5. Remove unused mirrors
raibid-cli mirror remove github.com/old/repo --force
```

### System Cleanup

```bash
# 1. Cancel any running jobs
raibid-cli job list --status running
raibid-cli job cancel <job-id> --force

# 2. Scale down agents
raibid-cli agent scale --count 0

# 3. Tear down infrastructure (if needed)
raibid-cli teardown all

# 4. Verify cleanup
raibid-cli status
```

## Troubleshooting

### Configuration Issues

**Problem: Configuration not loading**

```bash
# Check which config file is being used
raibid-cli config path

# Validate configuration
raibid-cli config validate

# View merged configuration
raibid-cli config show

# Check for syntax errors
raibid-cli config validate ~/.config/raibid/config.yaml
```

**Problem: Environment variables not working**

```bash
# Verify environment variables are set
env | grep RAIBID_

# Check merged config shows overrides
raibid-cli config show
```

### TUI Issues

**Problem: TUI not rendering correctly**

```bash
# Check terminal type
echo $TERM

# Try different terminal emulator
# Recommended: Alacritty, WezTerm, iTerm2, Windows Terminal

# Check terminal size
tput cols
tput lines
```

**Problem: TUI crashes or freezes**

```bash
# Run with debug logging
RUST_LOG=debug raibid-cli tui 2> tui-debug.log

# Check logs
tail -f tui-debug.log
```

### Build Issues

**Problem: Compilation errors**

```bash
# Update Rust toolchain
rustup update stable

# Clean build artifacts
cargo clean

# Rebuild
cargo build --release
```

**Problem: Missing dependencies**

```bash
# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated
```

### Runtime Issues

**Problem: Command hangs or times out**

```bash
# Increase timeout (if supported)
# Check logs
raibid-cli --verbose <command>

# Check system resources
top
df -h
```

**Problem: Permission denied**

```bash
# Check file permissions
ls -la ~/.config/raibid/

# Fix permissions
chmod 644 ~/.config/raibid/config.yaml
```

## Tips and Best Practices

### Configuration

1. **Use environment variables for secrets**
   ```bash
   export RAIBID_GITEA_ADMIN_PASSWORD="secure-password"
   ```

2. **Keep local configs for different environments**
   ```bash
   # Development
   cp config-dev.yaml ./raibid.yaml

   # Production
   cp config-prod.yaml ./raibid.yaml
   ```

3. **Validate before applying changes**
   ```bash
   raibid-cli config validate
   ```

### Monitoring

1. **Use TUI for real-time monitoring**
   ```bash
   raibid-cli tui
   ```

2. **Set up periodic status checks**
   ```bash
   # Add to cron
   */5 * * * * raibid-cli status > /var/log/raibid-status.log
   ```

3. **Monitor resource usage**
   ```bash
   raibid-cli agent list
   # Check CPU and Memory columns
   ```

### Performance

1. **Scale agents based on queue depth**
   ```bash
   # Check pending jobs
   raibid-cli job list --status pending

   # Scale accordingly
   raibid-cli agent scale --count <number>
   ```

2. **Use scale-to-zero during idle periods**
   ```bash
   raibid-cli agent scale --count 0
   ```

3. **Optimize sync intervals for mirrors**
   ```bash
   # Frequently updated repos
   raibid-cli mirror add github.com/active/repo --sync-interval 15

   # Stable repos
   raibid-cli mirror add github.com/stable/repo --sync-interval 120
   ```

### Safety

1. **Always validate before teardown**
   ```bash
   raibid-cli job list --status running
   # Make sure no critical jobs are running
   ```

2. **Use force flags cautiously**
   ```bash
   # Prefer confirmation prompts for safety
   raibid-cli job cancel <id>

   # Use --force only when certain
   raibid-cli job cancel <id> --force
   ```

3. **Back up configuration**
   ```bash
   cp ~/.config/raibid/config.yaml ~/.config/raibid/config.yaml.bak
   ```

### Automation

1. **Script common operations**
   ```bash
   #!/bin/bash
   # scale-up.sh
   raibid-cli agent scale --count 8
   raibid-cli agent list
   ```

2. **Use JSON output for parsing**
   ```bash
   raibid-cli job list --output json | jq '.[] | select(.status == "failed")'
   ```

3. **Integrate with CI/CD**
   ```bash
   # In CI pipeline
   raibid-cli mirror sync github.com/myorg/myrepo --force
   ```

## Advanced Topics

### Custom Configuration Locations

```bash
# Use config from specific location
raibid-cli config show --file /custom/path/config.yaml

# Initialize in specific location
raibid-cli config init --output /custom/path/config.yaml
```

### Multiple Environments

```bash
# Development environment
export RAIBID_CONFIG=~/.config/raibid/dev.yaml
raibid-cli config show

# Production environment
export RAIBID_CONFIG=~/.config/raibid/prod.yaml
raibid-cli config show
```

### JSON Processing

```bash
# Parse job data
raibid-cli job list --output json | jq '.[] | {id, status, repository}'

# Filter failed jobs
raibid-cli job list --output json | jq '.[] | select(.status == "Failed")'

# Count jobs by status
raibid-cli job list --output json | jq 'group_by(.status) | map({status: .[0].status, count: length})'
```

### Integration Examples

```bash
# Slack notification for failed jobs
#!/bin/bash
FAILED=$(raibid-cli job list --status failed --output json | jq length)
if [ "$FAILED" -gt 0 ]; then
  curl -X POST -H 'Content-type: application/json' \
    --data "{\"text\":\"⚠️ $FAILED failed jobs detected\"}" \
    $SLACK_WEBHOOK_URL
fi
```

## Getting Help

### Command Help

```bash
# General help
raibid-cli --help

# Command-specific help
raibid-cli job --help
raibid-cli agent --help
raibid-cli config --help

# Subcommand help
raibid-cli job list --help
raibid-cli config init --help
```

### Verbose Logging

```bash
# Enable verbose output
raibid-cli --verbose status

# Debug logging
RUST_LOG=debug raibid-cli tui
RUST_LOG=trace raibid-cli job list
```

### Resources

- **GitHub**: https://github.com/raibid-labs/raibid-cli
- **Issues**: https://github.com/raibid-labs/raibid-cli/issues
- **Man Page**: `man raibid-cli` (after installation)

---

**Last Updated**: 2025-01-15
**Version**: 0.1.0
**Status**: WS-01 Complete
