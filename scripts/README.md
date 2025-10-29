# Raibid-CI Scripts

Automation scripts for managing the raibid-ci multi-agent development workflow.

## Available Scripts

### `launch-orchestrator.nu`

Launches the Claude Code orchestrator agent that coordinates multi-agent parallel development.

**Usage:**
```bash
nu scripts/launch-orchestrator.nu
```

**What it does:**
1. Checks prerequisites (gh CLI, authentication, documentation)
2. Shows current project status (open issues, paused work)
3. Displays orchestrator instructions
4. Provides guidance for spawning the orchestrator agent

**Prerequisites:**
- [Nushell](https://www.nushell.sh/) installed
- [GitHub CLI (gh)](https://cli.github.com/) installed and authenticated
- Claude Code CLI installed and authenticated
- Repository properly configured (see `docs/SETUP_COMPLETE.md`)

**Note:** This script displays instructions and checks prerequisites. To actually spawn the orchestrator agent in Claude Code, use the Task tool as shown in the script output.

## Development Workflow

### 1. Launch Orchestrator
```bash
nu scripts/launch-orchestrator.nu
```

### 2. Orchestrator Monitors Issues
The orchestrator will:
- Monitor GitHub issues every 5 minutes
- Check for clarifying questions that need answers
- Detect when questions are answered
- Spawn development agents for ready issues
- Track progress and dependencies
- Post status updates

### 3. Answer Clarifying Questions
As project maintainer, review and answer questions on GitHub issues:
```bash
gh issue list --label "status:paused"
gh issue view <number>
```

Answer questions in issue comments using this format:
```markdown
## Answers to Clarifying Questions

**Q1: Project naming**
A: Use `raibid` (shorter). Users can alias to `raibid-cli` if they prefer.

**Q2: Configuration format**
A: Use YAML. More common in DevOps tooling and supports comments.
```

### 4. Orchestrator Resumes Agents
Once questions are answered, the orchestrator will:
- Detect the answers
- Post resumption signal on issue
- Spawn or resume development agents
- Agents proceed with TDD workflow

### 5. Monitor Progress
```bash
# View all open issues
gh issue list

# View open PRs
gh pr list

# View CI runs
gh run list

# View issue with comments
gh issue view <number>
```

## Project Status

Current phase: **Ready to Launch** 🎯

**Setup Complete:**
- ✅ 8 workstreams organized (59 total issues)
- ✅ WS-01 issues created (CLI-001 through CLI-008)
- ✅ Clarifying questions posted on all WS-01 issues
- ✅ TDD workflows documented
- ✅ Orchestrator instructions written
- ✅ Repository configured (squash-merge only)

**Next Steps:**
1. Answer clarifying questions on WS-01 issues
2. Launch orchestrator
3. Orchestrator spawns development agents
4. Development begins following TDD workflow

## File Organization

```
scripts/
├── README.md                    # This file
└── launch-orchestrator.nu       # Orchestrator launcher
```

## Additional Resources

- **Orchestrator Guide:** `docs/ORCHESTRATOR_AGENT.md`
- **Questions Document:** `docs/CLARIFYING_QUESTIONS.md`
- **Setup Summary:** `docs/SETUP_COMPLETE.md`
- **Workstreams:** `docs/workstreams/`
- **Quick Start:** `docs/workstreams/START_HERE.md`

## Support

For issues or questions:
- Check `docs/SETUP_COMPLETE.md` for troubleshooting
- Review `docs/ORCHESTRATION.md` for multi-agent workflow
- See `docs/workstreams/START_HERE.md` for quick reference
