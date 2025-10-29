# WS-01: CLI/TUI Application

## Description

Build the primary Rust-based CLI/TUI application for managing the CI system. This workstream focuses on the user interface layer first, with mock commands for infrastructure operations. The CLI provides both command-line interface and terminal UI (Ratatui) for monitoring and management.

**Philosophy:** Build the interface first with realistic mock data, then wire it up to real infrastructure later. This allows rapid iteration on UX before infrastructure complexity.

## Dependencies

**Blockers:** None - can start immediately

**Blocks:**
- WS-04: Infrastructure Provisioning (CLI commands will trigger infrastructure)
- WS-08: Integration & Deployment (needs CLI for end-to-end testing)

## Priority

**Critical** - User interface drives development workflow

## Estimated Duration

4-6 days

## Parallelization

Can run in parallel with:
- WS-02: CI Agent Core (build logic development)
- WS-03: API Services (backend development)

Within this workstream:
- CLI-001 must complete first (scaffolding)
- CLI-002 after CLI-001 (mock commands)
- CLI-003, CLI-004 can run in parallel after CLI-002
- CLI-005, CLI-006 can run in parallel after CLI-004
- CLI-007, CLI-008 can run in parallel after CLI-005, CLI-006

## Agent Workflow

Follow this TDD-based workflow for each issue in this workstream:

### 1. Issue Selection & Question Check

**⚠️ CRITICAL: Check for clarifying questions BEFORE starting any work**

- Review all issues in this workstream (listed below)
- Select the next issue that is:
  - Not yet started (no branch exists)
  - Not blocked by dependencies
  - Highest priority among available issues
- Check parallelization notes to identify issues that can run concurrently

**Question Check Protocol:**
1. **Check GitHub issue** for "Clarifying Questions" section
2. **If questions exist and are UNANSWERED:**
   ```bash
   # Post this comment on the GitHub issue
   gh issue comment <issue-number> --body "🤖 **Agent Status: Paused**

   I've been assigned to this issue but found unanswered clarifying questions.
   I'm pausing work until these questions are answered.

   **Unanswered Questions:**
   [List questions that need answers]

   **Current Status:** ⏸️ Paused, monitoring for answers
   **Next Steps:** Will resume automatically when questions are answered

   See docs/CLARIFYING_QUESTIONS.md for details."

   # Report to orchestrator
   # DO NOT proceed to step 2 (Branch Creation)
   # WAIT for orchestrator to signal that questions are answered
   ```
3. **If questions are ANSWERED or NO questions exist:**
   ```bash
   # Post this comment on the GitHub issue
   gh issue comment <issue-number> --body "🤖 **Agent Starting Work**

   Clarifying questions have been answered (or no questions required).
   Proceeding with TDD workflow.

   **Starting:** $(date)
   **Expected Duration:** [duration from issue]
   **Next Update:** Will post progress update in 2-4 hours"

   # Proceed to step 2 (Branch Creation)
   ```

**Reference:** See `docs/CLARIFYING_QUESTIONS.md` for all questions by issue

### 2. Branch Creation
```bash
# Checkout new branch named after the issue
git checkout -b <issue-id>-<brief-description>
# Example: git checkout -b cli-001-project-scaffold
```

### 3. Test-First Development (TDD)
**Write tests BEFORE implementation:**

For Rust CLI/TUI code, create unit and integration tests:

**Example test structure:**
```rust
// tests/cli_commands_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;

    #[test]
    fn test_cli_help_command() {
        let mut cmd = Command::cargo_bin("raibid-cli").unwrap();
        cmd.arg("--help");
        cmd.assert().success();
    }

    #[test]
    fn test_setup_command_mock() {
        let mut cmd = Command::cargo_bin("raibid-cli").unwrap();
        cmd.arg("setup").arg("--dry-run");
        cmd.assert().success();
    }
}
```

**Test types for CLI/TUI:**
- **Unit tests**: Test individual functions and modules
- **Integration tests**: Test CLI commands end-to-end (use `assert_cmd`)
- **UI snapshot tests**: Verify Ratatui rendering (use `insta` crate)
- **Mock tests**: Use mock responses for infrastructure calls

