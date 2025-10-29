# Testing & Validation Plan: Event-Driven Orchestration

## Overview

This document provides comprehensive testing procedures for the event-driven orchestration system. The system must reliably detect GitHub events, analyze issue readiness, spawn agents, and complete the full workflow without missing events or duplicating work.

## Test Environment Setup

### Prerequisites

1. **GitHub Repository**: raibid-labs/raibid-cli with admin access
2. **GitHub CLI**: `gh` authenticated with proper permissions
3. **Git**: Workflows committed to default branch (main)
4. **Permissions**: GitHub Actions enabled, workflows have write access to issues

### Verification

```bash
# Check GitHub CLI authentication
gh auth status

# Verify repository access
gh repo view raibid-labs/raibid-cli

# Check GitHub Actions enabled
gh api repos/raibid-labs/raibid-cli | jq .has_issues,.has_actions

# List workflows
gh workflow list
```

## Test Phases

### Phase 1: Unit Tests (Scripts)

Test individual scripts in isolation before deploying workflows.

#### Test 1.1: Check Issue Readiness Script

**Test Case**: Issue with no clarifying questions

```bash
# Create test issue
ISSUE_NUM=$(gh issue create --title "Test: No Questions" --body "## Description\nSimple test issue" | grep -oP '#\K\d+')

# Test script
export ISSUE_NUMBER=$ISSUE_NUM
export GITHUB_TOKEN=$(gh auth token)
./.github/scripts/check-issue-readiness.sh

# Expected output:
# ready=true
# unanswered_count=0

# Cleanup
gh issue close $ISSUE_NUM
```

**Test Case**: Issue with unanswered questions

```bash
# Create test issue with questions
BODY="## Description
Test issue

## Clarifying Questions
1. Question one?
2. Question two?"

ISSUE_NUM=$(gh issue create --title "Test: Unanswered Questions" --body "$BODY" | grep -oP '#\K\d+')

# Test script
export ISSUE_NUMBER=$ISSUE_NUM
./.github/scripts/check-issue-readiness.sh

# Expected output:
# ready=false
# unanswered_count=2
# total_questions=2

# Cleanup
gh issue close $ISSUE_NUM
```

**Test Case**: Issue with answered questions

```bash
# Create issue with questions
BODY="## Description
Test issue

## Clarifying Questions
1. Question one?
2. Question two?"

ISSUE_NUM=$(gh issue create --title "Test: Answered Questions" --body "$BODY" | grep -oP '#\K\d+')

# Post answers
gh issue comment $ISSUE_NUM --body "A1: Answer one
A2: Answer two"

# Test script
export ISSUE_NUMBER=$ISSUE_NUM
./.github/scripts/check-issue-readiness.sh

# Expected output:
# ready=true
# unanswered_count=0

# Cleanup
gh issue close $ISSUE_NUM
```

#### Test 1.2: Spawn Agent Comment Script

```bash
# Create test issue
ISSUE_NUM=$(gh issue create --title "CLI-001: Test Agent Spawn" --body "Test issue" | grep -oP '#\K\d+')

# Test script
export ISSUE_NUMBER=$ISSUE_NUM
./.github/scripts/spawn-agent-comment.sh

# Verify spawn comment posted
gh issue view $ISSUE_NUM --json comments --jq '.comments[] | select(.body | contains("ORCHESTRATOR-SPAWN-AGENT"))'

# Expected: Comment with ORCHESTRATOR-SPAWN-AGENT marker

# Cleanup
gh issue close $ISSUE_NUM
```

#### Test 1.3: Assign Next Issue Script

```bash
# Create multiple test issues with labels
gh issue create --title "Test: Critical Priority" --body "Test" --label "ready:work,priority:critical"
gh issue create --title "Test: High Priority" --body "Test" --label "ready:work,priority:high"
gh issue create --title "Test: No Priority" --body "Test" --label "ready:work"

# Test script
./.github/scripts/assign-next-issue.sh

# Expected output: Issue number of critical priority issue

# Cleanup
gh issue list --label "ready:work" --json number --jq '.[].number' | xargs -I {} gh issue close {}
```

