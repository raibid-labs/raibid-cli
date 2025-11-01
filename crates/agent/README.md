# raibid-ci Agent Container

Ephemeral CI agent container image for executing Rust builds on NVIDIA DGX Spark.

## Overview

This container provides a complete Rust build environment optimized for ARM64 architecture with:

- **Rust toolchain** (stable, with rustfmt and clippy)
- **Cargo tools** (nextest, audit, deny)
- **Docker CLI** for container image builds
- **Git** with LFS support
- **Multi-stage build** for minimal image size
- **BuildKit caching** for fast rebuilds
- **Health checks** for container orchestration

## Image Details

### Base Image
- **Base**: `rust:1.82-bookworm`
- **Architecture**: ARM64 (aarch64)
- **OS**: Debian 12 (Bookworm)

### Installed Tools

#### System Dependencies
- Git 2.39+ with Git LFS
- OpenSSH client
- Docker CLI (latest stable)
- Build essentials (gcc, make, pkg-config)
- SSL/TLS libraries (OpenSSL)

#### Rust Toolchain
- Rust 1.82 (stable)
- Cargo
- Rustfmt
- Clippy

#### Cargo Tools
- **cargo-nextest** (0.9.72): Advanced test runner with better reporting
- **cargo-audit** (0.20.0): Security vulnerability scanner
- **cargo-deny** (0.14.24): License and dependency checker

### Image Size

**Target**: < 1.5 GB (1536 MB)

The multi-stage build optimizes image size by:
1. Building cargo tools in a separate stage
2. Copying only the compiled binaries to the final image
3. Removing package manager caches
4. Using minimal base images

## Building the Image

### Quick Start

```bash
# Basic build (local)
./build.sh

# Build and push to Gitea registry
./build.sh --registry gitea.local:3000/raibid --push

# Build with cache optimization
./build.sh \
  --cache-from gitea.local:3000/raibid/cache:agent \
  --cache-to gitea.local:3000/raibid/cache:agent \
  --push
```

### Build Script Options

```bash
./build.sh [OPTIONS]

OPTIONS:
  -n, --name NAME         Image name (default: raibid-ci-agent)
  -t, --tag TAG           Image tag (default: latest)
  -r, --registry URL      Registry URL (default: localhost:5000)
  -p, --platform PLATFORM Target platform (default: linux/arm64)
  --cache-from REF        Cache source reference
  --cache-to REF          Cache destination reference
  --build-arg ARG         Additional build arguments
  --push                  Push to registry after build
  -h, --help              Show this help message
```

### Environment Variables

```bash
IMAGE_NAME=my-agent      # Override image name
IMAGE_TAG=v1.0.0         # Override image tag
REGISTRY=my-registry     # Override registry URL
PLATFORM=linux/amd64     # Override platform (for testing)
PUSH=true                # Auto-push after build
```

## Running the Container

### Interactive Shell

```bash
docker run -it --rm \
  --platform linux/arm64 \
  localhost:5000/raibid-ci-agent:latest \
  /bin/bash
```

### Health Check

```bash
docker run --rm \
  --platform linux/arm64 \
  localhost:5000/raibid-ci-agent:latest \
  /usr/local/bin/healthcheck.sh
```

### With Docker Socket (for building images)

```bash
docker run -it --rm \
  --platform linux/arm64 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  localhost:5000/raibid-ci-agent:latest \
  /bin/bash
```

### With Build Cache Volume

```bash
docker run -it --rm \
  --platform linux/arm64 \
  -v cargo-cache:/home/agent/.cargo \
  -v build-cache:/workspace/target \
  localhost:5000/raibid-ci-agent:latest \
  /bin/bash
```

## BuildKit Caching

The image uses Docker BuildKit for optimal caching:

### Local Cache

```bash
# Local filesystem cache
docker buildx build \
  --cache-from type=local,src=/tmp/buildkit-cache \
  --cache-to type=local,dest=/tmp/buildkit-cache,mode=max \
  .
```

### Registry Cache

```bash
# Remote registry cache (recommended for CI)
docker buildx build \
  --cache-from type=registry,ref=gitea.local:3000/raibid/cache:agent \
  --cache-to type=registry,ref=gitea.local:3000/raibid/cache:agent,mode=max \
  .
```

### Cache Modes

- **mode=min**: Cache only final image layers (default)
- **mode=max**: Cache all intermediate layers (recommended)

## Health Check

The container includes a comprehensive health check script that verifies:

- ✓ Core system tools (git, ssh, docker)
- ✓ Rust toolchain (rustc, cargo, rustfmt, clippy)
- ✓ Cargo tools (nextest, audit, deny)
- ✓ Version information
- ✓ Git configuration
- ✓ Filesystem permissions
- ✓ Environment variables

Health check is automatically run by container orchestration systems.

## Container Configuration

### User and Permissions

- **User**: `agent` (UID: 1000, GID: 1000)
- **Home**: `/home/agent`
- **Workspace**: `/workspace`
- **Cargo Home**: `/home/agent/.cargo`

### Environment Variables