### 4. Initial Test Commit
```bash
# Create test files
mkdir -p tests/
# Write your tests in tests/ directory

# Ensure tests compile (they should fail, but must compile!)
cargo test --no-run

# Add test files
git add src/ tests/ Cargo.toml

# Commit tests
git commit -m "test: add tests for <issue-id>

- Add unit tests for <functionality>
- Add integration tests for <component>
- Tests currently failing (expected before implementation)

Relates to <issue-id>"

# Push to remote
git push -u origin <branch-name>
```

### 5. Implementation
Implement the functionality to make tests pass:
- Follow the task list in the issue description
- Use TDD: write test → watch it fail → implement → watch it pass
- Run `cargo test` frequently
- Use `cargo watch -x test` for automatic test running
- Keep commits small and focused

**For Rust CLI development:**
```bash
# Run tests in watch mode during development
cargo watch -x test

# Run specific test
cargo test test_cli_help_command

# Run CLI manually for testing
cargo run -- setup --dry-run
```

### 6. Implementation Commits
```bash
# Make incremental commits as you implement
git add <files>
git commit -m "feat(<issue-id>): <what you implemented>

<detailed description of changes>

- Implements <feature>
- Adds <functionality>
- Tests passing: <test names>

Relates to <issue-id>"

# Run tests after each change
cargo test

# Push regularly
git push
```

### 7. Final Validation
Before creating PR, ensure:
- [ ] All tests pass (`cargo test`)
- [ ] No compiler warnings (`cargo clippy -- -D warnings`)
- [ ] Code formatted (`cargo fmt --check`)
- [ ] Documentation updated (doc comments, README.md)
- [ ] No hardcoded secrets or credentials
- [ ] Error handling comprehensive
- [ ] Help text clear and complete
- [ ] Success criteria from issue met

### 8. Run Full Test Suite
```bash
# Run all tests
cargo test --all-features

# Run clippy
cargo clippy --all-features -- -D warnings

# Check formatting
cargo fmt --check

# Build for release (ensure it compiles)
cargo build --release

# Test CLI help
cargo run -- --help
```

### 9. Create Pull Request
```bash
# Ensure all changes are committed and pushed
git push

# Create PR via GitHub CLI
gh pr create --title "<issue-id>: <brief description>" \
  --body "## Summary
Implements <issue-id>

## Changes
- Change 1
- Change 2

## Testing
\`\`\`bash
cargo test
cargo run -- --help
\`\`\`

**Test Results:**
- [x] All unit tests passing
- [x] All integration tests passing
- [x] Clippy checks passing
- [x] Formatting checks passing
- [x] CLI commands work as expected

## Checklist
- [x] Tests passing
- [x] Documentation updated
- [x] Issue comments added
- [x] No secrets committed
- [x] Help text added

Closes #<issue-number>"
```

### 10. PR Acceptance Criteria
Your PR must meet ALL of these criteria:

- [ ] **Tests passing**: `cargo test --all-features` succeeds
- [ ] **No warnings**: `cargo clippy -- -D warnings` passes
- [ ] **Formatted**: `cargo fmt --check` passes
- [ ] **Documentation updated**:
  - Doc comments for public APIs
  - README.md updated with CLI usage
  - Help text for all commands
- [ ] **Comments on related issues**:
  - Link PR to issue
  - Document any deviations from plan
  - Note any blockers or dependencies discovered
- [ ] **Code quality**:
  - Error handling comprehensive
  - No unwrap() in production code
  - Logging added appropriately
  - No TODO comments left unresolved
- [ ] **Success criteria met**: All success criteria from issue description satisfied

### 11. Continue to Next Issue
After PR is created and tests are passing:

- **Option A**: If you can build upon the current branch for the next issue:
  ```bash
  # Create new branch from current branch
  git checkout -b <next-issue-id>-<description>
  ```
  This is useful when issues are sequential (e.g., CLI-002 builds on CLI-001)

- **Option B**: If next issue is independent:
  ```bash
  # Return to main and start fresh
  git checkout main
  git pull
  git checkout -b <next-issue-id>-<description>
  ```

- **Option C**: If no issues remain:
  - Document completion in workstream tracking
  - Report readiness to workstream coordinator
  - Offer assistance to other workstreams