### Phase 2: Integration Tests (Workflows)

Test GitHub Actions workflows end-to-end.

#### Test 2.1: Issue Opened Event

**Scenario**: New issue created without clarifying questions

```bash
# Create issue (triggers workflow)
ISSUE_NUM=$(gh issue create \
  --title "Test: Issue Opened (No Questions)" \
  --body "## Description\nTest issue without questions" | \
  grep -oP '#\K\d+')

# Wait for workflow to complete (30-60 seconds)
sleep 60

# Verify workflow ran
gh run list --workflow=orchestrator-issue-events.yml --limit 1

# Verify labels added
LABELS=$(gh issue view $ISSUE_NUM --json labels --jq '.labels[].name')
echo "$LABELS" | grep -q "ready:work"

# Verify spawn comment posted
SPAWN_COMMENT=$(gh issue view $ISSUE_NUM --json comments --jq '.comments[] | select(.body | contains("ORCHESTRATOR-SPAWN-AGENT"))')
[ -n "$SPAWN_COMMENT" ] && echo "✅ Spawn comment posted" || echo "❌ Spawn comment missing"

# Cleanup
gh issue close $ISSUE_NUM
```

**Scenario**: New issue created with clarifying questions

```bash
# Create issue with questions (triggers workflow)
BODY="## Description
Test issue

## Clarifying Questions
1. **Question one**: How should this work?
2. **Question two**: Which approach to use?"

ISSUE_NUM=$(gh issue create \
  --title "Test: Issue Opened (With Questions)" \
  --body "$BODY" | \
  grep -oP '#\K\d+')

# Wait for workflow
sleep 60

# Verify labels
LABELS=$(gh issue view $ISSUE_NUM --json labels --jq '.labels[].name')
echo "$LABELS" | grep -q "waiting:answers"

# Verify paused comment
PAUSED_COMMENT=$(gh issue view $ISSUE_NUM --json comments --jq '.comments[] | select(.body | contains("Agent Status: Paused"))')
[ -n "$PAUSED_COMMENT" ] && echo "✅ Paused comment posted" || echo "❌ Paused comment missing"

# Verify NO spawn comment
SPAWN_COMMENT=$(gh issue view $ISSUE_NUM --json comments --jq '.comments[] | select(.body | contains("ORCHESTRATOR-SPAWN-AGENT"))')
[ -z "$SPAWN_COMMENT" ] && echo "✅ Correctly NOT spawned" || echo "❌ Incorrectly spawned"

# Cleanup
gh issue close $ISSUE_NUM
```

#### Test 2.2: Comment Created Event (Answers Provided)

**Scenario**: Maintainer answers clarifying questions

```bash
# Create issue with questions
BODY="## Description
Test issue

## Clarifying Questions
1. Question one?
2. Question two?"

ISSUE_NUM=$(gh issue create \
  --title "Test: Answer Questions" \
  --body "$BODY" | \
  grep -oP '#\K\d+')

# Wait for initial workflow (should mark as waiting:answers)
sleep 60

# Post answers (triggers comment workflow)
gh issue comment $ISSUE_NUM --body "A1: Answer to question one
A2: Answer to question two"

# Wait for workflow
sleep 60

# Verify labels updated
LABELS=$(gh issue view $ISSUE_NUM --json labels --jq '.labels[].name')
echo "$LABELS" | grep -q "ready:work" && echo "✅ ready:work label added"
echo "$LABELS" | grep -q "waiting:answers" && echo "❌ waiting:answers not removed" || echo "✅ waiting:answers removed"

# Verify resumption comment
RESUME_COMMENT=$(gh issue view $ISSUE_NUM --json comments --jq '.comments[] | select(.body | contains("Questions Answered - Resuming Work"))')
[ -n "$RESUME_COMMENT" ] && echo "✅ Resumption comment posted" || echo "❌ Resumption comment missing"

# Verify spawn comment
SPAWN_COMMENT=$(gh issue view $ISSUE_NUM --json comments --jq '.comments[] | select(.body | contains("ORCHESTRATOR-SPAWN-AGENT"))')
[ -n "$SPAWN_COMMENT" ] && echo "✅ Spawn comment posted" || echo "❌ Spawn comment missing"

# Cleanup
gh issue close $ISSUE_NUM
```

