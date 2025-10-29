# WS-01: Infrastructure Core

## Description

Establish the foundational k3s Kubernetes cluster on DGX Spark with networking and storage configuration. This is the blocking workstream that enables all subsequent work.

## Dependencies

**Blockers:** None - can start immediately

**Blocks:**
- WS-02: Data Services (requires k3s cluster)
- WS-03: GitOps & Orchestration (requires k3s cluster)

## Priority

**Critical** - Blocking workstream for entire project

## Estimated Duration

3-4 days

## Parallelization

Can run in parallel with:
- WS-04: API Services (development work)
- WS-05: Client TUI (development work)

## Agent Workflow

Follow this TDD-based workflow for each issue in this workstream:

### 1. Issue Selection
- Review all issues in this workstream (listed below)
- Select the next issue that is:
  - Not yet started (no branch exists)
  - Not blocked by dependencies
  - Highest priority among available issues
- Check parallelization notes to identify issues that can run concurrently

### 2. Branch Creation
```bash
# Checkout new branch named after the issue
git checkout -b <issue-id>-<brief-description>
# Example: git checkout -b infra-001-k3s-cluster-setup
```

### 3. Test-First Development (TDD)
**Write tests BEFORE implementation:**

For infrastructure work, create validation tests:
```bash
# Create test script in tests/ directory
# Example: tests/infra-001-k3s-validation.sh
```

**Test types for infrastructure:**
- Smoke tests (service running, endpoints reachable)
- Configuration validation (correct settings applied)
- Integration tests (component interactions)
- Health checks (pods ready, services responding)

**Example test structure:**
```bash
#!/bin/bash
# tests/infra-001-k3s-validation.sh

# Test 1: k3s service is running
systemctl is-active k3s || exit 1

# Test 2: kubectl commands work
kubectl get nodes || exit 1

# Test 3: Namespaces exist
kubectl get namespace ci || exit 1
kubectl get namespace infrastructure || exit 1

# Test 4: Node is Ready
kubectl get nodes | grep Ready || exit 1

echo "All k3s validation tests passed"
```

### 4. Initial Test Commit
```bash
# Add test files
git add tests/

# Commit tests (they should fail at this point - that's expected!)
git commit -m "test: add validation tests for <issue-id>

- Add smoke tests for <functionality>
- Add integration tests for <component>
- Tests currently failing (expected before implementation)

Relates to <issue-id>"

# Push to remote
git push -u origin <branch-name>
```

### 5. Implementation
Implement the functionality to make tests pass:
- Follow the task list in the issue description
- Reference documentation in `/docs/technology-research.md`
- Keep commits small and focused
- Run tests frequently during development

**For infrastructure work:**
- Create installation scripts in `scripts/`
- Create Kubernetes manifests in appropriate directories
- Document all configurations
- Create runbooks for manual procedures

### 6. Implementation Commits
```bash
# Make incremental commits as you implement
git add <files>
git commit -m "feat(<issue-id>): <what you implemented>

<detailed description of changes>

- Bullet point 1
- Bullet point 2

Relates to <issue-id>"

# Run tests after each significant change
./tests/<test-script>.sh

# Push regularly
git push
```

### 7. Final Validation
Before creating PR, ensure:
- [ ] All tests pass
- [ ] Configuration files validated (YAML syntax, etc.)
- [ ] Documentation updated (`README.md`, runbooks)
- [ ] No hardcoded secrets or credentials
- [ ] All manual steps documented
- [ ] Success criteria from issue met

### 8. Create Pull Request
```bash
# Ensure all changes are committed and pushed
git push

# Create PR via GitHub CLI or web interface
gh pr create --title "<issue-id>: <brief description>" \
  --body "## Summary
Implements <issue-id>

## Changes
- Change 1
- Change 2

## Testing
- [x] Validation tests pass
- [x] Manual testing completed
- [x] Documentation updated

## Checklist
- [x] Tests passing
- [x] Documentation updated
- [x] Issue comments added
- [x] No secrets committed

Closes #<issue-number>"
```

### 9. PR Acceptance Criteria
Your PR must meet ALL of these criteria:

- [ ] **Tests passing**: All validation tests execute successfully
- [ ] **Documentation updated**:
  - README.md reflects new functionality
  - Runbooks created for manual procedures
  - Configuration examples provided
- [ ] **Comments on related issues**:
  - Link PR to issue
  - Document any deviations from plan
  - Note any blockers or dependencies discovered
- [ ] **Code review**:
  - Self-review completed
  - No obvious issues or TODOs left
- [ ] **Success criteria met**: All success criteria from issue description satisfied

### 10. Continue to Next Issue
After PR is created and tests are passing:

- **Option A**: If you can build upon the current branch for the next issue:
  ```bash
  # Create new branch from current branch
  git checkout -b <next-issue-id>-<description>
  ```
  This is useful when issues are sequential (e.g., INFRA-002 builds on INFRA-001)

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

### 11. Edge Cases & Issue Management

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
3. Document edge cases in code comments
4. Update issue with findings

## Issues

### INFRA-001: k3s Cluster Setup
**Priority:** Critical | **Complexity:** Small | **Duration:** 1 day
- Install k3s on DGX Spark (ARM64)
- Configure kubeconfig
- Verify cluster health
- Set up namespaces (ci, infrastructure, monitoring)
- Configure resource reservations

### INFRA-002: Storage Configuration
**Priority:** High | **Complexity:** Small | **Duration:** 0.5 days
- Configure local-path storage provisioner
- Test PVC creation and mounting
- Verify storage performance

### INFRA-003: Network Configuration
**Priority:** High | **Complexity:** Small | **Duration:** 0.5 days
- Configure Flannel CNI
- Set up CoreDNS custom entries
- Test pod-to-pod networking
- Verify DNS resolution

### INFRA-004: Registry Integration
**Priority:** Medium | **Complexity:** Small | **Duration:** 0.5 days
- Configure k3s registry integration (`/etc/rancher/k3s/registries.yaml`)
- Set up registry mirror configuration
- Test registry connectivity (placeholder - will connect to Gitea later)

### INFRA-005: Resource Limits & Quotas
**Priority:** Medium | **Complexity:** Small | **Duration:** 0.5 days
- Define ResourceQuotas for namespaces
- Set LimitRanges for pods
- Document resource allocation strategy
- Reserve resources for system components (8 cores, 16GB)

### INFRA-006: Monitoring Setup (Optional)
**Priority:** Low | **Complexity:** Medium | **Duration:** 1 day
- Deploy metrics-server for resource metrics
- Set up basic Prometheus (optional for MVP)
- Configure kubectl top functionality

## Deliverables

- [ ] k3s cluster operational on DGX Spark
- [ ] Namespaces created and configured
- [ ] Storage provisioner functional
- [ ] Networking and DNS working
- [ ] Resource limits configured
- [ ] Documentation: k3s installation runbook

## Success Criteria

- `kubectl get nodes` shows Ready status
- `kubectl get namespaces` shows ci, infrastructure, monitoring
- PVC creation and pod mounting works
- Pod-to-pod communication functional
- DNS resolution for custom domains working

## Notes

- k3s chosen for lightweight footprint on DGX Spark
- ARM64 native support required
- Disable Traefik (using custom ingress strategy)
- All components verified ARM64-ready