### 12. Edge Cases & Issue Management

**If you discover issues during implementation:**
1. Document in PR description
2. Add comment to original issue
3. Create new issue for unexpected work if needed
4. Update workstream README if dependencies change

**If blocked by dependencies:**
1. Comment on issue with blocker details
2. Switch to another non-blocked issue in workstream
3. Notify workstream coordinator
4. Consider helping with blocking workstream

**If tests reveal edge cases:**
1. Add tests for edge cases
2. Implement handling for edge cases
3. Document edge cases in code comments and docs
4. Update issue with findings

## Issues

### CLI-001: Project Scaffolding & CLI Framework
**Priority:** Critical | **Complexity:** Small | **Duration:** 0.5 days

Create the foundational Rust CLI project with clap for argument parsing and basic structure.

**Tasks:**
- [ ] Create Rust project: `cargo new raibid-cli --name raibid`
- [ ] Add core dependencies to `Cargo.toml`:
  - `clap = { version = "4", features = ["derive", "cargo"] }` (CLI argument parsing)
  - `anyhow = "1"` (error handling)
  - `tracing = "0.1"` / `tracing-subscriber = "0.3"` (logging)
  - `tokio = { version = "1", features = ["full"] }` (async runtime)
  - `serde = { version = "1", features = ["derive"] }` (serialization)
  - `serde_yaml = "0.9"` or `toml = "0.8"` (configuration)
  - `colored = "2"` (terminal colors)
- [ ] Set up module structure:
  - `src/main.rs` - Entry point
  - `src/cli/` - CLI commands and argument parsing
  - `src/tui/` - TUI implementation (future)
  - `src/config/` - Configuration management
  - `src/commands/` - Command implementations
- [ ] Create basic CLI with clap:
  ```rust
  #[derive(Parser)]
  #[command(name = "raibid-cli")]
  #[command(version, about, long_about = None)]
  struct Cli {
      #[command(subcommand)]
      command: Commands,
  }
  ```
- [ ] Implement `--help` and `--version` flags
- [ ] Add configuration file support (YAML or TOML in `~/.config/raibid/config.yaml`)
- [ ] Set up logging with `RUST_LOG` environment variable
- [ ] Create README.md with:
  - Installation instructions
  - Basic usage examples
  - Configuration guide
- [ ] Write tests for CLI argument parsing

**Success Criteria:**
- `cargo build` succeeds without warnings
- `cargo run -- --help` displays help text
- `cargo run -- --version` displays version
- Project structure is clear and logical
- Tests pass for basic CLI functionality

---

### CLI-002: Mock Infrastructure Commands
**Priority:** Critical | **Complexity:** Medium | **Duration:** 1.5 days

**THIS IS THE KEY TICKET** - Create placeholder CLI commands for infrastructure management. These commands will print what they would do but not actually execute operations. This establishes the interface contract before infrastructure implementation.

**Tasks:**
- [ ] Create `setup` command with full argument parsing:
  ```bash
  raibid-cli setup [OPTIONS]
    --dry-run              Show what would be done (default)
    --execute              Actually perform setup (disabled in mock)
    --components <LIST>    Components to set up: k3s, gitea, redis, keda, all
    --skip-verify          Skip pre-flight checks
    --config <PATH>        Use custom config file
  ```
- [ ] Create `teardown` command:
  ```bash
  raibid-cli teardown [OPTIONS]
    --dry-run              Show what would be done
    --force                Skip confirmation prompts
    --preserve-data        Keep data volumes
    --components <LIST>    Components to teardown
  ```
- [ ] Create `status` command:
  ```bash
  raibid-cli status [OPTIONS]
    --component <NAME>     Check specific component
    --json                 Output as JSON
    --watch                Continuous monitoring mode
    --verbose             Show detailed status
  ```
- [ ] Implement mock responses (detailed, realistic output):
  - "Would install k3s v1.28 on current node (ARM64)"
  - "Would deploy Gitea 1.21 with 100GB PVC at gitea.dgx.local"
  - "Would configure Redis 7.2 with Streams enabled"
  - "Would deploy KEDA 2.12 with Redis Streams scaler"
  - Show estimated time for each operation
  - Show resource requirements (CPU, RAM, disk)
