# DGX Spark Personal CI Agent Pool

> **Ephemeral, fan-out build system for cross-platform native compilation on NVIDIA DGX Spark**

## 🎯 Project Overview

This project implements a **personal CI agent pool** optimized for the DGX Spark's unique hardware capabilities. It provides ephemeral, auto-scaling build agents with TUI-first management.

### Key Features

## 🏗️ Architecture

## 📂 Documentation Structure

### Core Architecture Documents


### Evaluation & Decision Documents


### Roadmap & Planning


## 🚀 Quick Start

### Prerequisites

- NVIDIA DGX Spark with Ubuntu 22.04 LTS
- 20 CPU cores available (10 reserved for system)
- 112GB RAM available (16GB reserved for system)
- Network connectivity for GitHub and container registries

### Installation


## 📊 System Specifications

### DGX Spark Hardware

- **CPU**: 20 cores (10x Cortex-X925, 10x Cortex-A725)
- **Memory**: 128GB LPDDR5x unified memory
- **Memory Bandwidth**: 273 GB/s
- **Storage**: Up to 4TB NVMe
- **Network**: 200 Gb/s ConnectX-7
- **Power**: 240W

## 🎯 MVP Goals

### Infrastructure & Server Side Setup
- k3s cluster bootstrapping on DGX spark
- gitea installation with OCI registry
- redis streams for job queueing
- flux gitops configuration for deployments from gitea repo
- keda autoscaler

### API
- server side rust api for job dispatching and communication via TUI
- ⏳ client side rust cli tool using Ratatui TUI to enhance management, monitoring, and control.
- client side rust cli also handles infrastructure setup, configuration, and teardown

### ci agents
- mvp will just have a rust agent for building and testing rust projects
- mvp will focus on scaling, scheduling, monitoring, and caching for this agent

### Repository Mirroring
- infra gets initialized and pre configured for either:
- mirroring a single github repository to gitea
- mirroring multiple github repositories to gitea via a list
- mirroring 1-n github organization's repositories to gitea using regex filtering
- repositories configured for mirroring should automatically sync on a push to github. github is the source of truth.

### Optimization & Polish
- ⏳ Cache hit rate optimization
- ⏳ Build parallelization tuning
- ⏳ Error handling and retry logic
- ⏳ Documentation and runbooks

## 🛠️ Technology Stack

### Infrastructure

### CI/CD

### Monitoring & Management

### Build Caching

## 📋 Design Decisions

### ✅ Decisions Made


### 🤔 Future Considerations

- Tauri GUI for visual management
- Multi-DGX clustering (405B parameter models)
- GPU time-slicing for ML model testing in CI

## 🔗 Integration Points


## 📈 Success Metrics


## 🤝 Contributing


## 📚 Additional Resources


## 📄 License

See main repository LICENSE file for details.

---

**Status**: 🚧 In Planning / MVP Development

**Last Updated**:

**Next Milestone**:
