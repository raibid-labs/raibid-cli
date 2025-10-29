# Clarifying Questions for All Issues

This document contains clarifying questions that must be answered before agents can begin work on each issue. These questions should be posted as comments on the GitHub issues when they are created.

## How This Works

1. **When creating GitHub issues**: Post these questions as initial comments
2. **Before starting work**: Agents check if questions are answered
3. **If unanswered**: Agent reports to orchestrator and pauses
4. **When answered**: Orchestrator detects responses and resumes agent
5. **Agent continues**: Work proceeds with clarified requirements

---

## WS-01: CLI/TUI Application

### CLI-001: Project Scaffolding & CLI Framework

**Clarifying Questions:**

1. **Project naming**: Should the binary be named `raibid` or `raibid-cli`? This affects `cargo new` command and user experience.
   - Option A: `raibid` (shorter, cleaner)
   - Option B: `raibid-cli` (explicit, clear it's a CLI tool)

2. **Configuration format**: Should we use YAML or TOML for configuration files?
   - YAML: More human-readable, supports comments, common in DevOps
   - TOML: Rust ecosystem standard (Cargo.toml), simpler parsing
   - Recommendation needed

3. **Module structure**: Should `commands/` be nested under `cli/` or be a top-level module?
   - Option A: `src/cli/commands/` - All CLI code together
   - Option B: `src/commands/` - Commands separate from arg parsing

4. **Async runtime**: Do we need tokio for CLI-001 or can we add it later when needed?
   - CLI commands might not need async initially
   - Could reduce initial dependencies
   - Or add now for consistency?

**Status**: ⏸️ PAUSED - Awaiting responses

**Agent Instructions**: Post these questions on GitHub issue, report to orchestrator, pause work.

---

### CLI-002: Mock Infrastructure Commands

**Clarifying Questions:**

1. **Component dependencies**: Should the mock output show component dependencies?
   - Example: "Gitea requires k3s, would install k3s first"
   - Or just list components in order without explaining dependencies?

2. **Pre-flight check depth**: How detailed should pre-flight checks be in mock mode?
   - Option A: Just check basics (disk space, RAM, CPU count)
   - Option B: Also check ports, network, DNS
   - Option C: Full validation including k8s connectivity tests

3. **Error simulation**: Should mock commands simulate errors for testing?
   - Add a `--simulate-error` flag for testing error handling?
   - Mock different failure scenarios?
   - Or keep it simple with success-only mocks?

4. **Issue creation timing**: When should we create the sub-issues for real implementations?
   - During CLI-002 PR (automated via script)?
   - After CLI-002 merges (manual creation)?
   - As part of the task list in CLI-002?

5. **Component list**: Are these the complete set of components, or are more needed?
   - Current: k3s, gitea, redis, keda, all
   - Missing: flux, prometheus, grafana?
   - Should "all" include everything or just MVP?

6. **Dry-run default**: Should `--dry-run` be the default or require explicit flag?
   - Option A: Dry-run by default (safer)
   - Option B: Real execution by default (more intuitive)
   - Current design has dry-run as default

**Status**: ⏸️ PAUSED - Awaiting responses

**Agent Instructions**: This is a critical decision point. Do not proceed without answers.

---

### CLI-003: Ratatui Setup & Basic Dashboard

**Clarifying Questions:**

1. **Panel proportions**: Are the suggested proportions (60% jobs, 20% agents, 20% queue) optimal?
   - Based on what user needs to see most
   - Adjustable via config later?

2. **Mock data quantity**: How many mock jobs should be generated initially?
   - 10 jobs (minimal)
   - 20-30 jobs (suggested)
   - 50+ jobs (stress test)
   - Configurable?

3. **Update frequency**: Is 1-second refresh too fast or too slow?
   - Consider SSH latency
   - Battery/CPU usage
   - User perception
   - Make configurable?

4. **Terminal minimum size**: Should we enforce minimum terminal size or gracefully degrade?
   - Hard minimum: 80x24 (error if smaller)
   - Soft minimum: Show message if too small but still try
   - No minimum: Adapt to any size

**Status**: ⏸️ PAUSED - Awaiting responses

---

### CLI-004: TUI Widgets & Mock Data Display

**Clarifying Questions:**

1. **Table column widths**: Are the suggested column widths appropriate?
   - ID: 6 chars - enough for UUID prefix?
   - Repo: 20 chars - enough for "org/repository"?
   - Branch: 15 chars - enough for feature branch names?

2. **Sorting default**: What should be the default sort order for jobs table?
   - Most recent first (by start time)?
   - Status priority (running > failed > success > pending)?
   - User choice?

3. **Tab persistence**: Should selected tab persist across TUI restarts?
   - Save to config file?
   - Always start on Jobs tab?

4. **Color accessibility**: Should we provide alternative color schemes for color blindness?
   - Shapes/symbols in addition to colors?
   - High contrast mode?
   - MVP or future?

**Status**: ⏸️ PAUSED - Awaiting responses

---

### CLI-005: Interactive Controls & Navigation

**Clarifying Questions:**

1. **Modal vs split view**: Should job details be shown in a modal/popup or split view?
   - Modal: Cleaner, focused
   - Split: See list while viewing details
   - Current design: Modal (popup)

2. **Log line limit**: What's the maximum number of log lines to keep in memory?
   - Trade-off between completeness and memory usage
   - 100 lines (current)?
   - 1000 lines?
   - Configurable?

3. **Search behavior**: Should search be case-sensitive or case-insensitive by default?
   - Case-insensitive more user-friendly
   - Case-sensitive more precise
   - Toggle with flag?

4. **Config editing**: Should pressing 'c' allow editing or just viewing?
   - Current: View only
   - Future: Edit inline?
   - Open in $EDITOR?

**Status**: ⏸️ PAUSED - Awaiting responses

---

### CLI-006: Additional Mock Commands

**Clarifying Questions:**

1. **Command structure**: Should all subcommands be under `raibid-cli` or have shortcuts?
   - Current: `raibid-cli job list`
   - Alternative: `raibid job list` (alias)
   - Or both?

2. **JSON format**: Should JSON output be compact or pretty-printed by default?
   - Pretty for humans (readable)
   - Compact for machines (smaller)
   - Flag to control?

3. **Table styling**: What table style should we use?
   - ASCII (classic, universal)
   - Unicode (pretty, modern)
   - Both with flag?

4. **Confirmation prompts**: Should confirmations show what will happen?
   - "Cancel job XYZ (running for 5 min, 45% complete)?" (detailed)
   - "Cancel job XYZ?" (simple)

**Status**: ⏸️ PAUSED - Awaiting responses

---

### CLI-007: Configuration Management & Examples

**Clarifying Questions:**

1. **Config merge strategy**: How should we merge configs from multiple sources?
   - Deep merge (merge nested objects)?
   - Shallow merge (top-level only)?
   - Array handling (append or replace)?

2. **Environment variable naming**: What prefix for environment variables?
   - `RAIBID_*` (explicit)
   - `RBC_*` (short for Raibid CI)
   - No prefix (risky, might conflict)

3. **Config validation strictness**: Should validation be strict or permissive by default?
   - Strict: Error on unknown fields (catch typos)
   - Permissive: Warn on unknown fields (forward compatibility)
   - Flag to control?

4. **Example configs completeness**: Should examples include all possible options or just common ones?
   - All options: Comprehensive but overwhelming
   - Common only: Easier to start but incomplete reference
   - Both: Multiple example files?

**Status**: ⏸️ PAUSED - Awaiting responses

---

### CLI-008: Testing & Documentation

**Clarifying Questions:**

1. **Test coverage target**: Is 80% coverage sufficient or should we aim higher?
   - 80%: Industry standard, achievable
   - 90%: More thorough, more effort
   - Different targets for different modules?

2. **Platform testing priority**: Which platforms are MVP vs nice-to-have?
   - MVP: Linux (Ubuntu 22.04), macOS ARM64
   - Nice: Other Linux distros, macOS x86, Windows
   - DGX Spark (ARM64 Linux) is primary target

3. **Binary size optimization**: Is 10MB target strict or approximate?
   - <10MB: Hard requirement
   - ~10MB: Guideline
   - Trade size for features if needed?

4. **Man page scope**: Should man page cover all commands or just main command?
   - All commands: Comprehensive (like git)
   - Main only: Simpler, point to --help for subcommands
   - Separate man pages per subcommand?

**Status**: ⏸️ PAUSED - Awaiting responses

---

## WS-02: CI Agent Core

*Note: Full analysis not yet completed. Questions TBD based on issue definitions.*

**Placeholder Questions:**
- Docker-in-Docker vs Docker socket mount?
- Build cache strategy and size limits?
- Rust toolchain versions to support?
- Test execution timeout limits?

---

## WS-03: API Services

*Note: Full analysis not yet completed. Questions TBD based on issue definitions.*

**Placeholder Questions:**
- Authentication mechanism (JWT, API keys, both)?
- Rate limiting strategy and limits?
- Webhook signature validation algorithm?
- Database for job history or Redis only?

---

## WS-04: Infrastructure Provisioning

*Note: This workstream depends on CLI-002 creating sub-issues. Questions will be added when those issues are created.*

**Placeholder Questions:**
- k3s version pinning or latest stable?
- Gitea admin password generation or configuration?
- Redis persistence settings (AOF, RDB, both)?
- Storage class for PVCs (local-path, NFS, Longhorn)?

---

## WS-05: Data Services

*Note: Full analysis not yet completed. Questions TBD based on issue definitions.*

**Placeholder Questions:**
- Gitea backup frequency and retention?
- Redis maxmemory policy?
- PostgreSQL version for Gitea backend?
- SSL/TLS for services (internal cluster traffic)?

---

## WS-06: GitOps & Orchestration

*Note: Full analysis not yet completed. Questions TBD based on issue definitions.*

**Placeholder Questions:**
- Flux sync interval?
- KEDA polling interval?
- ScaledJob maxReplicaCount for MVP?
- Secret management strategy (SOPS, Sealed Secrets, none)?

---

## WS-07: Repository Management

*Note: Full analysis not yet completed. Questions TBD based on issue definitions.*

**Placeholder Questions:**
- Mirror sync frequency (hourly, on-push, both)?
- GitHub rate limit handling strategy?
- Private repo support in MVP?
- Mirror naming conventions?

---

## WS-08: Integration & Deployment

*Note: This workstream is final integration. Questions will emerge from earlier workstreams.*

**Placeholder Questions:**
- Performance benchmark targets firm or guidelines?
- Failure scenario coverage completeness?
- Production readiness checklist reviewer?
- Go-live criteria for MVP?

---

## Orchestrator Protocol

### Question Lifecycle

```
1. Issue Created
   ↓
2. Questions Posted (from this doc)
   ↓
3. Agent Assigned to Issue
   ↓
4. Agent Checks for Unanswered Questions
   ↓
5a. Questions Unanswered          5b. Questions Answered
    ↓                                  ↓
6a. Agent Pauses & Reports        6b. Agent Proceeds with Work
    ↓
7. Orchestrator Monitors Issue
   ↓
8. Answers Detected
   ↓
9. Orchestrator Resumes Agent
   ↓
10. Agent Proceeds with Work
```

### Agent Behavior

**When starting a new issue:**
```bash
1. git checkout -b <issue-id>-description
2. Check GitHub issue for comments
3. Look for "Clarifying Questions" section
4. If questions present and unanswered:
   a. Post comment: "Paused: Awaiting responses to clarifying questions"
   b. Report to orchestrator: "Issue <ID> paused pending clarification"
   c. Do NOT start writing tests or code
   d. Do NOT commit anything
5. If questions answered or no questions:
   a. Proceed with TDD workflow
   b. Start writing tests
```

### Orchestrator Behavior

**Monitoring loop (every 5 minutes):**
```bash
1. Query GitHub API for all open issues with "Clarifying Questions"
2. Check for new comments since last check
3. Detect if questions have been answered:
   - Look for maintainer/owner responses
   - Look for comment with "Answer:" or "A:" prefix
   - Look for edits to issue description
4. If new answers detected:
   - Identify paused agent for that issue
   - Post comment: "@agent-name questions answered, resuming work"
   - Signal agent to resume
5. Update tracking: issue -> answered timestamp
```

### Question Answer Format

**For project maintainer answering questions:**
```markdown
## Answers to Clarifying Questions

**Q1: Project naming**
A: Use `raibid` (shorter). Users can alias to `raibid-cli` if they prefer.

**Q2: Configuration format**
A: Use YAML. More common in DevOps tooling and supports comments.

**Q3: Module structure**
A: Use `src/commands/` as top-level. Commands might be shared between CLI and TUI.

**Q4: Async runtime**
A: Add tokio now. We'll need it soon anyway and it's easier to have from the start.
```

### Integration with Agent Workflow

**Updated step 1 (Issue Selection) to include:**
```
1. Issue Selection
   - Review all issues in this workstream
   - Select next issue (highest priority, not blocked)
   - **CHECK GITHUB ISSUE FOR CLARIFYING QUESTIONS**
   - If questions exist and are unanswered:
     * Post comment on issue: "Agent assigned. Pausing until clarifying questions are answered."
     * Report to orchestrator: "Paused on issue <ID>"
     * Do NOT proceed to step 2
     * Wait for orchestrator signal to resume
   - If no questions or questions answered:
     * Post comment: "Agent starting work on issue"
     * Proceed to step 2 (Branch Creation)
```

## Priority Guidance

**Questions marked "critical decision point":**
- Block all work until answered
- Might affect other issues
- Orchestrator should prioritize getting answers

**Questions marked "nice to have":**
- Agent can make reasonable assumption
- Document assumption in PR
- Can be changed later if needed

**Questions marked "affects architecture":**
- Can impact multiple workstreams
- Should involve multiple stakeholders
- Orchestrator should escalate

## Example Workflow

**Day 1:**
```
09:00 - Agent assigned to CLI-001
09:01 - Agent checks issue, finds questions
09:01 - Agent posts: "Paused pending clarification"
09:02 - Agent reports to orchestrator
09:05 - Orchestrator logs: "CLI-001 paused, 4 questions pending"
10:30 - Maintainer posts answers
10:35 - Orchestrator detects answers (next monitor cycle)
10:36 - Orchestrator posts: "Questions answered, resuming"
10:36 - Agent receives signal, resumes work
10:37 - Agent starts TDD workflow
```

**Day 2:**
```
09:00 - Agent assigned to CLI-003
09:01 - Agent checks issue, finds questions
09:01 - Some questions already answered, some not
09:02 - Agent asks: "Q1 and Q2 answered, but Q3 and Q4 still pending. Should I proceed?"
09:03 - Maintainer: "Q3 is critical, Q4 you can assume. Proceed with Q4 assumption."
09:04 - Agent proceeds, documents Q4 assumption in code comments
```

## Benefits of This Approach

1. **Prevents wasted work**: Agents don't implement wrong solutions
2. **Clarifies requirements early**: Questions surfaced before coding begins
3. **Improves quality**: Decisions documented on issues
4. **Enables parallel work**: Other agents continue while one is paused
5. **Creates audit trail**: All clarifications visible in issue history
6. **Flexible**: Agents can work on other issues while waiting

## Next Steps

1. Create GitHub issues for all workstreams
2. Post clarifying questions from this document as initial comments
3. Set up orchestrator monitoring script
4. Update agent prompts to include question-checking step
5. Test the pause/resume workflow with a pilot issue