- [ ] Add pre-flight checks (mock):
  - Check system requirements (CPU, RAM, disk)
  - Check network connectivity
  - Check required ports available
  - Check prerequisites installed (Docker, etc.)
- [ ] Create progress indicators using `indicatif` crate:
  - Spinners for in-progress operations
  - Progress bars for multi-step operations
  - Clear success/failure indicators
- [ ] Implement colored output using `colored` crate:
  - Green for success
  - Yellow for warnings
  - Red for errors
  - Blue for informational
- [ ] Add detailed help text for each command
- [ ] Write comprehensive tests:
  - Unit tests for command parsing
  - Integration tests for command execution
  - Test all flag combinations
- [ ] Create separate issues for each command implementation:
  - Create issue: "Implement setup command - k3s installation"
  - Create issue: "Implement setup command - Gitea deployment"
  - Create issue: "Implement setup command - Redis deployment"
  - Create issue: "Implement teardown command - resource cleanup"
  - Create issue: "Implement status command - cluster health checks"

**Mock Output Example:**
```
$ raibid-cli setup --components k3s,gitea --dry-run

🔍 Pre-flight checks:
  ✓ System requirements met (20 cores, 128GB RAM, 4TB disk)
  ✓ Network connectivity verified
  ✓ Ports 6443, 3000, 2222 available
  ⚠ Docker not installed (will be installed by k3s)

📋 Setup plan:
  1. Install k3s v1.28 (ARM64)
     - Estimated time: 2-3 minutes
     - Resources: 2 cores, 2GB RAM

  2. Deploy Gitea 1.21
     - Estimated time: 5-7 minutes
     - Resources: 2 cores, 4GB RAM, 100GB disk
     - URL: http://gitea.dgx.local:3000

💡 To execute this plan, run:
   raibid-cli setup --components k3s,gitea --execute

⚠️  Note: --execute flag is not yet implemented (mock mode only)
```

**Success Criteria:**
- All commands parse arguments correctly
- `--help` for each command shows comprehensive options
- Commands print clear, actionable mock output with realistic details
- Pre-flight checks execute and show results
- Progress indicators work correctly
- Colored output enhances readability
- Tests pass for all command variations
- Separate issues created for real implementations

---

### CLI-003: Ratatui Setup & Basic Dashboard
**Priority:** High | **Complexity:** Medium | **Duration:** 1.5 days

Set up Ratatui framework and create a basic TUI dashboard with mock data.

**Tasks:**
- [ ] Add Ratatui dependencies:
  - `ratatui = "0.25"`
  - `crossterm = "0.27"`
- [ ] Create TUI entry point: `raibid-cli tui`
- [ ] Set up terminal:
  - Initialize terminal with alternate screen
  - Enter raw mode
  - Hide cursor
  - Set up panic handler for clean terminal restoration
- [ ] Implement event loop:
  - Handle keyboard input (crossterm events)
  - Handle terminal resize
  - Implement refresh timer (1 second)
  - Handle Ctrl+C and 'q' for quit
- [ ] Create basic 3-panel layout using Ratatui Layout:
  - Top 60%: Jobs table panel
  - Bottom-left 20%: Agents list panel
  - Bottom-right 20%: Queue sparkline panel
- [ ] Add header block:
  - App title: "Raibid CI - DGX Spark Agent Pool"
  - System info: hostname, timestamp
  - Connection status indicator
- [ ] Add footer block:
  - Keybindings: `q: Quit | Tab: Switch View | ?: Help`
  - Status message area
- [ ] Implement graceful shutdown:
  - Restore terminal on exit
  - Show cursor
  - Leave alternate screen
  - Disable raw mode
- [ ] Create mock data generators:
  - Generate 20-30 mock jobs with varied states
  - Generate 3-5 mock agents
  - Generate queue depth history (60 data points)
- [ ] Test at different terminal sizes:
  - 80x24 (minimum)
  - 120x40 (common)
  - 200x60 (large)
- [ ] Add color scheme using Ratatui styles
- [ ] Write tests for terminal initialization/cleanup