#### Test 2.3: Pull Request Merged Event

**Scenario**: PR merged triggers issue closure and next issue assignment

```bash
# Setup: Create issue and branch
ISSUE_NUM=$(gh issue create \
  --title "Test: PR Merge Workflow" \
  --body "Test issue" | \
  grep -oP '#\K\d+')

git checkout -b test-pr-$ISSUE_NUM
echo "test" > test.txt
git add test.txt
git commit -m "Test commit"
git push -u origin test-pr-$ISSUE_NUM

# Create PR
PR_NUM=$(gh pr create \
  --title "Test: PR for issue #$ISSUE_NUM" \
  --body "Closes #$ISSUE_NUM" \
  --head test-pr-$ISSUE_NUM | \
  grep -oP '#\K\d+')

# Create next ready issue
NEXT_ISSUE=$(gh issue create \
  --title "Test: Next Issue" \
  --body "Next work" \
  --label "ready:work" | \
  grep -oP '#\K\d+')

# Merge PR (triggers workflow)
gh pr merge $PR_NUM --squash

# Wait for workflow
sleep 60

# Verify completion comment on original issue
COMPLETION=$(gh issue view $ISSUE_NUM --json comments --jq '.comments[] | select(.body | contains("Work Completed"))')
[ -n "$COMPLETION" ] && echo "✅ Completion comment posted" || echo "❌ Completion comment missing"

# Verify issue closed
STATE=$(gh issue view $ISSUE_NUM --json state --jq .state)
[ "$STATE" = "CLOSED" ] && echo "✅ Issue closed" || echo "❌ Issue not closed"

# Verify next issue has spawn comment
NEXT_SPAWN=$(gh issue view $NEXT_ISSUE --json comments --jq '.comments[] | select(.body | contains("ORCHESTRATOR-SPAWN-AGENT"))')
[ -n "$NEXT_SPAWN" ] && echo "✅ Next issue spawn comment posted" || echo "❌ Next issue spawn comment missing"

# Cleanup
git checkout main
git branch -D test-pr-$ISSUE_NUM
git push origin --delete test-pr-$ISSUE_NUM
gh issue close $NEXT_ISSUE
```

### Phase 3: Edge Case Tests

#### Test 3.1: Rapid Issue Creation

**Scenario**: Multiple issues created simultaneously

```bash
# Create 5 issues in parallel
for i in {1..5}; do
  gh issue create \
    --title "Test: Rapid Creation $i" \
    --body "## Description\nTest issue $i" &
done
wait

# Wait for all workflows
sleep 90

# Verify all have spawn comments
READY_ISSUES=$(gh issue list --label "ready:work" --json number --jq '.[].number')
for issue in $READY_ISSUES; do
  SPAWN=$(gh issue view $issue --json comments --jq '.comments[] | select(.body | contains("ORCHESTRATOR-SPAWN-AGENT"))')
  if [ -n "$SPAWN" ]; then
    echo "✅ Issue #$issue has spawn comment"
  else
    echo "❌ Issue #$issue missing spawn comment"
  fi
done

# Cleanup
echo "$READY_ISSUES" | xargs -I {} gh issue close {}
```

#### Test 3.2: Partial Answers

**Scenario**: User answers some but not all questions

```bash
# Create issue with 3 questions
BODY="## Description
Test

## Clarifying Questions
1. Question one?
2. Question two?
3. Question three?"

ISSUE_NUM=$(gh issue create \
  --title "Test: Partial Answers" \
  --body "$BODY" | \
  grep -oP '#\K\d+')

sleep 60

# Answer only 2 questions
gh issue comment $ISSUE_NUM --body "A1: Answer one
A2: Answer two"

sleep 60

# Verify still waiting (not ready)
LABELS=$(gh issue view $ISSUE_NUM --json labels --jq '.labels[].name')
echo "$LABELS" | grep -q "waiting:answers" && echo "✅ Still waiting" || echo "❌ Incorrectly marked ready"

# Answer last question
gh issue comment $ISSUE_NUM --body "A3: Answer three"

sleep 60

# Verify now ready
LABELS=$(gh issue view $ISSUE_NUM --json labels --jq '.labels[].name')
echo "$LABELS" | grep -q "ready:work" && echo "✅ Now ready" || echo "❌ Not marked ready"

# Cleanup
gh issue close $ISSUE_NUM
```

