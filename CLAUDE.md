# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**raibid-ci** is a DGX Spark Personal CI Agent Pool - an ephemeral, auto-scaling build system for cross-platform native compilation on NVIDIA DGX Spark. This is a TUI-first, developer-experience-focused tool for provisioning and managing self-hosted CI agents.

### Target Hardware
- **NVIDIA DGX Spark** running Ubuntu 22.04 LTS
- **CPU**: 20 cores (10x Cortex-X925, 10x Cortex-A725)
- **Memory**: 128GB LPDDR5x unified memory
- **Memory Bandwidth**: 273 GB/s
- **Storage**: Up to 4TB NVMe
- **Network**: 200 Gb/s ConnectX-7

## Technology Stack

### Core Infrastructure
- **k3s**: Lightweight Kubernetes distribution for DGX Spark
- **Gitea**: Self-hosted Git service with OCI registry
- **Flux**: GitOps continuous delivery
- **KEDA**: Kubernetes-based event-driven autoscaling
- **Redis Streams**: Job queue management

### Application Layer
- **Rust**: Primary language for API server and CLI/TUI client
- **Ratatui**: Terminal UI framework for management interface
- **Nushell**: Scripting and automation

## Architecture Characteristics

- **DX-first**: Developer experience is the top priority
- **TUI-native**: Terminal UI for all management and monitoring
- **Ephemeral**: Agents spin up on-demand and tear down when idle
- **Auto-scaling**: KEDA-driven scaling based on job queue depth
- **Plugin-based**: Extensible architecture for different build agent types

## MVP Scope

### Infrastructure Setup
1. k3s cluster bootstrapping on DGX Spark
2. Gitea installation with OCI registry
3. Redis Streams for job queueing
4. Flux GitOps configuration for deployments from Gitea repo
5. KEDA autoscaler integration

### API & Client
- Server-side Rust API for job dispatching and TUI communication
- Client-side Rust CLI tool using Ratatui for management, monitoring, and control
- CLI handles infrastructure setup, configuration, and teardown

### CI Agents
- MVP focuses on a single Rust agent for building and testing Rust projects
- Emphasis on scaling, scheduling, monitoring, and caching

### Repository Mirroring
- Mirror single GitHub repository to Gitea
- Mirror multiple GitHub repositories via list
- Mirror GitHub organization repositories with regex filtering
- Auto-sync on GitHub push (GitHub is source of truth)

## Documentation Standards

### File Organization
- **`./docs/`**: All research, notes, diagrams, and documentation
- **`./docs/work/`**: Milestones, issues, and tasks (markdown files formatted for GitHub issues)
- **`./docs/diagrams/`**: Mermaid diagrams for architecture visualization

### Style Guidelines
- Use terse language and bullet points
- Create Mermaid diagrams for complex concepts
- Include internal and external links/references
- Keep content concise and scannable
- Markdown files should be GitHub-ready (especially issue descriptions)

## Development Workflow

### Current Phase
The project is in **Planning / MVP Development** phase. The immediate focus is on:

1. Research and knowledge gathering for required technologies
2. Creating comprehensive project plans and documentation
3. Architecture design and specification
4. No implementation/coding yet - documentation and planning first

### Working with This Codebase
- All architectural decisions should be documented in `./docs/`
- Use Mermaid diagrams to visualize complex systems and workflows
- When creating issues/tasks, format them as markdown in `./docs/work/` for eventual GitHub submission
- Consider the DGX Spark hardware constraints (20 cores, 128GB RAM, resource reservation needs)

## Design Principles

1. **Ephemeral by Default**: Agents should be stateless and disposable
2. **Auto-scaling First**: KEDA drives all scaling decisions based on job queue
3. **GitOps Everything**: Flux manages all deployments from Gitea
4. **TUI for Control**: All management through terminal interface
5. **Cache Aggressively**: Optimize for build cache hit rates
6. **Rust for Performance**: Critical path uses Rust for speed and safety

## Future Considerations

- Tauri GUI for visual management (beyond TUI)
- Multi-DGX clustering for massive workloads
- GPU time-slicing for ML model testing in CI
- Additional build agent types (beyond Rust)