```bash
RUST_BACKTRACE=1                    # Enable Rust backtraces
CARGO_HOME=/home/agent/.cargo       # Cargo cache directory
CARGO_TARGET_DIR=/workspace/target  # Build output directory
CARGO_INCREMENTAL=1                 # Enable incremental compilation
```

### Volumes (Recommended)

```yaml
volumes:
  - cargo-cache:/home/agent/.cargo      # Cargo registry and build cache
  - build-cache:/workspace/target       # Compiled artifacts
  - /var/run/docker.sock:/var/run/docker.sock  # Docker socket (if needed)
```

## Kubernetes Deployment

### Example Pod Spec

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: raibid-agent
spec:
  containers:
  - name: agent
    image: gitea.local:3000/raibid/raibid-ci-agent:latest
    imagePullPolicy: Always
    resources:
      requests:
        memory: "2Gi"
        cpu: "1000m"
      limits:
        memory: "4Gi"
        cpu: "2000m"
    env:
    - name: REDIS_URL
      value: "redis://redis-master:6379"
    - name: GITEA_URL
      value: "https://gitea.local:3000"
    volumeMounts:
    - name: cargo-cache
      mountPath: /home/agent/.cargo
    - name: workspace
      mountPath: /workspace
  volumes:
  - name: cargo-cache
    persistentVolumeClaim:
      claimName: agent-cargo-cache
  - name: workspace
    emptyDir: {}
```

## Development

### Testing Locally

```bash
# 1. Build the image
./build.sh

# 2. Run health check
docker run --rm localhost:5000/raibid-ci-agent:latest /usr/local/bin/healthcheck.sh

# 3. Test Rust build
docker run --rm -v $(pwd)/examples:/workspace localhost:5000/raibid-ci-agent:latest \
  bash -c "cd /workspace && cargo build --release"

# 4. Test cargo-nextest
docker run --rm -v $(pwd)/examples:/workspace localhost:5000/raibid-ci-agent:latest \
  bash -c "cd /workspace && cargo nextest run"
```

### Debugging

```bash
# Interactive shell with all mounts
docker run -it --rm \
  -v $(pwd):/workspace \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v cargo-cache:/home/agent/.cargo \
  --platform linux/arm64 \
  localhost:5000/raibid-ci-agent:latest \
  /bin/bash

# Inside container:
rustc --version
cargo --version
docker --version
git --version
cargo-nextest --version
cargo-audit --version
cargo-deny --version
```

## CI/CD Integration

### GitHub Actions

```yaml
- name: Build Agent Image
  run: |
    cd crates/agent
    ./build.sh \
      --registry ghcr.io/raibid-labs \
      --tag ${{ github.sha }} \
      --push
```

### GitLab CI

```yaml
build-agent:
  script:
    - cd crates/agent
    - ./build.sh --registry $CI_REGISTRY_IMAGE --tag $CI_COMMIT_SHA --push
```

### Gitea Actions

```yaml
- name: Build and Push
  run: |
    cd crates/agent
    ./build.sh \
      --registry gitea.local:3000/raibid \
      --cache-from gitea.local:3000/raibid/cache:agent \
      --cache-to gitea.local:3000/raibid/cache:agent \
      --tag latest \
      --push
```

## Troubleshooting

### Image Too Large

If the image exceeds 1.5 GB:

1. Check for unnecessary dependencies in Dockerfile
2. Verify multi-stage build is working correctly
3. Ensure apt caches are being cleaned
4. Consider using `--squash` flag (experimental)

### BuildKit Not Available

```bash
# Enable BuildKit
export DOCKER_BUILDKIT=1

# Or use buildx
docker buildx create --use
```

### Permission Errors

The container runs as non-root user `agent` (UID 1000). If you encounter permission errors:

```bash
# Run as root for debugging (not recommended for production)
docker run --user root ...

# Fix volume permissions
sudo chown -R 1000:1000 /path/to/volume
```

### Health Check Failing

```bash
# Run health check manually for detailed output
docker run --rm localhost:5000/raibid-ci-agent:latest /usr/local/bin/healthcheck.sh

# Check logs
docker logs <container-id>
```

## Security Considerations

- Container runs as non-root user (`agent`)
- No sudo or privilege escalation available
- Docker socket access requires explicit mounting
- Secrets should be passed via environment variables or mounted secrets
- Use registry authentication for private registries

## Performance Optimization

### Build Performance

1. **Use BuildKit caching** with `mode=max`
2. **Mount cargo cache** to persistent volume
3. **Enable incremental compilation** (default)
4. **Use cargo-nextest** for faster test execution

### Runtime Performance

1. **Set appropriate resource limits** (2 CPU, 4GB RAM recommended)
2. **Use local registry** to reduce image pull time
3. **Pre-pull images** on nodes for faster startup
4. **Enable cargo build cache** persistence

## License

MIT OR Apache-2.0

## Contributing

See main repository [CONTRIBUTING.md](../../CONTRIBUTING.md)

## Support

File issues at: https://github.com/raibid-labs/raibid-ci/issues