**Success Criteria:**
- TUI launches without errors
- Terminal restores properly on exit (no artifacts)
- Dashboard renders correctly at 80x24+
- All panels display mock data
- Keybindings work (q to quit, Ctrl+C)
- No flickering or visual artifacts
- Layout adapts to terminal resize
- Tests pass for terminal lifecycle

---

### CLI-004: TUI Widgets & Mock Data Display
**Priority:** High | **Complexity:** Medium | **Duration:** 2 days

Implement detailed TUI widgets with rich mock data display.

**Tasks:**
- [ ] Implement Jobs table widget using `ratatui::widgets::Table`:
  - Columns: ID (6 chars), Status (icon), Repo (20 chars), Branch (15 chars), Started (8 chars), Duration (8 chars)
  - Color coding:
    - Green (success) - `Style::default().fg(Color::Green)`
    - Yellow (running) - `Style::default().fg(Color::Yellow)`
    - Red (failed) - `Style::default().fg(Color::Red)`
    - Gray (pending) - `Style::default().fg(Color::Gray)`
  - Add Unicode icons: ✓ (success), ⟳ (running), ✗ (failed), ○ (pending)
  - Scrolling support (Up/Down arrow keys)
  - Row selection (highlight selected row)
  - Header row with bold styling
- [ ] Implement Agents list widget using `ratatui::widgets::List`:
  - Show agent ID, status icon, current job, CPU%, Memory%
  - Color indicators based on agent state
  - Auto-refresh every second with updated mock data
  - Show "No agents" message when empty
- [ ] Implement Queue depth sparkline using `ratatui::widgets::Sparkline`:
  - Rolling 60-second history
  - Visual representation of queue size (range 0-20)
  - Auto-scroll as new data arrives
  - Show current value above sparkline
  - Color gradient based on queue depth
- [ ] Create realistic mock data generators:
  - Job generator:
    - Random repos: "raibid/core", "raibid/cli", "user/project"
    - Random branches: "main", "develop", "feature/xyz"
    - Random states with transitions
    - Realistic timestamps and durations
  - Agent generator:
    - Simulated state changes (idle → running → idle)
    - Realistic CPU/memory usage (30-80%)
    - Job assignments that match running jobs
  - Queue depth generator:
    - Fluctuate between 0-15 jobs
    - Peaks and valleys for realism
- [ ] Add status bar at bottom of Jobs panel:
  - Total jobs: X running, Y completed, Z failed
  - Active agents: N/10
  - Current queue depth: M
- [ ] Implement tab switching with `Tab` key:
  - Jobs view (default)
  - Agents view (focus on agents panel)
  - System view (show system metrics)
  - Help view (show keybindings)
- [ ] Add visual indicator for selected tab
- [ ] Write tests for widget rendering

**Success Criteria:**
- All widgets render correctly with no layout issues
- Mock data updates every second showing changes
- Scrolling works smoothly (no lag)
- Color coding is clear and accessible
- Tab switching works correctly
- UI performs well with 100+ mock jobs
- Tests pass for widget functionality

---

### CLI-005: Interactive Controls & Navigation
**Priority:** Medium | **Complexity:** Medium | **Duration:** 1.5 days

Add keyboard controls and interactive features to the TUI.

**Tasks:**
- [ ] Implement job detail view:
  - Press `Enter` on selected job to view details
  - Show popup/modal with full job information:
    - Repository URL
    - Commit hash and author
    - Branch name
    - Trigger (webhook, manual, schedule)
    - Start/end timestamps
    - Duration
    - Exit code
    - Agent ID
    - Resource usage
  - Press `Esc` or `q` to return to job list
  - Use `ratatui::widgets::Paragraph` in a centered block
- [ ] Implement log viewer:
  - Press `l` on selected job to view logs
  - Show mock logs in scrollable text area:
    - Generate 50-200 lines of realistic build logs
    - Include timestamps
    - Include log levels (INFO, WARN, ERROR)
    - Include build output (cargo build, test results)
  - Scroll with arrow keys or PgUp/PgDn
  - Search logs with `/` (highlight matches)
  - Press `Esc` to close
- [ ] Add filter/search functionality:
  - Press `/` to open search input bar at bottom
  - Type to filter jobs by:
    - Repository name (partial match)
    - Status (success, failed, running, pending)
    - Branch name
  - Show match count: "X of Y jobs"
  - Press `Esc` to clear search
  - Press `Enter` to cycle through matches
