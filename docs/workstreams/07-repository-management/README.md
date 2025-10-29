# WS-07: Repository Management

## Description

Implement GitHub to Gitea repository mirroring with support for single repos, multiple repos, and organization-level sync with regex filtering. Enables automated code synchronization.

## Dependencies

**Blockers:**
- WS-02: Data Services (requires Gitea)

**Runtime Dependencies:**
- WS-04: API Services (webhook handler for instant sync)

**Blocks:**
- WS-08: Integration & Deployment (mirroring required for full workflow)

## Priority

**Medium** - Important for automation but not blocking core CI

## Estimated Duration

3-4 days

## Parallelization

Can start after Gitea is deployed (WS-02 partial completion).

Strategy design (REPO-001) can start immediately.

Can run in parallel with:
- WS-03: GitOps & Orchestration
- WS-04: API Services
- WS-05: Client TUI
- WS-06: CI Agents

Within this workstream:
- REPO-001 can start immediately
- REPO-002 after Gitea ready
- REPO-003 after REPO-002
- REPO-004 after REPO-002 (parallel with REPO-003)
- REPO-005 after REPO-003 and REPO-004
- REPO-006 can run in parallel with REPO-003, REPO-004, REPO-005

## Issues

### REPO-001: Mirroring Strategy Design
**Priority:** Medium | **Complexity:** Small | **Duration:** 0.5 days
- Define configuration schema (YAML)
- Choose implementation approach (Gitea built-in vs custom)
- Design sync frequency (webhook vs polling)
- Plan authentication (GitHub PAT, SSH keys)
- Design conflict resolution (GitHub as source of truth)
- Document mirroring architecture
- Create decision matrix

### REPO-002: Gitea Mirror Configuration
**Priority:** Medium | **Complexity:** Medium | **Duration:** 1 day
- Create Gitea API client script (Nushell or Rust)
- Implement mirror creation via API (`POST /api/v1/repos/migrate`)
- Configure mirror parameters (interval, authentication)
- Configure GitHub PAT for authentication
- Test single repo mirroring
- Verify sync on GitHub push
- Add error handling for failed syncs
- Document API usage

### REPO-003: Organization-Level Sync
**Priority:** Medium | **Complexity:** Large | **Duration:** 1.5 days
- Create Nushell script: `mirror-org.nu`
- Fetch GitHub org repositories via API
- Implement regex filtering (include/exclude patterns)
- Iterate and create mirrors via Gitea API
- Store mirror configuration in Git (declarative)
- Implement idempotency (skip existing)
- Add dry-run mode for testing
- Schedule periodic re-scan (cron)

### REPO-004: Webhook-Based Instant Sync
**Priority:** Medium | **Complexity:** Medium | **Duration:** 1 day
- Create webhook endpoint in Rust API: `POST /webhook/github-sync`
- Register webhook in GitHub repository settings
- Validate webhook signature (HMAC)
- Extract repository name from payload
- Trigger Gitea mirror sync via API
- Handle rate limits (GitHub, Gitea)
- Add logging and monitoring
- Test webhook delivery

### REPO-005: Mirror Monitoring
**Priority:** Low | **Complexity:** Small | **Duration:** 1 day
- Add Mirrors tab to TUI
- Display sync status (last sync, next sync, errors)
- Fetch mirror status via Gitea API
- Highlight stale mirrors (no sync in 24 hours)
- Add manual sync trigger from TUI
- Export Prometheus metrics
- Create alerting rules

### REPO-006: Multi-Repository Management
**Priority:** Medium | **Complexity:** Medium | **Duration:** 1 day
- Implement batch mirror creation from list
- Support YAML configuration file with repo list
- Add validation for mirror configurations
- Implement conflict detection
- Add mirror deletion/cleanup
- Create management scripts
- Document workflow

### REPO-007: Documentation & Testing
**Priority:** Medium | **Complexity:** Small | **Duration:** 0.5 days
- Create user guide for mirroring
- Document configuration examples
- Create troubleshooting guide
- Test with real GitHub repositories
- Test organization sync
- Test webhook delivery
- Document edge cases

## Deliverables

- [ ] Repository mirroring functional
- [ ] Single repo mirror configuration
- [ ] Multi-repo batch mirroring
- [ ] Org-level sync with regex filtering
- [ ] Webhook-based instant sync
- [ ] Mirror monitoring in TUI
- [ ] Comprehensive documentation

## Success Criteria

- Gitea mirrors GitHub repos successfully
- Pushes to GitHub trigger sync within 5 minutes (polling) or 30 seconds (webhook)
- Mirror status visible in Gitea UI
- Org-level script mirrors entire org with filtering
- New repos auto-detected and mirrored
- TUI shows mirror health
- Manual sync works on-demand
- Metrics available for alerting

## Notes

- Use Gitea's built-in mirroring for simplicity
- GitHub is source of truth (force push on conflict)
- Consider GitHub API rate limits (5000/hour)
- Nushell for scripting automation
- Store mirror configs in Git for version control
- Support private repositories with authentication
- Consider mirror cleanup for deleted repos
