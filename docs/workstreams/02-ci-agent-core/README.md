# WS-06: CI Agents

## Description

Implement the ephemeral CI agent containers that consume jobs from Redis, build Rust projects, run tests, and publish artifacts. This is the core build execution layer.

## Dependencies

**Blockers:**
- WS-02: Data Services (requires Redis for job queue)

**Runtime Dependencies:**
- WS-03: GitOps & Orchestration (ScaledJob configuration)
- WS-02: Data Services (requires Gitea for code and registry)

**Blocks:**
- WS-08: Integration & Deployment (requires agents for end-to-end flow)

## Priority

**Critical** - Core build execution component

## Estimated Duration

4-6 days

## Parallelization

Can start after Redis is deployed (WS-02 partial completion).

Can run in parallel with:
- WS-04: API Services (deployment)
- WS-05: Client TUI (continued development)
- WS-07: Repository Management

Within this workstream:
- AGENT-001 must complete first
- AGENT-002 after AGENT-001
- AGENT-003 after AGENT-002
- AGENT-004 can run in parallel with AGENT-003
- AGENT-005 after AGENT-003
- AGENT-006 after all above

## Issues

### AGENT-001: Container Base Image
**Priority:** Critical | **Complexity:** Medium | **Duration:** 1 day
- Create Dockerfile based on `rust:1.82-bookworm` (ARM64)
- Install system dependencies (git, ssh, ca-certificates)
- Configure Rust toolchain (stable, ARM64 targets)
- Add Docker CLI for image building
- Install cargo tools (cargo-nextest, cargo-audit, cargo-deny)
- Configure credential helpers for Gitea
- Add healthcheck script
- Optimize image size with multi-stage build
- Test build on DGX Spark

### AGENT-002: Job Consumer Implementation
**Priority:** Critical | **Complexity:** Medium | **Duration:** 1.5 days
- Connect to Redis via `redis::Client`
- Join consumer group (ci-jobs / ci-workers)
- Implement consume loop with XREADGROUP
- Parse job message (repo, branch, commit, job_id)
- Update job status in Redis hash (pending → running)
- Clone repository via HTTPS with credentials
- Acknowledge message on success (XACK)
- Handle errors and dead-letter queue
- Implement graceful shutdown (SIGTERM)

### AGENT-003: Rust Build Pipeline
**Priority:** High | **Complexity:** Large | **Duration:** 2 days
- Run `cargo check` for validation
- Execute `cargo build --release` with caching
- Run tests with `cargo nextest run` or `cargo test`
- Capture test output and store in Redis
- Run linting with `cargo clippy`
- Run security audit with `cargo audit`
- Build Docker image if Dockerfile present
- Push image to Gitea registry
- Update job status (success/failed)
- Store build artifacts metadata
- Implement build timeout (30 min default)

### AGENT-004: Build Caching Strategy
**Priority:** High | **Complexity:** Medium | **Duration:** 1.5 days
- Mount persistent volume for Cargo cache
- Configure Docker BuildKit cache backend
- Use per-architecture cache refs
- Enable `mode=max` for full layer caching
- Implement cache pruning (7-day retention)
- Monitor cache hit rate via metrics
- Configure sccache (optional)
- Test cache effectiveness (measure build time)

### AGENT-005: Error Handling & Observability
**Priority:** Medium | **Complexity:** Medium | **Duration:** 1 day
- Implement comprehensive error handling
- Add structured logging (JSON format)
- Emit metrics (build duration, cache hits, failures)
- Implement retry logic for transient failures
- Add timeout handling
- Log streaming to Redis
- Create troubleshooting guide

### AGENT-006: Agent Deployment & Integration
**Priority:** High | **Complexity:** Medium | **Duration:** 1 day
- Build final agent image
- Push to Gitea registry
- Update KEDA ScaledJob to use agent image
- Configure resource limits (2 CPU, 4GB RAM)
- Mount Docker socket or use Docker-in-Docker
- Set environment variables (REDIS_URL, GITEA_URL, tokens)
- Add PVC for build cache
- Commit to flux-system repo
- Test end-to-end: push code → webhook → build → publish

### AGENT-007: Performance Optimization
**Priority:** Low | **Complexity:** Medium | **Duration:** 1.5 days
- Profile build performance
- Optimize Docker layer caching
- Tune Cargo cache configuration
- Implement parallel test execution
- Optimize image pull times
- Measure and document performance metrics
- Create performance tuning guide

## Deliverables

- [ ] CI agent container image functional
- [ ] Job consumer operational
- [ ] Rust build pipeline complete
- [ ] Build caching implemented
- [ ] Error handling and logging comprehensive
- [ ] End-to-end CI pipeline working
- [ ] Performance metrics documented

## Success Criteria

- Agent pods spawn on job creation
- Jobs consumed from Redis successfully
- Repositories cloned without errors
- Rust builds complete with tests
- Docker images pushed to Gitea registry
- Build logs available via Redis/API
- Agents scale to zero when idle
- Cache hit rate > 70%
- Build time reduced by 2-5x with warm cache
- Agent cold start < 60 seconds

## Notes

- Focus on Rust builds for MVP (other languages post-MVP)
- Docker-in-Docker vs Docker socket mount TBD (benchmark both)
- Cache storage limit: 50GB per agent
- Build timeout default: 30 minutes
- Consider sccache for distributed compilation (optional)
- Test with real Rust projects early