#### Test 3.3: Edited Issue Body

**Scenario**: Maintainer adds/edits clarifying questions after issue created

```bash
# Create issue without questions
ISSUE_NUM=$(gh issue create \
  --title "Test: Edit Issue Body" \
  --body "## Description\nInitial description" | \
  grep -oP '#\K\d+')

sleep 60

# Verify initially ready
LABELS=$(gh issue view $ISSUE_NUM --json labels --jq '.labels[].name')
echo "$LABELS" | grep -q "ready:work" && echo "✅ Initially ready"

# Edit issue to add questions
NEW_BODY="## Description
Updated description

## Clarifying Questions
1. New question added?"

gh issue edit $ISSUE_NUM --body "$NEW_BODY"

sleep 60

# Verify now waiting
LABELS=$(gh issue view $ISSUE_NUM --json labels --jq '.labels[].name')
echo "$LABELS" | grep -q "waiting:answers" && echo "✅ Now waiting after edit" || echo "❌ Not marked waiting"

# Cleanup
gh issue close $ISSUE_NUM
```

#### Test 3.4: Closed Issue (No Spawn)

**Scenario**: Issue closed before agent spawned

```bash
# Create issue
BODY="## Clarifying Questions\n1. Question?"
ISSUE_NUM=$(gh issue create \
  --title "Test: Closed Issue" \
  --body "$BODY" | \
  grep -oP '#\K\d+')

sleep 30

# Close immediately
gh issue close $ISSUE_NUM

# Answer question (should not trigger spawn on closed issue)
gh issue comment $ISSUE_NUM --body "A1: Answer"

sleep 60

# Verify no spawn comment (issue was closed)
SPAWN=$(gh issue view $ISSUE_NUM --json comments --jq '.comments[] | select(.body | contains("ORCHESTRATOR-SPAWN-AGENT"))')
[ -z "$SPAWN" ] && echo "✅ Correctly not spawned on closed issue" || echo "❌ Spawned on closed issue"
```

### Phase 4: Performance Tests

#### Test 4.1: Measure Event Detection Latency

```bash
# Create issue and measure time to spawn comment
START=$(date +%s)
ISSUE_NUM=$(gh issue create \
  --title "Test: Latency Measurement" \
  --body "## Description\nLatency test" | \
  grep -oP '#\K\d+')

# Poll for spawn comment (max 120 seconds)
for i in {1..120}; do
  SPAWN=$(gh issue view $ISSUE_NUM --json comments --jq '.comments[] | select(.body | contains("ORCHESTRATOR-SPAWN-AGENT"))')
  if [ -n "$SPAWN" ]; then
    END=$(date +%s)
    LATENCY=$((END - START))
    echo "✅ Spawn comment detected in $LATENCY seconds"
    break
  fi
  sleep 1
done

# Target: <60 seconds
if [ $LATENCY -lt 60 ]; then
  echo "✅ PASS: Latency within target (<60s)"
else
  echo "❌ FAIL: Latency exceeds target ($LATENCY seconds)"
fi

# Cleanup
gh issue close $ISSUE_NUM
```

#### Test 4.2: Concurrent Event Handling

