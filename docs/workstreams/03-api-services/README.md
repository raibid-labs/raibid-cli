# WS-04: API Services

## Description

Build the Rust-based API server that handles webhooks, job orchestration, and status tracking. Provides the backend services for TUI communication and CI automation.

## Dependencies

**Blockers:** None - development can start immediately

**Runtime Dependencies:**
- WS-02: Data Services (requires Redis and Gitea at deployment time)

**Blocks:**
- WS-08: Integration & Deployment (requires API for end-to-end flow)

## Priority

**Critical** - Core orchestration service

## Estimated Duration

4-6 days

## Parallelization

Can start immediately in parallel with:
- WS-01: Infrastructure Core
- WS-05: Client TUI
- WS-07: Repository Management (strategy design)

Within this workstream:
- API-001 (scaffolding) must complete first
- API-002, API-003, API-004 can run in parallel after API-001
- API-005 after API-002, API-003, API-004
- API-006 can run in parallel with API-005

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
# Example: git checkout -b api-002-webhook-handler
```

### 3. Test-First Development (TDD)
**Write tests BEFORE implementation:**

For Rust code, create unit and integration tests:

**Example test structure:**
```rust
// tests/webhook_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_gitea_webhook_accepts_valid_payload() {
        // Test implementation here
        // This test will fail until implementation is complete
    }

    #[tokio::test]
    async fn test_gitea_webhook_rejects_invalid_signature() {
        // Test implementation
    }

    #[tokio::test]
    async fn test_webhook_creates_redis_job() {
        // Test implementation
    }
}
```

**Test types for Rust API:**
- **Unit tests**: Test individual functions and modules
- **Integration tests**: Test API endpoints end-to-end
- **Mock tests**: Use mock Redis/Kubernetes clients
- **Property tests**: Use `proptest` for edge cases

### 4. Initial Test Commit
```bash
# Create test files
mkdir -p tests/
# Write your tests in tests/ directory or in module with #[cfg(test)]

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

**For Rust API development:**
```bash
# Run tests in watch mode during development
cargo watch -x test

# Run specific test
cargo test test_webhook_accepts_valid_payload

# Run tests with output
cargo test -- --nocapture
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
- [ ] Logging added for debugging
- [ ] API documentation updated (OpenAPI spec if applicable)
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

# Run security audit
cargo audit
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
\`\`\`

**Test Results:**
- [x] All unit tests passing
- [x] All integration tests passing
- [x] Clippy checks passing
- [x] Formatting checks passing

## Checklist
- [x] Tests passing
- [x] Documentation updated
- [x] Issue comments added
- [x] No secrets committed
- [x] Error handling added
- [x] Logging implemented

Closes #<issue-number>"
```

### 10. PR Acceptance Criteria
Your PR must meet ALL of these criteria:

- [ ] **Tests passing**: `cargo test --all-features` succeeds
- [ ] **No warnings**: `cargo clippy -- -D warnings` passes
- [ ] **Formatted**: `cargo fmt --check` passes
- [ ] **Documentation updated**:
  - Doc comments for public APIs
  - README.md updated if needed
  - API documentation (OpenAPI) updated
- [ ] **Comments on related issues**:
  - Link PR to issue
  - Document any deviations from plan
  - Note any blockers or dependencies discovered
- [ ] **Code quality**:
  - Error handling comprehensive
  - No unwrap() in production code (use proper error handling)
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
  This is useful when issues are sequential (e.g., API-003 builds on API-002)

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

### API-001: Project Scaffolding
**Priority:** Critical | **Complexity:** Small | **Duration:** 0.5 days
- Create Rust workspace: `raibid-api`
- Add dependencies (axum, tokio, redis, kube, serde, tracing)
- Set up module structure: api/, jobs/, webhook/, config/
- Configure cross-compilation for ARM64
- Create Dockerfile with multi-stage build
- Set up basic logging

### API-002: Webhook Handler - Gitea
**Priority:** High | **Complexity:** Medium | **Duration:** 1 day
- Create Axum route: `POST /webhook/gitea`
- Parse Gitea webhook payload (JSON)
- Validate webhook signature (HMAC)
- Extract repository, branch, commit SHA, author
- Generate unique job ID (UUID)
- Push job to Redis Streams
- Add error handling and logging
- Write unit tests

### API-003: Job Status Tracker
**Priority:** High | **Complexity:** Medium | **Duration:** 1.5 days
- Create Redis hash structure: `job:<job_id>`
- Implement status state machine (pending, running, success, failed)
- Update status on job state changes
- Implement TTL for completed jobs (24 hours)
- Create API endpoint: `GET /jobs/:id`
- Create list endpoint: `GET /jobs` with filtering
- Add pagination support

### API-004: Log Streaming (SSE)
**Priority:** Medium | **Complexity:** Medium | **Duration:** 1 day
- Implement Server-Sent Events for real-time logs
- Create endpoint: `GET /jobs/:id/logs`
- Stream logs from Redis
- Handle client disconnections
- Add reconnection support
- Test with multiple concurrent clients

### API-005: Kubernetes Job Creator (Optional)
**Priority:** Low | **Complexity:** Large | **Duration:** 2 days
- Initialize kube-rs client
- Create Job template in code
- Set resource limits (2 CPU, 4GB RAM)
- Configure volumes and environment variables
- Apply Job via Kubernetes API
- Watch Job status and update Redis
- Implement cleanup logic
- Add retry logic for failed Jobs

**Note:** May be deferred if KEDA ScaledJob handles Job creation adequately.

### API-006: Health & Metrics Endpoints
**Priority:** Medium | **Complexity:** Small | **Duration:** 0.5 days
- Create health check endpoint: `GET /health`
- Create readiness endpoint: `GET /ready`
- Implement Prometheus metrics (job counts, durations)
- Add request tracing
- Document API endpoints (OpenAPI spec)

### API-007: Configuration Management
**Priority:** Medium | **Complexity:** Small | **Duration:** 0.5 days
- Implement config loading (YAML/ENV)
- Add config validation
- Support environment variable overrides
- Document configuration options
- Create example configs

### API-008: API Deployment
**Priority:** High | **Complexity:** Medium | **Duration:** 1 day
- Build Docker image for ARM64
- Push to Gitea registry
- Create Kubernetes Deployment manifest
- Configure Service (ClusterIP)
- Set environment variables
- Add ConfigMap for configuration
- Commit to flux-system repo
- Test webhook endpoint accessibility

## Deliverables

- [ ] Rust API server deployed and operational
- [ ] Webhooks triggering job creation
- [ ] Job status tracking functional
- [ ] Log streaming via SSE
- [ ] API documentation (OpenAPI spec)
- [ ] Health and metrics endpoints

## Success Criteria

- API pod running in `ci` namespace
- Webhook endpoint reachable and processing requests
- Jobs appear in Redis stream on webhook
- Job status persists and updates correctly
- SSE streams logs in real-time
- Health checks passing
- API responds within 100ms for status queries

## Notes

- Axum chosen for async performance and ergonomics
- kube-rs for Kubernetes integration (optional for MVP)
- Redis for job queue and status storage
- Consider rate limiting for webhook endpoints
- OpenAPI spec enables TUI and external integrations
