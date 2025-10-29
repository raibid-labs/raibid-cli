# WS-03: GitOps & Orchestration

## Description

Implement GitOps-based cluster management with Flux CD and event-driven autoscaling with KEDA. Establishes the automation layer for CI agent lifecycle management.

## Dependencies

**Blockers:**
- WS-02: Data Services (requires Gitea for Flux, Redis for KEDA)

**Blocks:**
- WS-06: CI Agents (requires KEDA ScaledJob configuration)
- WS-08: Integration & Deployment (requires GitOps workflow)

## Priority

**Critical** - Automation backbone for CI system

## Estimated Duration

2-3 days

## Parallelization

**Sequential within workstream** (Flux → KEDA → ScaledJob)

Can run in parallel with:
- WS-04: API Services (continued development)
- WS-05: Client TUI (continued development)
- WS-07: Repository Management (mirroring strategy design)

## Issues

### GITOPS-001: Flux CD Bootstrap
**Priority:** Critical | **Complexity:** Medium | **Duration:** 1 day
- Install Flux CLI on DGX
- Generate Gitea personal access token
- Bootstrap Flux with Gitea as source
- Create repository structure: `clusters/dgx-spark/{infrastructure,apps}`
- Set up Kustomization hierarchy
- Verify Flux controllers running
- Test reconciliation

### GITOPS-002: Flux Repository Structure
**Priority:** High | **Complexity:** Small | **Duration:** 0.5 days
- Design directory layout for Flux manifests
- Create base and overlay structure
- Document GitOps workflow
- Set up RBAC for Flux
- Configure sync intervals

### GITOPS-003: KEDA Deployment via Flux
**Priority:** Critical | **Complexity:** Medium | **Duration:** 0.5 days
- Create HelmRepository for KEDA
- Create HelmRelease manifest
- Configure KEDA namespace and resource limits
- Commit to Git and verify Flux applies
- Verify KEDA pods and CRDs

### GITOPS-004: KEDA Redis Streams Scaler
**Priority:** Critical | **Complexity:** Medium | **Duration:** 1 day
- Create ScaledJob CRD manifest
- Configure Redis Streams trigger (stream: ci-jobs, group: ci-workers)
- Set scaling parameters (min: 0, max: 10, pending threshold: 1)
- Configure polling interval (10s)
- Set Job template (placeholder for testing)
- Test scaling: XADD message → pod creation

### GITOPS-005: Scaling Policies & Tuning
**Priority:** Medium | **Complexity:** Small | **Duration:** 0.5 days
- Define scaling policies (aggressive vs conservative)
- Tune polling intervals
- Configure cooldown periods
- Set max replicas based on DGX resources
- Test scaling behavior under load

### GITOPS-006: Secret Management
**Priority:** Medium | **Complexity:** Medium | **Duration:** 1 day
- Evaluate SOPS/Age for secret encryption
- Configure sealed secrets or SOPS
- Encrypt sensitive data (tokens, credentials)
- Test secret decryption in cluster
- Document secret management workflow

### GITOPS-007: GitOps Workflow Documentation
**Priority:** Medium | **Complexity:** Small | **Duration:** 0.5 days
- Document commit-to-cluster workflow
- Create developer guide for Flux usage
- Document troubleshooting procedures
- Create runbook for common tasks

## Deliverables

- [ ] Flux CD managing cluster state from Gitea
- [ ] KEDA autoscaling functional
- [ ] ScaledJob responds to Redis queue depth
- [ ] Secret management configured
- [ ] GitOps workflow documentation

## Success Criteria

- Flux controllers healthy (`flux check` passes)
- Git commits auto-applied to cluster
- `flux get all` shows synced resources
- ScaledJob resource created
- Adding messages to Redis spawns pods
- Pods terminate after processing
- Scaling to zero after queue empty

## Notes

- Flux chosen for Gitea integration over ArgoCD
- KEDA provides zero-to-N scaling capability
- ScaledJob creates Job resources (not Deployments)
- Redis Streams scaler polls queue depth
- Secret management is optional for MVP but recommended