- [ ] Implement help screen:
  - Press `?` to show comprehensive keybindings
  - List all available commands by category:
    - Navigation (arrows, Tab, Enter, Esc)
    - Actions (l for logs, / for search, r for refresh)
    - Views (different tabs)
    - Misc (q for quit, ? for help)
  - Scrollable if content exceeds screen
  - Press any key to close
- [ ] Add refresh control:
  - Press `r` to force immediate refresh
  - Show "Refreshing..." indicator briefly
  - Update all mock data
- [ ] Implement configuration view:
  - Press `c` to view current configuration
  - Display mock configuration settings in table format
  - Show config file path
  - Read-only for now (editing in future issue)
- [ ] Add visual feedback for all interactions:
  - Status messages in footer
  - Loading spinners where appropriate
  - Confirmation messages
- [ ] Write tests for keyboard handling

**Success Criteria:**
- All keybindings work as documented
- Detail views display correctly and are readable
- Log viewer scrolls smoothly
- Search highlights matches correctly
- Navigation is intuitive and responsive
- Help screen is comprehensive and accurate
- No crashes on unexpected input
- Tests pass for all interactions

---

### CLI-006: Additional Mock Commands
**Priority:** Medium | **Complexity:** Medium | **Duration:** 1 day

Add more CLI commands for CI operations (all mocked).

**Tasks:**
- [ ] Create `job` subcommands:
  ```bash
  raibid-cli job list [OPTIONS]
    --status <STATUS>     Filter by status (running, success, failed, pending)
    --repo <REPO>         Filter by repository
    --limit <N>           Show last N jobs (default: 20)
    --json                Output as JSON

  raibid-cli job show <JOB_ID>
    --json                Output as JSON

  raibid-cli job cancel <JOB_ID>
    --force               Skip confirmation

  raibid-cli job retry <JOB_ID>

  raibid-cli job logs <JOB_ID>
    --follow              Stream logs in real-time
    --tail <N>            Show last N lines (default: 100)
  ```
- [ ] Create `agent` subcommands:
  ```bash
  raibid-cli agent list [OPTIONS]
    --status <STATUS>     Filter by status
    --json                Output as JSON

  raibid-cli agent show <AGENT_ID>
    --json                Output as JSON

  raibid-cli agent scale --count <N>
    --min <N>             Set minimum agents (default: 0)
    --max <N>             Set maximum agents (default: 10)
  ```
- [ ] Create `mirror` subcommands:
  ```bash
  raibid-cli mirror add <GITHUB_URL> [OPTIONS]
    --name <NAME>         Custom mirror name
    --sync-interval <M>   Sync interval in minutes (default: 60)

  raibid-cli mirror list [OPTIONS]
    --json                Output as JSON

  raibid-cli mirror sync <REPO>
    --force               Force sync even if up-to-date

  raibid-cli mirror remove <REPO>
    --force               Skip confirmation
  ```
- [ ] Implement JSON output for all commands using `serde_json`:
  - Structured data format
  - Pretty-printed by default
  - Compact option (`--json-compact`)
- [ ] Add table formatting for list commands using `comfy-table`:
  - Clean ASCII tables
  - Column alignment
  - Color support
  - Responsive to terminal width
- [ ] Create mock responses with realistic data:
  - Job listings with 10-50 mock jobs
  - Agent listings with 3-10 mock agents
  - Mirror listings with 5-15 mock mirrors
  - Detailed views with all attributes
- [ ] Add confirmation prompts for destructive operations:
  - Use `dialoguer` crate for interactive prompts
  - "Are you sure you want to cancel job XYZ? [y/N]"
  - `--force` flag skips prompts
- [ ] Write comprehensive tests:
  - Unit tests for each command
  - Integration tests for command execution
  - Test JSON output format
  - Test table output format
  - Test filtering and sorting

**Success Criteria:**
- All commands parse correctly
- Help text is clear and comprehensive
- JSON output is valid and well-structured
- Table output is readable and aligned
- Mock data is realistic
- Confirmation prompts work correctly
- Tests pass for all commands and options

---

