# GitHub Actions Workflows: Event-Driven Orchestration

This directory contains GitHub Actions workflows that implement the event-driven orchestration system for raibid-ci. These workflows replace the polling-based orchestrator with instant event response.

## Overview

The event-driven system responds to GitHub events in real-time:

- **Issue opened/edited** → Analyze for clarifying questions
- **Comment added** → Check if questions answered
- **PR merged** → Close issue, assign next work

**Performance**: 30-60 second response time (vs 5 minutes with polling)

## Architecture

```
GitHub Event → Workflow → Script → Analysis → Action
                                              ├─ Add labels
                                              ├─ Post comment
                                              └─ Spawn trigger
```

## Workflows

### 1. Issue Events (`orchestrator-issue-events.yml`)

**Triggers**: `issues: [opened, edited, labeled, unlabeled]`

**Purpose**: Detect new issues and analyze readiness for agent assignment

**Flow**:
1. Issue event triggers workflow
2. Script analyzes issue for clarifying questions
3. If no questions or all answered:
   - Add `ready:work` label
   - Post spawn trigger comment
4. If questions unanswered:
   - Add `waiting:answers` label
   - Post paused comment

### 2. Comment Events (`orchestrator-comment-events.yml`)

**Triggers**: `issue_comment: [created, edited]`

**Purpose**: Detect when maintainer answers clarifying questions

**Flow**:
1. Comment event triggers workflow
2. Check if comment contains answer patterns (A1:, Answer:, etc.)
3. Re-analyze issue for completeness
4. If all questions now answered:
   - Add `ready:work` label
   - Remove `waiting:answers` label
   - Post resumption comment
   - Post spawn trigger comment

### 3. PR Events (`orchestrator-pr-events.yml`)

**Triggers**: `pull_request: [closed]`

**Purpose**: Handle work completion and assign next issue

**Flow**:
1. PR merged event triggers workflow
2. Extract issue number from PR body/branch
3. Post completion comment on issue
4. Close issue
5. Find next ready issue (priority-based)
6. Post spawn trigger comment on next issue

## Scripts

### `check-issue-readiness.sh`

Analyzes issue for clarifying questions and determines readiness.

**Logic**:
1. Fetch issue with comments
2. Extract "Clarifying Questions" section
3. Parse question numbers (1., 2., 3., etc.)
4. Search comments for answer patterns
5. Output: `ready=true/false`, `unanswered_count=N`

**Answer Patterns**:
- `A1: Answer text`
- `Answer 1: Answer text`
- `Q1: Question? A: Answer text`
- `1. Answer text` (in comment)

### `spawn-agent-comment.sh`

Posts spawn trigger comment for orchestrator to detect.

**Output**:
```markdown
🤖 **ORCHESTRATOR-SPAWN-AGENT**

**Issue**: #123
**Issue ID**: CLI-001
**Type**: rust-pro
**Status**: ready
**Timestamp**: 2025-10-29T10:30:00Z

<!-- ORCHESTRATOR-STATE {...} -->
```

### `assign-next-issue.sh`

Finds next ready issue based on priority.

**Priority Order**:
1. `priority:critical` label
2. `priority:high` label
3. `priority:medium` label
4. Oldest issue with `ready:work` label

## Labels

| Label | Meaning | Applied By |
|-------|---------|------------|
| `waiting:answers` | Has unanswered questions | Issue/Comment workflows |
| `ready:work` | Ready for agent | Issue/Comment workflows |
| `status:in-progress` | Agent working | Orchestrator |
| `status:completed` | Work done | PR workflow |

## Permissions

Workflows require these permissions (already configured):

```yaml
permissions:
  issues: write        # Update labels, post comments
  pull-requests: read  # Read PR status
  contents: read       # Read repository files
```

`GITHUB_TOKEN` is automatically provided by GitHub Actions.

## Testing

See [TESTING_EVENT_DRIVEN_ORCHESTRATION.md](../docs/TESTING_EVENT_DRIVEN_ORCHESTRATION.md) for comprehensive testing procedures.

**Quick Test**:

