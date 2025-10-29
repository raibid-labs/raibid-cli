# WS-02: Data Services

## Description

Deploy Gitea (Git service + OCI registry) and Redis Streams (job queue) as the core data services for the CI system.

## Dependencies

**Blockers:**
- WS-01: Infrastructure Core (requires k3s cluster)

**Blocks:**
- WS-03: GitOps & Orchestration (Flux requires Gitea)
- WS-06: CI Agents (requires Redis job queue)
- WS-07: Repository Management (requires Gitea)

## Priority

**Critical** - Core data layer for CI system

## Estimated Duration

3-4 days

## Parallelization

**Redis and Gitea can be deployed in parallel** after k3s is ready.

Within this workstream:
- DATA-001 (Gitea) ∥ DATA-004 (Redis)
- DATA-002 after DATA-001
- DATA-003 after DATA-001
- DATA-005 after DATA-004
- DATA-006 can run in parallel with DATA-003 and DATA-005

## Issues

### DATA-001: Gitea Deployment
**Priority:** Critical | **Complexity:** Medium | **Duration:** 1.5 days
- Deploy PostgreSQL StatefulSet for Gitea
- Create PVC for Gitea data (100GB)
- Deploy Gitea using Helm or manifests
- Configure ingress/service (3000 HTTP, 2222 SSH)
- Create admin user

### DATA-002: Gitea OCI Registry Configuration
**Priority:** Critical | **Complexity:** Small | **Duration:** 0.5 days
- Enable OCI registry in `app.ini`
- Configure registry storage
- Test container push/pull
- Configure registry mirror for Docker Hub

### DATA-003: Gitea Integration Testing
**Priority:** High | **Complexity:** Small | **Duration:** 0.5 days
- Test Git push/pull operations
- Verify SSH key authentication
- Test HTTPS access
- Verify OCI registry endpoint reachable
- Test webhook delivery

### DATA-004: Redis Streams Deployment
**Priority:** Critical | **Complexity:** Small | **Duration:** 1 day
- Deploy Redis using Bitnami Helm chart
- Configure AOF persistence
- Enable RDB snapshots
- Create PVC for Redis data (10GB)
- Expose Redis service (port 6379)

### DATA-005: Redis Job Queue Configuration
**Priority:** Critical | **Complexity:** Small | **Duration:** 0.5 days
- Create initial consumer group: `ci-jobs` stream
- Configure consumer group: `ci-workers`
- Test stream operations (XADD, XREADGROUP, XACK)
- Configure maxmemory policy: noeviction
- Verify persistence after pod restart

### DATA-006: Network & Service Discovery
**Priority:** Medium | **Complexity:** Small | **Duration:** 0.5 days
- Configure DNS entries: `gitea.dgx.local`, `redis.dgx.local`
- Test service discovery from other namespaces
- Verify cross-namespace communication
- Document connection strings

### DATA-007: Backup & Recovery Strategy
**Priority:** Low | **Complexity:** Medium | **Duration:** 1 day
- Design backup strategy for Gitea
- Design backup strategy for Redis
- Create backup scripts
- Test restore procedures
- Document recovery runbook

## Deliverables

- [ ] Gitea accessible with OCI registry enabled
- [ ] Redis Streams operational with persistence
- [ ] Service discovery configured
- [ ] Backup/recovery documentation
- [ ] Connection strings and credentials documented

## Success Criteria

- Gitea accessible via browser at configured URL
- Git push/pull operations work
- Docker image push to Gitea registry succeeds
- Redis pod running and healthy
- Stream creation/consumption functional
- Consumer group visible via `XINFO GROUPS ci-jobs`

## Notes

- Gitea serves dual purpose: Git hosting + OCI registry
- Redis Streams chosen over Redis Pub/Sub for durability
- Both services require persistent storage
- PostgreSQL for Gitea metadata (better than SQLite for production)