### CLI-007: Configuration Management & Examples
**Priority:** Medium | **Complexity:** Small | **Duration:** 1 day

Create configuration file support and example configurations.

**Tasks:**
- [ ] Define configuration schema (YAML):
  ```yaml
  cluster:
    name: "dgx-spark-ci"
    kubeconfig: "~/.kube/config"
    namespace: "ci"

  resources:
    max_agents: 10
    cpu_per_agent: 2
    memory_per_agent: "4Gi"
    storage_class: "local-path"

  cache:
    enabled: true
    size: "50Gi"
    retention_days: 7
    type: "persistent"  # or "ephemeral"

  gitea:
    url: "http://gitea.dgx.local:3000"
    api_token: "${GITEA_TOKEN}"
    registry: "gitea.dgx.local"

  redis:
    url: "redis://redis.dgx.local:6379"
    stream: "ci-jobs"
    consumer_group: "ci-workers"

  api:
    host: "0.0.0.0"
    port: 8080
    webhook_secret: "${WEBHOOK_SECRET}"

  ui:
    refresh_rate: 1000  # milliseconds
    theme: "default"    # default, dark, light
    log_lines: 100      # lines to show in TUI

  monitoring:
    enabled: true
    metrics_port: 9090
  ```
- [ ] Create Rust struct for config using `serde`:
  ```rust
  #[derive(Debug, Deserialize, Serialize)]
  struct Config {
      cluster: ClusterConfig,
      resources: ResourcesConfig,
      cache: CacheConfig,
      // ... etc
  }
  ```
- [ ] Implement config loading with priority:
  1. Command-line flags (highest priority)
  2. Environment variables (with `${VAR}` expansion)
  3. Project-local config: `./raibid.yaml`
  4. User config: `~/.config/raibid/config.yaml`
  5. System config: `/etc/raibid/config.yaml`
  6. Defaults (lowest priority)
- [ ] Implement config validation:
  - Check required fields present
  - Validate value ranges (e.g., cpu_per_agent > 0)
  - Validate URLs and paths
  - Check mutually exclusive options
- [ ] Create `config` subcommands:
  ```bash
  raibid-cli config init [PATH]
    --minimal             Create minimal config
    --full                Create fully documented config
    --overwrite           Overwrite existing config

  raibid-cli config show
    --json                Output as JSON
    --resolved            Show after env var expansion

  raibid-cli config validate [--file PATH]
    --strict              Fail on warnings

  raibid-cli config edit
    # Opens config in $EDITOR

  raibid-cli config path
    # Show which config file is being used
  ```
- [ ] Create example configurations:
  - `examples/config.example.yaml` - Fully documented template
  - `examples/config.minimal.yaml` - Minimal working config
  - `examples/config.production.yaml` - Production-ready settings
  - `examples/config.development.yaml` - Development settings
- [ ] Implement environment variable expansion:
  - `${VAR}` - Required variable (error if not set)
  - `${VAR:-default}` - Optional with default
  - Document all expandable variables
- [ ] Add config merge logic (for multiple config sources)
- [ ] Write configuration guide in README:
  - Configuration file locations
  - Configuration priority
  - Environment variables
  - All configuration options explained
  - Examples for common scenarios
- [ ] Write tests:
  - Test config loading from all sources
  - Test priority/merging
  - Test validation
  - Test environment variable expansion

**Success Criteria:**
- Config files load successfully from all locations
- Validation catches errors clearly with helpful messages
- `config init` creates working config
- Environment variables override file settings correctly
- `config show --resolved` shows fully expanded config
- Documentation is complete and clear
- Tests pass for all config operations

---

### CLI-008: Testing & Documentation
**Priority:** High | **Complexity:** Small | **Duration:** 1 day

Comprehensive testing and documentation for the CLI/TUI.

**Tasks:**
- [ ] Write comprehensive integration tests:
  - Test all CLI commands end-to-end
  - Test TUI launch and shutdown
  - Test configuration loading from multiple sources
  - Test error handling and edge cases
  - Test with invalid inputs
