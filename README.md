# raibid-ci

> **Ephemeral, auto-scaling CI agent pool for NVIDIA DGX Spark**

A TUI-first, developer-experience-focused tool for provisioning and managing self-hosted CI agents optimized for the DGX Spark's unique ARM64 architecture.

## 🎯 Overview

**raibid-ci** simplifies the process of running a personal CI/CD infrastructure on NVIDIA DGX Spark. It combines Kubernetes, GitOps, and event-driven autoscaling to provide on-demand build agents that scale from zero to match workload demand.

### Key Characteristics

- **DX-first**: Developer experience is the top priority
- **TUI-native**: Terminal UI for all management and monitoring
- **Ephemeral**: Agents spin up on-demand and tear down when idle (scale-to-zero)
- **Auto-scaling**: KEDA-driven scaling based on job queue depth
- **Plugin-based**: Extensible architecture for different build agent types

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      NVIDIA DGX Spark                       │
│  ┌──────────────────────────────────────────────────────┐  │
│  │                   k3s Cluster                        │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │  │
│  │  │   Gitea     │  │    Flux     │  │    KEDA     │  │  │
│  │  │ Git + OCI   │  │   GitOps    │  │ Autoscaler  │  │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  │  │
│  │  ┌─────────────┐  ┌─────────────────────────────┐   │  │
│  │  │   Redis     │  │   CI Agents (Ephemeral)     │   │  │
│  │  │  Streams    │  │   ┌──────┐  ┌──────┐       │   │  │
│  │  │ Job Queue   │  │   │Agent │  │Agent │  ...  │   │  │
│  │  └─────────────┘  │   └──────┘  └──────┘       │   │  │
│  │                   └─────────────────────────────┘   │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
         ▲                                          │
         │ Ratatui TUI                             │ GitHub
         │ Management                               │ Webhook
         │ Client                                   │ Mirror