```bash
# Create 10 issues, answer questions on 5 simultaneously
echo "Creating test issues..."
for i in {1..10}; do
  BODY="## Clarifying Questions\n1. Question $i?"
  gh issue create \
    --title "Test: Concurrent $i" \
    --body "$BODY" &
done
wait

sleep 60

# Get first 5 issues
ISSUES=$(gh issue list --label "waiting:answers" --limit 5 --json number --jq '.[].number')

# Answer all 5 simultaneously
echo "Answering questions..."
for issue in $ISSUES; do
  gh issue comment $issue --body "A1: Concurrent answer" &
done
wait

sleep 90

# Verify all have spawn comments
SUCCESS=0
FAIL=0
for issue in $ISSUES; do
  SPAWN=$(gh issue view $issue --json comments --jq '.comments[] | select(.body | contains("ORCHESTRATOR-SPAWN-AGENT"))')
  if [ -n "$SPAWN" ]; then
    SUCCESS=$((SUCCESS + 1))
  else
    FAIL=$((FAIL + 1))
  fi
done

echo "✅ Success: $SUCCESS/5"
echo "❌ Failed: $FAIL/5"

# Cleanup
gh issue list --label "ready:work" --json number --jq '.[].number' | xargs -I {} gh issue close {}
gh issue list --label "waiting:answers" --json number --jq '.[].number' | xargs -I {} gh issue close {}
```

### Phase 5: Orchestrator Integration Tests

Test orchestrator detection of spawn trigger comments.

#### Test 5.1: Orchestrator Detects Spawn Trigger

```bash
# Create issue with spawn comment (manual simulation)
ISSUE_NUM=$(gh issue create \
  --title "Test: Orchestrator Detection" \
  --body "Test issue" | \
  grep -oP '#\K\d+')

gh issue comment $ISSUE_NUM --body \
"🤖 **ORCHESTRATOR-SPAWN-AGENT**

**Issue**: #${ISSUE_NUM}
**Issue ID**: TEST-001
**Type**: rust-pro
**Status**: ready
**Timestamp**: $(date -u +"%Y-%m-%dT%H:%M:%SZ")"

# Run orchestrator monitoring script
# (This would be done in Claude Code orchestrator session)
echo "Run orchestrator monitor to detect this spawn trigger"
echo "Issue #$ISSUE_NUM is ready for orchestrator"

# Manual verification:
# 1. Orchestrator should detect the spawn comment
# 2. Orchestrator should parse issue details
# 3. Orchestrator should spawn development agent
# 4. Agent should post acknowledgment comment

# Cleanup
gh issue close $ISSUE_NUM
```

## Validation Checklist

### Functional Requirements

- [ ] Issue opened without questions → Spawn comment posted immediately
- [ ] Issue opened with questions → Paused comment posted, no spawn
- [ ] Questions answered → Resumption comment + spawn comment posted
- [ ] Partial answers → Remains in waiting state
- [ ] PR merged → Issue closed, next issue assigned
- [ ] Multiple events → All handled without loss
- [ ] Closed issues → No spawn triggered

### Performance Requirements

- [ ] Spawn latency <60 seconds (average)
- [ ] Spawn latency <120 seconds (max)
- [ ] No workflow failures under load (10 concurrent events)
- [ ] No duplicate spawn comments
- [ ] All events processed (zero loss)

### Reliability Requirements

- [ ] Workflow runs complete successfully (>95%)
- [ ] Scripts handle missing data gracefully
- [ ] Invalid input doesn't crash workflows
- [ ] State transitions are idempotent
- [ ] Retries work correctly on transient failures

## Monitoring & Observability

### Workflow Run Dashboard

```bash
# View recent workflow runs
gh run list --limit 20

# Check for failures
gh run list --status failure

# View specific run details
gh run view <run-id>

# Download logs
gh run download <run-id>
```

### Issue State Audit

```bash
# Count issues by state
echo "Ready: $(gh issue list --label ready:work --json number --jq '. | length')"
echo "Waiting: $(gh issue list --label waiting:answers --json number --jq '. | length')"
echo "In Progress: $(gh issue list --label status:in-progress --json number --jq '. | length')"

# Find orphaned issues (ready but no spawn comment)
gh issue list --label "ready:work" --json number,comments | \
  jq -r '.[] | select(.comments | map(.body | contains("ORCHESTRATOR-SPAWN-AGENT")) | any | not) | .number'
```

### Performance Metrics