```bash
# Create test issue
gh issue create --title "Test: Workflow" --body "Test issue"

# Wait 60 seconds

# Check for spawn comment
gh issue list --label "ready:work" --json number --jq '.[0].number' | \
  xargs gh issue view --json comments --jq '.comments[] | select(.body | contains("ORCHESTRATOR-SPAWN-AGENT"))'
```

## Monitoring

### View Workflow Runs

```bash
# List recent runs
gh run list --limit 10

# View specific workflow
gh run list --workflow=orchestrator-issue-events.yml

# Check for failures
gh run list --status failure

# View run logs
gh run view <run-id> --log
```

### Check Issue State

```bash
# Count issues by state
gh issue list --label "ready:work" --json number --jq '. | length'
gh issue list --label "waiting:answers" --json number --jq '. | length'

# Find issues missing spawn comments
gh issue list --label "ready:work" --json number,comments | \
  jq -r '.[] | select(.comments | map(.body | contains("ORCHESTRATOR-SPAWN-AGENT")) | any | not) | .number'
```

## Troubleshooting

### Workflow Not Triggering

**Check**:
1. Workflows exist on default branch (main)
2. GitHub Actions enabled: `gh api repos/raibid-labs/raibid-cli | jq .has_actions`
3. Recent runs: `gh run list --limit 5`

**Fix**:
- Commit workflows to main: `git push origin main`
- Enable Actions in repo settings

### Spawn Comment Not Posted

**Check**:
1. Workflow completed: `gh run view <run-id>`
2. Script output: `gh run view <run-id> --log | grep "check-issue-readiness"`
3. Labels: `gh issue view <issue> --json labels`

**Fix**:
- Check script permissions: `chmod +x .github/scripts/*.sh`
- Review workflow logs for errors
- Test script locally: `./github/scripts/check-issue-readiness.sh`

### Questions Not Detected as Answered

**Check**:
1. Answer format: `gh issue view <issue> --json comments --jq '.comments[].body'`
2. Run script locally: `export ISSUE_NUMBER=123; ./github/scripts/check-issue-readiness.sh`

**Fix**:
- Use correct answer format: `A1:`, `Answer 1:`, etc.
- Check question numbering matches
- Update regex patterns in script if needed

## Performance Metrics

**Target**:
- Workflow completion: <30 seconds
- Issue detection latency: <60 seconds
- Zero missed events
- Zero duplicate spawns

**Actual** (update after deployment):
- TBD

## Integration with Orchestrator

The orchestrator (Claude Code session) monitors for spawn trigger comments:

1. **Poll every 30 seconds** for comments containing "ORCHESTRATOR-SPAWN-AGENT"
2. **Parse trigger details** (issue number, agent type, issue ID)
3. **Spawn development agent** using Claude Code Task tool
4. **Track active agents** and monitor progress

See [ORCHESTRATOR_AGENT.md](../docs/ORCHESTRATOR_AGENT.md) for details.

## Migration from Polling

**Current State**: Polling every 5 minutes (legacy)

**Migration Plan**:
1. Deploy event-driven workflows (this directory)
2. Run parallel operation (both systems)
3. Validate event-driven system (1 week)
4. Disable polling orchestrator
5. Remove polling code

**Rollback**: Rename `.yml` files to `.yml.disabled` to revert

## Documentation

- **Design**: [EVENT_DRIVEN_ORCHESTRATION.md](../docs/EVENT_DRIVEN_ORCHESTRATION.md)
- **Orchestrator**: [ORCHESTRATOR_AGENT.md](../docs/ORCHESTRATOR_AGENT.md)
- **Testing**: [TESTING_EVENT_DRIVEN_ORCHESTRATION.md](../docs/TESTING_EVENT_DRIVEN_ORCHESTRATION.md)

## Support

For issues or questions:
1. Check workflow run logs: `gh run list --status failure`
2. Review documentation above
3. Test scripts locally
4. Open issue in raibid-labs/raibid-cli

---

**Version**: 1.0
**Status**: Ready for Deployment
**Last Updated**: 2025-10-29
