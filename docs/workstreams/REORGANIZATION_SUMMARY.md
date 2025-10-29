# Workstream Reorganization - CLI/TUI First Approach

## ✅ Changes Made

### Priority Reordering

**Old Order (Infrastructure First):**
1. WS-01: Infrastructure Core (k3s, networking, storage)
2. WS-02: Data Services (Gitea, Redis)
3. WS-03: GitOps & Orchestration (Flux, KEDA)
4. WS-04: API Services
5. WS-05: Client TUI
6. WS-06: CI Agents
7. WS-07: Repository Management
8. WS-08: Integration & Deployment

**New Order (CLI/TUI First):**
1. **WS-01: CLI/TUI Application** ✅ (was WS-05) - **START HERE**
2. **WS-02: CI Agent Core** (was WS-06)
3. **WS-03: API Services** (was WS-04)
4. **WS-04: Infrastructure Provisioning** (was WS-01) - **DO LATER**
5. **WS-05: Data Services** (was WS-02)
6. **WS-06: GitOps & Orchestration** (was WS-03)
7. **WS-07: Repository Management** (unchanged)
8. **WS-08: Integration & Deployment** (unchanged)

### Philosophy Change

**Before:** Build infrastructure → Build application → Connect them

**After:** Build application with mocks → Build infrastructure → Connect them

**Benefits:**
- Rapid iteration on UX without infrastructure complexity
- CLI establishes interface contract before implementation
- Can develop/test CLI independently
- Clear separation of concerns
- Parallelizable work (CLI, API, Agent Core can all start immediately)

## 🎯 New WS-01: CLI/TUI Application

### Completely Rewritten
File: `docs/workstreams/01-cli-tui-application/README.md`

### 8 New Issues (CLI-001 through CLI-008)

**Key Ticket: CLI-002 - Mock Infrastructure Commands**
- Creates `setup`, `teardown`, `status` commands
- All commands print mock output (no real execution)
- Shows realistic output with progress indicators
- **Creates separate GitHub issues** for real implementations in WS-04:
  - "Implement setup command - k3s installation"
  - "Implement setup command - Gitea deployment"
  - "Implement setup command - Redis deployment"
  - "Implement teardown command - resource cleanup"
  - "Implement status command - cluster health checks"

### Other CLI Issues
- **CLI-001:** Project scaffolding with clap
- **CLI-003:** Ratatui setup & basic dashboard
- **CLI-004:** TUI widgets with mock data
- **CLI-005:** Interactive controls & navigation
- **CLI-006:** Additional mock commands (job, agent, mirror subcommands)
- **CLI-007:** Configuration management
- **CLI-008:** Testing & documentation

### Example Mock Output
```bash
$ raibid-cli setup --components k3s,gitea --dry-run

🔍 Pre-flight checks:
  ✓ System requirements met (20 cores, 128GB RAM, 4TB disk)
  ✓ Network connectivity verified
  ✓ Ports 6443, 3000, 2222 available

📋 Setup plan:
  1. Install k3s v1.28 (ARM64)
     - Estimated time: 2-3 minutes
     - Resources: 2 cores, 2GB RAM

  2. Deploy Gitea 1.21
     - Estimated time: 5-7 minutes
     - Resources: 2 cores, 4GB RAM, 100GB disk

💡 To execute this plan, run:
   raibid-cli setup --components k3s,gitea --execute

⚠️  Note: --execute flag is not yet implemented (mock mode only)
```

## 🚀 Quick Start (Updated)

### Phase 1: Application Layer (Start Immediately)
Launch 3 agents in parallel:

```javascript
Task("CLI/TUI Developer",
     "Complete WS-01: CLI/TUI Application. Follow docs/workstreams/01-cli-tui-application/README.md. Focus on mock commands in CLI-002.",
     "rust-pro")

Task("API Developer",
     "Complete WS-03: API Services. Follow docs/workstreams/03-api-services/README.md. Build backend with mock data.",
     "rust-pro")

Task("Agent Developer",
     "Complete WS-02: CI Agent Core. Follow docs/workstreams/02-ci-agent-core/README.md. Build pipeline logic.",
     "rust-pro")
```

### Phase 2: Infrastructure Layer (After Application)
**Only start after WS-01 CLI-002 creates the interface issues:**

```javascript
Task("Infrastructure Specialist",
     "Complete WS-04: Infrastructure Provisioning. Implement issues created by WS-01 CLI-002. Follow docs/workstreams/04-infrastructure-provisioning/README.md.",
     "cloud-architect")
```

## 📝 Status of Updates

### ✅ Completed
- [x] WS-01 completely rewritten (01-cli-tui-application/README.md)
- [x] Workstream directories renamed
- [x] TDD workflow added to WS-01

### ⏳ Needs Update
- [ ] WS-02: CI Agent Core (rename, update dependencies)
- [ ] WS-03: API Services (already good, just update references)
- [ ] WS-04: Infrastructure Provisioning (add issues from CLI-002)
- [ ] WS-05: Data Services (update dependencies)
- [ ] WS-06: GitOps & Orchestration (update dependencies)
- [ ] docs/ORCHESTRATION.md (update phases and priorities)
- [ ] docs/workstreams/START_HERE.md (update launch commands)
- [ ] docs/workstreams/README.md (update order)
- [ ] docs/diagrams/workstream-dependencies.md (update diagram)

## 🎯 Next Actions

1. **Review WS-01** - Verify the CLI/TUI approach is correct
2. **Update remaining workstreams** - Update dependencies and references
3. **Update orchestration docs** - Reflect new priorities
4. **Test workflow** - Ensure agents can start WS-01 immediately

## 💡 Key Insight

By building the CLI/TUI first with mock commands, we:
1. **Define the interface contract** - CLI commands establish what infrastructure must do
2. **Enable parallel development** - CLI, API, and Agent Core can all develop simultaneously
3. **Test UX early** - Get feedback on usability before infrastructure complexity
4. **Reduce risk** - Infrastructure implementation guided by CLI interface needs
5. **Create issues automatically** - CLI-002 generates the infrastructure work tickets

The CLI becomes the **specification** for infrastructure implementation!