```bash
# Measure average workflow duration
gh run list --workflow=orchestrator-issue-events.yml --limit 50 --json databaseId,createdAt,updatedAt | \
  jq '[.[] | ((.updatedAt | fromdateiso8601) - (.createdAt | fromdateiso8601))] | add / length'

# Count events processed per day
gh run list --workflow=orchestrator-issue-events.yml --created "$(date -d '1 day ago' +%Y-%m-%d)" --json databaseId | \
  jq '. | length'
```

## Troubleshooting Guide

### Issue: Workflow not triggering

**Symptoms**: Issue created, no workflow run appears

**Diagnosis**:
```bash
# Check if workflows exist on default branch
gh api repos/raibid-labs/raibid-cli/contents/.github/workflows

# Check GitHub Actions enabled
gh api repos/raibid-labs/raibid-cli | jq .has_actions

# Check recent workflow runs
gh run list --limit 5
```

**Solutions**:
1. Ensure workflows committed to main branch
2. Verify GitHub Actions enabled in repo settings
3. Check workflow syntax: `gh workflow view <workflow-name>`

### Issue: Spawn comment not posted

**Symptoms**: Issue ready but no spawn comment

**Diagnosis**:
```bash
# Check workflow run logs
gh run view <run-id> --log

# Check script output
gh run view <run-id> --log | grep "check-issue-readiness"

# Check issue labels
gh issue view <issue-number> --json labels
```

**Solutions**:
1. Check script permissions (chmod +x)
2. Verify GitHub token has write permissions
3. Check for script errors in workflow logs
4. Manually run script locally to debug

### Issue: Duplicate spawn comments

**Symptoms**: Multiple spawn comments on same issue

**Diagnosis**:
```bash
# Count spawn comments on issue
gh issue view <issue-number> --json comments | \
  jq '[.comments[] | select(.body | contains("ORCHESTRATOR-SPAWN-AGENT"))] | length'
```

**Solutions**:
1. Add idempotency check in orchestrator (already spawned?)
2. Check for workflow re-runs
3. Add deduplication logic in spawn script

### Issue: Questions not detected as answered

**Symptoms**: Answers posted but issue still in waiting state

**Diagnosis**:
```bash
# Check answer format
gh issue view <issue-number> --json comments --jq '.comments[].body'

# Test question detection script locally
export ISSUE_NUMBER=<issue-number>
./.github/scripts/check-issue-readiness.sh
```

**Solutions**:
1. Verify answer format matches patterns (A1:, Answer 1:, etc.)
2. Check for question numbering mismatch
3. Update answer detection regex if needed

## Rollback Procedure

If event-driven system fails critically:

1. **Immediate**: Disable workflows
   ```bash
   # Rename workflows to disable
   for file in .github/workflows/orchestrator-*.yml; do
     git mv "$file" "${file}.disabled"
   done
   git commit -m "Emergency: Disable event-driven orchestration"
   git push
   ```

2. **Restore**: Re-enable polling orchestrator
   ```bash
   # Resume polling script (adjust path as needed)
   ./scripts/orchestrator_monitor.sh
   ```

3. **Investigate**: Review workflow logs, identify root cause

4. **Fix**: Correct issue, re-test, re-enable

## Success Criteria

System is considered validated when:

- ✅ All functional tests pass (100%)
- ✅ Performance tests meet targets (<60s latency)
- ✅ No test failures in 10 consecutive runs
- ✅ Zero duplicate spawns in concurrent tests
- ✅ Zero missed events in stress tests
- ✅ Orchestrator successfully spawns agents from triggers
- ✅ Full end-to-end workflow (issue → answers → spawn → PR → close → next) completes successfully

## Next Steps After Validation

1. **Document results**: Record test metrics, failure rates, latency measurements
2. **Update metrics**: Establish baseline performance data
3. **Monitor production**: Track metrics for 1 week
4. **Disable polling**: Remove old polling orchestrator
5. **Optimize**: Fine-tune based on production data

---

**Document Version**: 1.0
**Created**: 2025-10-29
**Status**: Ready for Testing