- [ ] Create CLI usage documentation:
  - Complete command reference (all commands, all options)
  - Usage examples for common workflows:
    - Setting up the cluster
    - Monitoring jobs
    - Managing agents
    - Configuring mirrors
  - Configuration guide with all options explained
  - Troubleshooting section:
    - Common errors and solutions
    - Debug mode instructions
    - Log file locations
  - FAQ section
- [ ] Create TUI user guide:
  - Keybindings reference card (printable)
  - Navigation guide
  - Feature walkthrough with screenshots
  - Tips and tricks
- [ ] Add demo video/GIF:
  - Record TUI in action with `asciinema` or `vhs`
  - Show CLI commands and output
  - Demonstrate key features
  - Add to README.md
- [ ] Write developer guide:
  - Architecture overview (modules, dependencies)
  - How to add new CLI commands
  - How to add new TUI widgets
  - How to add new config options
  - Testing guidelines
  - Code style guide
- [ ] Create man page:
  - Generate with `clap_mangen`
  - Install to `/usr/local/share/man/man1/`
- [ ] Add shell completion scripts:
  - Generate with `clap_complete`
  - Bash, Zsh, Fish completions
  - Installation instructions
- [ ] Run security audit:
  - `cargo audit` - check for vulnerabilities
  - `cargo deny check` - check licenses
  - Review dependencies
- [ ] Test on multiple platforms:
  - Linux (Ubuntu 22.04, Arch)
  - macOS (ARM64, x86_64)
  - Test in various terminals (iTerm2, Alacritty, Terminal.app, GNOME Terminal)
  - Test over SSH
- [ ] Optimize binary size:
  - Enable LTO in release profile
  - Strip symbols with `strip`
  - Use `cargo bloat` to find large dependencies
  - Target: < 10MB release binary
- [ ] Create installation guide:
  - Pre-built binaries
  - Building from source
  - Package managers (homebrew, apt)
  - Docker image
- [ ] Add CHANGELOG.md with version history

**Success Criteria:**
- All tests pass (`cargo test --all-features`)
- Test coverage > 80% for CLI code
- Documentation is complete, clear, and accurate
- Demo shows all key features
- Man page is comprehensive
- Shell completions work in bash/zsh
- No security vulnerabilities found
- Binary size < 10MB (release build)
- Tested successfully on Linux and macOS
- Installation guide is easy to follow

---

## Deliverables

- [ ] Rust CLI application with clap-based argument parsing
- [ ] Mock commands for all infrastructure operations (setup, teardown, status, etc.)
- [ ] Full-featured Ratatui TUI with mock data
- [ ] Interactive controls and navigation
- [ ] Configuration management system with multiple sources
- [ ] Comprehensive testing (unit + integration)
- [ ] Complete documentation (CLI reference, TUI guide, examples)
- [ ] Demo video/GIF showing key features
- [ ] README with usage examples and screenshots
- [ ] Man page and shell completions

## Success Criteria

- `cargo build --release` succeeds without warnings
- CLI help text is clear and complete
- All mock commands execute successfully with realistic output
- TUI launches and displays mock data correctly
- TUI performs well over SSH connections (important for DGX Spark)
- Configuration system works correctly with all sources
- All tests pass (unit + integration)
- Documentation is comprehensive and easy to follow
- Binary size < 10MB
- Code passes `cargo clippy` and `cargo audit`

## Notes

- **Focus on UI/UX first** - Real infrastructure integration comes in WS-04
- **Mock data should be realistic** - Helps with testing and development
- **TUI must work over low-bandwidth SSH** - Critical for DGX Spark access
- **CLI commands establish the interface contract** - Infrastructure implementations will match these interfaces
- **Use `assert_cmd` for CLI testing** - Makes integration tests easy
- **Use Ratatui examples as reference** - Well-documented patterns
- **Consider terminal compatibility** - Test with multiple terminals
- **Error messages should be helpful** - Guide users to solutions
- **Create separate issues in CLI-002** for each command implementation that will happen in WS-04

## Future Workstreams Dependencies

This workstream establishes the interface that will be implemented in:
- **WS-04: Infrastructure Provisioning** - Real implementation of setup/teardown/status commands
- **WS-06: GitOps & Orchestration** - Real data for TUI from Kubernetes/KEDA
- **WS-08: Integration & Deployment** - End-to-end testing with real infrastructure