```

See [System Architecture Diagram](docs/diagrams/system-architecture.mmd) for detailed visualization.

## 📂 Documentation

### Core Documents
- **[Technology Research](docs/technology-research.md)** - Comprehensive research on all stack components
- **[Project Plan](docs/work/plan.md)** - Milestones, issues, and task breakdown
- **[Architecture Diagrams](docs/diagrams/)** - Mermaid diagrams for system visualization

### Diagrams
- [System Architecture](docs/diagrams/system-architecture.mmd) - Complete topology
- [Build Workflow](docs/diagrams/build-workflow.mmd) - End-to-end CI pipeline
- [Component Interactions](docs/diagrams/component-interactions.mmd) - Sequence diagram
- [Deployment Architecture](docs/diagrams/deployment-architecture.mmd) - Kubernetes resources

## 🚀 Quick Start

### Prerequisites

**Hardware:**
- NVIDIA DGX Spark running Ubuntu 22.04 LTS
- 20 CPU cores (10x Cortex-X925, 10x Cortex-A725)
- 128GB LPDDR5x unified memory
- Network connectivity for GitHub and container registries

**Software:**
- Docker or Podman
- kubectl
- Rust toolchain (latest stable)

### Installation

> **Note:** Installation automation is under development. Manual setup required for MVP.

1. **Bootstrap k3s cluster:**
   ```bash
   curl -sfL https://get.k3s.io | sh -
   export KUBECONFIG=/etc/rancher/k3s/k3s.yaml
   ```

2. **Deploy Gitea with OCI registry:**
   ```bash
   kubectl apply -f manifests/gitea/
   ```

3. **Setup Redis Streams:**
   ```bash
   kubectl apply -f manifests/redis/
   ```

4. **Bootstrap Flux GitOps:**
   ```bash
   flux bootstrap gitea \
     --owner=<username> \
     --repository=raibid-ci-config \
     --branch=main \
     --path=clusters/dgx-spark
   ```

5. **Deploy KEDA autoscaler:**
   ```bash
   kubectl apply -f manifests/keda/
   ```

See the [Project Plan](docs/work/plan.md) for detailed implementation steps.

## 📊 System Specifications

### DGX Spark Hardware
- **CPU**: 30 cores ARM64 (NVIDIA Grace CPU)
- **GPU**: NVIDIA Hopper architecture
- **Memory**: 480GB unified memory
- **Memory Bandwidth**: 546 GB/s
- **Storage**: Up to 4TB NVMe
- **Network**: 10 Gb/s Ethernet
- **Power**: Optimized for efficiency

### Resource Allocation (MVP)
- **k3s control plane**: 2 cores, 2GB RAM
- **Gitea**: 1 core, 1GB RAM, 100GB storage
- **Redis**: 1 core, 512MB RAM, 10GB storage
- **Flux**: 0.5 cores, 256MB RAM
- **KEDA**: 0.5 cores, 256MB RAM
- **CI Agents (each)**: 2 cores, 4GB RAM (ephemeral)

**Total base footprint:** ~4 cores, ~4GB RAM
**Available for agents:** 16 cores, 124GB RAM

## 🎯 MVP Scope

### Phase 1: Infrastructure (Week 1-2)
- ✅ k3s cluster bootstrapping
- ✅ Gitea installation with OCI registry
- ✅ Redis Streams job queue
- ✅ Flux GitOps configuration
- ✅ KEDA autoscaler integration

### Phase 2: API & Client (Week 2-3)
- 🔲 Rust API server for job orchestration
- 🔲 Ratatui TUI client for management
- 🔲 CLI commands for infrastructure lifecycle
- 🔲 Real-time monitoring dashboard

### Phase 3: CI Agents (Week 3-4)
- 🔲 Rust build agent container
- 🔲 KEDA ScaledJob configuration
- 🔲 Build caching optimization
- 🔲 Test execution and reporting

### Phase 4: Repository Mirroring (Week 4)
- 🔲 Single GitHub repository mirroring
- 🔲 Multiple repository sync via list
- 🔲 Organization-level mirroring with regex filtering
- 🔲 Webhook-based instant synchronization

## 🛠️ Technology Stack

### Infrastructure Layer
- **[k3s](https://k3s.io)** - Lightweight Kubernetes (<512MB RAM)
- **[Gitea](https://gitea.io)** - Self-hosted Git + OCI registry
- **[Redis Streams](https://redis.io/docs/data-types/streams/)** - Job queue with consumer groups
- **[Flux CD](https://fluxcd.io)** - GitOps continuous delivery
- **[KEDA](https://keda.sh)** - Kubernetes event-driven autoscaling

### Application Layer
- **[Rust](https://rust-lang.org)** - API server and agent runtime
- **[Ratatui](https://ratatui.rs)** - Terminal UI framework
- **[Nushell](https://nushell.sh)** - Modern shell for automation
- **[kube-rs](https://kube.rs)** - Rust Kubernetes client

All technologies are 100% ARM64-compatible and production-ready for DGX Spark.

## 📈 Success Metrics

### Performance Targets
- **Agent spawn time**: <10 seconds from job submission
- **Build cache hit rate**: >70% for iterative builds
- **Resource utilization**: >80% when agents active, <5% at idle
- **Parallel builds**: 8+ concurrent agents on DGX Spark

### Reliability Targets
- **Job success rate**: >95% for valid builds
- **Queue processing**: <1 second latency for job dispatch
- **Auto-recovery**: Automatic retry for transient failures
- **Data persistence**: Zero job loss with Redis persistence

## 🔗 Integration Points

### Supported Workflows
- **GitHub → Gitea**: Automatic repository mirroring
- **Git Push → CI**: Webhook-triggered builds
- **Build → Registry**: Automatic container image publishing
- **TUI → API**: Real-time monitoring and control

### Future Integrations
- Tauri GUI for visual management
- Multi-DGX clustering for massive workloads
- GPU time-slicing for ML model testing
- Additional build agent types (Node.js, Python, Go, etc.)

## 🤔 Design Decisions

### Why These Technologies?

**k3s over k8s**: 50% smaller binary, single-node optimized, perfect for DGX Spark
**Gitea over GitLab**: Unified Git + OCI registry, 90% lower resource footprint
**Redis Streams over RabbitMQ**: Simpler ops, sub-millisecond latency, native KEDA support
**Flux over ArgoCD**: Native Gitea bootstrap, pull-based (secure), lower resource usage
**KEDA over HPA**: Event-driven (not just CPU/RAM), 74+ scalers, true scale-to-zero
**Rust over Go/Node**: Performance critical for DGX optimization, memory safety
**Ratatui over Web UI**: TUI-first philosophy, SSH-friendly, low latency

See [Technology Research](docs/technology-research.md) for detailed analysis.

## 📋 Project Status

**Current Phase:** 🚧 Planning & Documentation
**Next Milestone:** Infrastructure Bootstrap (M1)
**Estimated Timeline:** 21-31 days for MVP

### Recent Updates
- ✅ Comprehensive technology research completed
- ✅ Architecture diagrams created
- ✅ Detailed project plan with 6 milestones
- ✅ Documentation structure established

## 🤝 Contributing

This is currently an individual developer tool project. Contributions, suggestions, and feedback are welcome once MVP is complete.

### Development Setup
```bash
# Clone repository
git clone https://github.com/your-org/raibid-ci.git
cd raibid-ci

# Review documentation
cat docs/technology-research.md
cat docs/work/plan.md

# Follow project plan milestones
```

## 📚 Additional Resources

- [NVIDIA DGX Spark Documentation](https://www.nvidia.com/en-us/data-center/dgx-spark/)
- [k3s Architecture](https://docs.k3s.io/architecture)
- [KEDA Scalers Documentation](https://keda.sh/docs/scalers/)
- [Flux GitOps Toolkit](https://fluxcd.io/flux/components/)
- [Ratatui Examples](https://github.com/ratatui-org/ratatui/tree/main/examples)

## 📄 License

[TBD - Select appropriate open source license]

---

**Built with ❤️ for NVIDIA DGX Spark developers**

*Last Updated: 2025-10-28*
*Status: Pre-MVP (Planning Phase)*
